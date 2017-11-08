use super::{Decoder, Scalar, encoded_shape, EncodedShape, SliceDecodeSink, DecodeQuadSink, decode_num_scalar, cumulative_encoded_len};

/// Offers more flexible decoding than the top-level `decode()`.
///
/// You can skip numbers you don't need with `skip()`, and decode the parts of your input you need
/// with `decode_slice()`.
///
/// If you need maximum flexibility, you can use `decode_sink()` with a custom `DecodeQuadSink`
/// implementation to receive numbers as they are decoded rather than storing them into a slice
/// and then inspecting them.
///
/// # Decode sinks
///
/// If you don't want to write decoded numbers into a slice and inspect them later, you can use a
/// custom sink. This is probably most useful when you want to minimize memory usage. For instance,
/// you could `mmap` a file and scan through its contents with a custom sink without ever allocating
/// on the heap.
///
/// There are two traits to represent a sink: one for complete quads
/// (`DecodeQuadSink`), and one for any trailing leftover numbers that may not fill a quad
/// (`DecodeSingleSink`). You will need to implement both with the appropriate
/// `Decoder::DecodedQuad` type for the `Decoder` you are using.
///
/// # Examples
/// 
/// Here's a sink that calculates the maximum number in the input without writing the decoded input
/// anywhere. An implementation of `DecodeQuadSink` is included for `x86::Ssse3` to show how to work
/// with `m128i`.
/// 
/// ```
/// extern crate rand;
/// extern crate stream_vbyte;
/// #[cfg(feature = "x86_ssse3")]
/// extern crate x86intrin;
///
/// use std::cmp;
///
/// use rand::Rng;
///
/// use stream_vbyte::DecodeSingleSink;
///
/// struct MaxSink {
///     max: u32
/// }
/// 
/// impl MaxSink {
///     fn new() -> MaxSink {
///         MaxSink {
///             max: 0
///         }
///     }
/// }
/// 
/// impl stream_vbyte::DecodeSingleSink for MaxSink {
///     fn on_number(&mut self, num: u32, _nums_decoded: usize) {
///         self.max = cmp::max(self.max, num)
///     }
/// }
/// 
/// impl stream_vbyte::DecodeQuadSink<()> for MaxSink {
///     fn on_quad(&mut self, _quad: (), _nums_decoded: usize) {
///         // on_quad not used if type is ()
///         panic!("Should never be called")
///     }
/// }
/// 
/// #[cfg(feature = "x86_ssse3")]
/// impl stream_vbyte::DecodeQuadSink<x86intrin::m128i> for MaxSink {
///     fn on_quad(&mut self, quad: x86intrin::m128i, _nums_decoded: usize) {
///         let u32s = quad.as_u32x4();
/// 
///         self.on_number(
///             cmp::max(
///                 cmp::max(u32s.extract(0), u32s.extract(1)),
///                 cmp::max(u32s.extract(2), u32s.extract(3))),
///             0)
///     }
/// }
///
/// #[cfg(not(feature = "x86_ssse3"))]
/// fn use_x86_simd_if_available(mut cursor: stream_vbyte::DecodeCursor, len: usize, sink: &mut MaxSink) {
///     cursor.decode_sink::<stream_vbyte::Scalar, _>(sink, len);
/// }
///
/// #[cfg(feature = "x86_ssse3")]
/// fn use_x86_simd_if_available(mut cursor: stream_vbyte::DecodeCursor, len: usize, sink: &mut MaxSink) {
///     cursor.decode_sink::<stream_vbyte::x86::Ssse3, _>(sink, len);
/// }
///
/// fn main() {
///     let mut nums = vec![1, 2, 3, 5, 8, 13, 21, 34];
///
///     // shuffle the numbers just so there's clearly nothing up our sleeve
///     let mut rng = rand::thread_rng();
///     rng.shuffle(&mut nums[..]);
///
///     let mut encoded = vec![0; nums.len() * 5];
///     stream_vbyte::encode::<stream_vbyte::Scalar>(&nums, &mut encoded);
///
///     let mut cursor = stream_vbyte::DecodeCursor::new(&encoded, nums.len());
///     let mut sink = MaxSink::new();
///     use_x86_simd_if_available(cursor, nums.len(), &mut sink);
///
///     assert_eq!(34, sink.max);
/// }
/// 
/// ```
///
#[derive(Debug)]
pub struct DecodeCursor<'a> {
    control_bytes: &'a [u8],
    encoded_nums: &'a [u8],
    encoded_shape: EncodedShape,
    total_nums: usize,
    nums_decoded: usize,
    control_bytes_read: usize,
    encoded_bytes_read: usize
}

impl<'a> DecodeCursor<'a> {
    /// Create a new cursor.
    pub fn new(input: &'a [u8], count: usize) -> DecodeCursor<'a> {
        let shape = encoded_shape(count);

        DecodeCursor {
            control_bytes: &input[0..shape.control_bytes_len],
            encoded_nums: &input[shape.control_bytes_len..],
            encoded_shape: shape,
            total_nums: count,
            nums_decoded: 0,
            control_bytes_read: 0,
            encoded_bytes_read: 0
        }
    }

    /// Skip `to_skip` numbers. `to_skip` must be a multiple of 4, and must not be greater than the
    /// count of remaining numbers that are in complete blocks of 4. In other words, if you have
    /// 7 numbers remaining (a block of 4 and a partial block of 3), the only count you can skip is
    /// 4.
    ///
    /// Skipping numbers is faster than decoding them.
    pub fn skip(&mut self, to_skip: usize) {
        assert_eq!(to_skip % 4, 0, "Must be a multiple of 4");
        let control_bytes_to_skip = to_skip / 4;
        assert!(self.control_bytes_read + control_bytes_to_skip
                        <= self.encoded_shape.complete_control_bytes_len,
                "Can't skip past the end of complete control bytes");

        let slice_to_skip = &self.control_bytes[self.control_bytes_read..(self.control_bytes_read + control_bytes_to_skip)];
        let skipped_encoded_len = cumulative_encoded_len(&slice_to_skip);

        self.control_bytes_read += control_bytes_to_skip;
        self.encoded_bytes_read += skipped_encoded_len;
        self.nums_decoded += to_skip;
    }

    /// Decode into the `output` buffer.
    ///
    /// If there is at least one complete quad of input remaining to decode, the buffer must be
    /// at least of size 4.
    ///
    /// If there is only a final partial quad of input, the buffer must be at least as big as the
    /// remaining input.
    ///
    /// Returns the number of numbers decoded by this invocation, which may be less than the size
    /// of the buffer.
    pub fn decode_slice<D: Decoder>(&mut self, output: &mut [u32]) -> usize
        where for <'b> SliceDecodeSink<'b>: DecodeQuadSink<D::DecodedQuad> {
        let output_len = output.len();

        let mut sink = SliceDecodeSink::new(output);

        self.decode_sink::<D, SliceDecodeSink>(&mut sink, output_len)
    }

    /// Decode at most `max_numbers_to_decode` numbers from the input and hand them to `sink`.
    ///
    /// Decoding is done one quad at a time, except for the last quad, which may have fewer than
    /// four corresponding encoded numbers. Consequently, the number of numbers decoded will be a
    /// multiple of 4, unless `max_numbers_to_decode` includes the end of the encoded input, in
    /// which case the number of numbers will be all remaining numbers in the input regardless of
    /// whether that's a multiple of 4 or not.
    ///
    /// With each invocation of `decode()`, the `nums_decoded` parameter used in
    /// `DecodeQuadSink.on_quad()` will start counting up from 0 again.
    ///
    /// Returns the number of numbers decoded.
    pub fn decode_sink<D, S>(&mut self, sink: &mut S, max_numbers_to_decode: usize) -> usize
        where D: Decoder, S: DecodeQuadSink<D::DecodedQuad> + DecodeQuadSink<<Scalar as Decoder>::DecodedQuad> {
        let start_nums_decoded = self.nums_decoded;
        let mut complete_quad_nums_decoded_this_invocation;

        let complete_control_bytes_to_decode = max_numbers_to_decode / 4;

        {
            // decode complete quads
            let (primary_nums_decoded, primary_bytes_read) =
                D::decode_quads(&self.control_bytes[self.control_bytes_read..self.encoded_shape.complete_control_bytes_len],
                                &self.encoded_nums[self.encoded_bytes_read..],
                                complete_control_bytes_to_decode,
                                0,
                                sink);

            complete_quad_nums_decoded_this_invocation = primary_nums_decoded;
            self.nums_decoded += primary_nums_decoded;
            self.encoded_bytes_read += primary_bytes_read;
            self.control_bytes_read += complete_quad_nums_decoded_this_invocation / 4;
        }

        {
            // handle any remaining full quads if the provided Decoder did not consume all the control
            // bytes
            let (more_nums_decoded, more_bytes_read) = Scalar::decode_quads(
                &self.control_bytes[self.control_bytes_read..self.encoded_shape.complete_control_bytes_len],
                &self.encoded_nums[self.encoded_bytes_read..],
                complete_control_bytes_to_decode - complete_quad_nums_decoded_this_invocation / 4,
                complete_quad_nums_decoded_this_invocation,
                sink);

            complete_quad_nums_decoded_this_invocation += more_nums_decoded;
            self.encoded_bytes_read += more_bytes_read;
            self.control_bytes_read += more_nums_decoded / 4;
            self.nums_decoded += more_nums_decoded;
        }

        // decode incomplete quad if we're at the end and we were asked to decode all leftovers
        if max_numbers_to_decode - complete_quad_nums_decoded_this_invocation >= self.encoded_shape.leftover_numbers
            && self.control_bytes_read == self.encoded_shape.complete_control_bytes_len
            && self.encoded_shape.leftover_numbers > 0
            && self.nums_decoded < self.total_nums {

            let control_byte = self.control_bytes[self.encoded_shape.complete_control_bytes_len];

            for i in 0..self.encoded_shape.leftover_numbers {
                // first num's length in low 2 bits, last in high 2 bits
                let bitmask = 0x03 << (i * 2);
                let len = ((control_byte & bitmask) >> (i * 2)) as usize + 1;
                sink.on_number(decode_num_scalar(len, &self.encoded_nums[self.encoded_bytes_read..]),
                    complete_quad_nums_decoded_this_invocation + i);
                self.nums_decoded += 1;
                self.encoded_bytes_read += len;
            }
        }

        self.nums_decoded - start_nums_decoded
    }

    /// Returns the total length of input scanned so far: the complete block of control bytes, plus
    /// any encoded numbers decoded.
    pub fn input_consumed(&self) -> usize {
        self.encoded_shape.control_bytes_len + self.encoded_bytes_read
    }

    /// Returns true iff there are more numbers to be decoded.
    pub fn has_more(&self) -> bool {
        self.nums_decoded < self.total_nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    #[should_panic(expected = "Must be a multiple of 4")]
    fn skip_panics_on_not_multiple_of_4() {
        DecodeCursor::new(&vec![], 0).skip(3)
    }

    #[test]
    #[should_panic(expected = "Can't skip past the end of complete control bytes")]
    fn skip_panics_on_exceeding_full_quads() {
        let nums: Vec<u32> = (0..100).collect();
        let mut encoded = Vec::new();
        encoded.resize(nums.len() * 5, 0);

        let encoded_len = encode::<Scalar>(&nums, &mut encoded);

        DecodeCursor::new(&encoded[0..encoded_len], nums.len()).skip(104);
    }

    #[test]
    fn skip_entire_enput_is_done() {
        let nums: Vec<u32> = (0..100).collect();
        let mut encoded = Vec::new();
        encoded.resize(nums.len() * 5, 0);

        let encoded_len = encode::<Scalar>(&nums, &mut encoded);
        let mut cursor = DecodeCursor::new(&encoded[0..encoded_len], nums.len());

        assert!(cursor.has_more());

        cursor.skip(100);

        assert!(!cursor.has_more());

        let mut decoded: Vec<u32> = (0..100).map(|_| 0).collect();
        // decoded has room...
        assert_eq!(100, decoded.len());
        // but nothing gets decoded into it
        assert_eq!(0, cursor.decode_slice::<Scalar>(&mut decoded[..]))
    }
}
