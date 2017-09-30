pub const DECODE_TABLE: &'static [(u8, u8, u8, u8)] = &[
    (1, 1, 1, 1), // 0 = 0x0 = 0b00000000
    (1, 1, 1, 2), // 1 = 0x1 = 0b00000001
    (1, 1, 1, 3), // 2 = 0x2 = 0b00000010
    (1, 1, 1, 4), // 3 = 0x3 = 0b00000011
    (1, 1, 2, 1), // 4 = 0x4 = 0b00000100
    (1, 1, 2, 2), // 5 = 0x5 = 0b00000101
    (1, 1, 2, 3), // 6 = 0x6 = 0b00000110
    (1, 1, 2, 4), // 7 = 0x7 = 0b00000111
    (1, 1, 3, 1), // 8 = 0x8 = 0b00001000
    (1, 1, 3, 2), // 9 = 0x9 = 0b00001001
    (1, 1, 3, 3), // 10 = 0xA = 0b00001010
    (1, 1, 3, 4), // 11 = 0xB = 0b00001011
    (1, 1, 4, 1), // 12 = 0xC = 0b00001100
    (1, 1, 4, 2), // 13 = 0xD = 0b00001101
    (1, 1, 4, 3), // 14 = 0xE = 0b00001110
    (1, 1, 4, 4), // 15 = 0xF = 0b00001111
    (1, 2, 1, 1), // 16 = 0x10 = 0b00010000
    (1, 2, 1, 2), // 17 = 0x11 = 0b00010001
    (1, 2, 1, 3), // 18 = 0x12 = 0b00010010
    (1, 2, 1, 4), // 19 = 0x13 = 0b00010011
    (1, 2, 2, 1), // 20 = 0x14 = 0b00010100
    (1, 2, 2, 2), // 21 = 0x15 = 0b00010101
    (1, 2, 2, 3), // 22 = 0x16 = 0b00010110
    (1, 2, 2, 4), // 23 = 0x17 = 0b00010111
    (1, 2, 3, 1), // 24 = 0x18 = 0b00011000
    (1, 2, 3, 2), // 25 = 0x19 = 0b00011001
    (1, 2, 3, 3), // 26 = 0x1A = 0b00011010
    (1, 2, 3, 4), // 27 = 0x1B = 0b00011011
    (1, 2, 4, 1), // 28 = 0x1C = 0b00011100
    (1, 2, 4, 2), // 29 = 0x1D = 0b00011101
    (1, 2, 4, 3), // 30 = 0x1E = 0b00011110
    (1, 2, 4, 4), // 31 = 0x1F = 0b00011111
    (1, 3, 1, 1), // 32 = 0x20 = 0b00100000
    (1, 3, 1, 2), // 33 = 0x21 = 0b00100001
    (1, 3, 1, 3), // 34 = 0x22 = 0b00100010
    (1, 3, 1, 4), // 35 = 0x23 = 0b00100011
    (1, 3, 2, 1), // 36 = 0x24 = 0b00100100
    (1, 3, 2, 2), // 37 = 0x25 = 0b00100101
    (1, 3, 2, 3), // 38 = 0x26 = 0b00100110
    (1, 3, 2, 4), // 39 = 0x27 = 0b00100111
    (1, 3, 3, 1), // 40 = 0x28 = 0b00101000
    (1, 3, 3, 2), // 41 = 0x29 = 0b00101001
    (1, 3, 3, 3), // 42 = 0x2A = 0b00101010
    (1, 3, 3, 4), // 43 = 0x2B = 0b00101011
    (1, 3, 4, 1), // 44 = 0x2C = 0b00101100
    (1, 3, 4, 2), // 45 = 0x2D = 0b00101101
    (1, 3, 4, 3), // 46 = 0x2E = 0b00101110
    (1, 3, 4, 4), // 47 = 0x2F = 0b00101111
    (1, 4, 1, 1), // 48 = 0x30 = 0b00110000
    (1, 4, 1, 2), // 49 = 0x31 = 0b00110001
    (1, 4, 1, 3), // 50 = 0x32 = 0b00110010
    (1, 4, 1, 4), // 51 = 0x33 = 0b00110011
    (1, 4, 2, 1), // 52 = 0x34 = 0b00110100
    (1, 4, 2, 2), // 53 = 0x35 = 0b00110101
    (1, 4, 2, 3), // 54 = 0x36 = 0b00110110
    (1, 4, 2, 4), // 55 = 0x37 = 0b00110111
    (1, 4, 3, 1), // 56 = 0x38 = 0b00111000
    (1, 4, 3, 2), // 57 = 0x39 = 0b00111001
    (1, 4, 3, 3), // 58 = 0x3A = 0b00111010
    (1, 4, 3, 4), // 59 = 0x3B = 0b00111011
    (1, 4, 4, 1), // 60 = 0x3C = 0b00111100
    (1, 4, 4, 2), // 61 = 0x3D = 0b00111101
    (1, 4, 4, 3), // 62 = 0x3E = 0b00111110
    (1, 4, 4, 4), // 63 = 0x3F = 0b00111111
    (2, 1, 1, 1), // 64 = 0x40 = 0b01000000
    (2, 1, 1, 2), // 65 = 0x41 = 0b01000001
    (2, 1, 1, 3), // 66 = 0x42 = 0b01000010
    (2, 1, 1, 4), // 67 = 0x43 = 0b01000011
    (2, 1, 2, 1), // 68 = 0x44 = 0b01000100
    (2, 1, 2, 2), // 69 = 0x45 = 0b01000101
    (2, 1, 2, 3), // 70 = 0x46 = 0b01000110
    (2, 1, 2, 4), // 71 = 0x47 = 0b01000111
    (2, 1, 3, 1), // 72 = 0x48 = 0b01001000
    (2, 1, 3, 2), // 73 = 0x49 = 0b01001001
    (2, 1, 3, 3), // 74 = 0x4A = 0b01001010
    (2, 1, 3, 4), // 75 = 0x4B = 0b01001011
    (2, 1, 4, 1), // 76 = 0x4C = 0b01001100
    (2, 1, 4, 2), // 77 = 0x4D = 0b01001101
    (2, 1, 4, 3), // 78 = 0x4E = 0b01001110
    (2, 1, 4, 4), // 79 = 0x4F = 0b01001111
    (2, 2, 1, 1), // 80 = 0x50 = 0b01010000
    (2, 2, 1, 2), // 81 = 0x51 = 0b01010001
    (2, 2, 1, 3), // 82 = 0x52 = 0b01010010
    (2, 2, 1, 4), // 83 = 0x53 = 0b01010011
    (2, 2, 2, 1), // 84 = 0x54 = 0b01010100
    (2, 2, 2, 2), // 85 = 0x55 = 0b01010101
    (2, 2, 2, 3), // 86 = 0x56 = 0b01010110
    (2, 2, 2, 4), // 87 = 0x57 = 0b01010111
    (2, 2, 3, 1), // 88 = 0x58 = 0b01011000
    (2, 2, 3, 2), // 89 = 0x59 = 0b01011001
    (2, 2, 3, 3), // 90 = 0x5A = 0b01011010
    (2, 2, 3, 4), // 91 = 0x5B = 0b01011011
    (2, 2, 4, 1), // 92 = 0x5C = 0b01011100
    (2, 2, 4, 2), // 93 = 0x5D = 0b01011101
    (2, 2, 4, 3), // 94 = 0x5E = 0b01011110
    (2, 2, 4, 4), // 95 = 0x5F = 0b01011111
    (2, 3, 1, 1), // 96 = 0x60 = 0b01100000
    (2, 3, 1, 2), // 97 = 0x61 = 0b01100001
    (2, 3, 1, 3), // 98 = 0x62 = 0b01100010
    (2, 3, 1, 4), // 99 = 0x63 = 0b01100011
    (2, 3, 2, 1), // 100 = 0x64 = 0b01100100
    (2, 3, 2, 2), // 101 = 0x65 = 0b01100101
    (2, 3, 2, 3), // 102 = 0x66 = 0b01100110
    (2, 3, 2, 4), // 103 = 0x67 = 0b01100111
    (2, 3, 3, 1), // 104 = 0x68 = 0b01101000
    (2, 3, 3, 2), // 105 = 0x69 = 0b01101001
    (2, 3, 3, 3), // 106 = 0x6A = 0b01101010
    (2, 3, 3, 4), // 107 = 0x6B = 0b01101011
    (2, 3, 4, 1), // 108 = 0x6C = 0b01101100
    (2, 3, 4, 2), // 109 = 0x6D = 0b01101101
    (2, 3, 4, 3), // 110 = 0x6E = 0b01101110
    (2, 3, 4, 4), // 111 = 0x6F = 0b01101111
    (2, 4, 1, 1), // 112 = 0x70 = 0b01110000
    (2, 4, 1, 2), // 113 = 0x71 = 0b01110001
    (2, 4, 1, 3), // 114 = 0x72 = 0b01110010
    (2, 4, 1, 4), // 115 = 0x73 = 0b01110011
    (2, 4, 2, 1), // 116 = 0x74 = 0b01110100
    (2, 4, 2, 2), // 117 = 0x75 = 0b01110101
    (2, 4, 2, 3), // 118 = 0x76 = 0b01110110
    (2, 4, 2, 4), // 119 = 0x77 = 0b01110111
    (2, 4, 3, 1), // 120 = 0x78 = 0b01111000
    (2, 4, 3, 2), // 121 = 0x79 = 0b01111001
    (2, 4, 3, 3), // 122 = 0x7A = 0b01111010
    (2, 4, 3, 4), // 123 = 0x7B = 0b01111011
    (2, 4, 4, 1), // 124 = 0x7C = 0b01111100
    (2, 4, 4, 2), // 125 = 0x7D = 0b01111101
    (2, 4, 4, 3), // 126 = 0x7E = 0b01111110
    (2, 4, 4, 4), // 127 = 0x7F = 0b01111111
    (3, 1, 1, 1), // 128 = 0x80 = 0b10000000
    (3, 1, 1, 2), // 129 = 0x81 = 0b10000001
    (3, 1, 1, 3), // 130 = 0x82 = 0b10000010
    (3, 1, 1, 4), // 131 = 0x83 = 0b10000011
    (3, 1, 2, 1), // 132 = 0x84 = 0b10000100
    (3, 1, 2, 2), // 133 = 0x85 = 0b10000101
    (3, 1, 2, 3), // 134 = 0x86 = 0b10000110
    (3, 1, 2, 4), // 135 = 0x87 = 0b10000111
    (3, 1, 3, 1), // 136 = 0x88 = 0b10001000
    (3, 1, 3, 2), // 137 = 0x89 = 0b10001001
    (3, 1, 3, 3), // 138 = 0x8A = 0b10001010
    (3, 1, 3, 4), // 139 = 0x8B = 0b10001011
    (3, 1, 4, 1), // 140 = 0x8C = 0b10001100
    (3, 1, 4, 2), // 141 = 0x8D = 0b10001101
    (3, 1, 4, 3), // 142 = 0x8E = 0b10001110
    (3, 1, 4, 4), // 143 = 0x8F = 0b10001111
    (3, 2, 1, 1), // 144 = 0x90 = 0b10010000
    (3, 2, 1, 2), // 145 = 0x91 = 0b10010001
    (3, 2, 1, 3), // 146 = 0x92 = 0b10010010
    (3, 2, 1, 4), // 147 = 0x93 = 0b10010011
    (3, 2, 2, 1), // 148 = 0x94 = 0b10010100
    (3, 2, 2, 2), // 149 = 0x95 = 0b10010101
    (3, 2, 2, 3), // 150 = 0x96 = 0b10010110
    (3, 2, 2, 4), // 151 = 0x97 = 0b10010111
    (3, 2, 3, 1), // 152 = 0x98 = 0b10011000
    (3, 2, 3, 2), // 153 = 0x99 = 0b10011001
    (3, 2, 3, 3), // 154 = 0x9A = 0b10011010
    (3, 2, 3, 4), // 155 = 0x9B = 0b10011011
    (3, 2, 4, 1), // 156 = 0x9C = 0b10011100
    (3, 2, 4, 2), // 157 = 0x9D = 0b10011101
    (3, 2, 4, 3), // 158 = 0x9E = 0b10011110
    (3, 2, 4, 4), // 159 = 0x9F = 0b10011111
    (3, 3, 1, 1), // 160 = 0xA0 = 0b10100000
    (3, 3, 1, 2), // 161 = 0xA1 = 0b10100001
    (3, 3, 1, 3), // 162 = 0xA2 = 0b10100010
    (3, 3, 1, 4), // 163 = 0xA3 = 0b10100011
    (3, 3, 2, 1), // 164 = 0xA4 = 0b10100100
    (3, 3, 2, 2), // 165 = 0xA5 = 0b10100101
    (3, 3, 2, 3), // 166 = 0xA6 = 0b10100110
    (3, 3, 2, 4), // 167 = 0xA7 = 0b10100111
    (3, 3, 3, 1), // 168 = 0xA8 = 0b10101000
    (3, 3, 3, 2), // 169 = 0xA9 = 0b10101001
    (3, 3, 3, 3), // 170 = 0xAA = 0b10101010
    (3, 3, 3, 4), // 171 = 0xAB = 0b10101011
    (3, 3, 4, 1), // 172 = 0xAC = 0b10101100
    (3, 3, 4, 2), // 173 = 0xAD = 0b10101101
    (3, 3, 4, 3), // 174 = 0xAE = 0b10101110
    (3, 3, 4, 4), // 175 = 0xAF = 0b10101111
    (3, 4, 1, 1), // 176 = 0xB0 = 0b10110000
    (3, 4, 1, 2), // 177 = 0xB1 = 0b10110001
    (3, 4, 1, 3), // 178 = 0xB2 = 0b10110010
    (3, 4, 1, 4), // 179 = 0xB3 = 0b10110011
    (3, 4, 2, 1), // 180 = 0xB4 = 0b10110100
    (3, 4, 2, 2), // 181 = 0xB5 = 0b10110101
    (3, 4, 2, 3), // 182 = 0xB6 = 0b10110110
    (3, 4, 2, 4), // 183 = 0xB7 = 0b10110111
    (3, 4, 3, 1), // 184 = 0xB8 = 0b10111000
    (3, 4, 3, 2), // 185 = 0xB9 = 0b10111001
    (3, 4, 3, 3), // 186 = 0xBA = 0b10111010
    (3, 4, 3, 4), // 187 = 0xBB = 0b10111011
    (3, 4, 4, 1), // 188 = 0xBC = 0b10111100
    (3, 4, 4, 2), // 189 = 0xBD = 0b10111101
    (3, 4, 4, 3), // 190 = 0xBE = 0b10111110
    (3, 4, 4, 4), // 191 = 0xBF = 0b10111111
    (4, 1, 1, 1), // 192 = 0xC0 = 0b11000000
    (4, 1, 1, 2), // 193 = 0xC1 = 0b11000001
    (4, 1, 1, 3), // 194 = 0xC2 = 0b11000010
    (4, 1, 1, 4), // 195 = 0xC3 = 0b11000011
    (4, 1, 2, 1), // 196 = 0xC4 = 0b11000100
    (4, 1, 2, 2), // 197 = 0xC5 = 0b11000101
    (4, 1, 2, 3), // 198 = 0xC6 = 0b11000110
    (4, 1, 2, 4), // 199 = 0xC7 = 0b11000111
    (4, 1, 3, 1), // 200 = 0xC8 = 0b11001000
    (4, 1, 3, 2), // 201 = 0xC9 = 0b11001001
    (4, 1, 3, 3), // 202 = 0xCA = 0b11001010
    (4, 1, 3, 4), // 203 = 0xCB = 0b11001011
    (4, 1, 4, 1), // 204 = 0xCC = 0b11001100
    (4, 1, 4, 2), // 205 = 0xCD = 0b11001101
    (4, 1, 4, 3), // 206 = 0xCE = 0b11001110
    (4, 1, 4, 4), // 207 = 0xCF = 0b11001111
    (4, 2, 1, 1), // 208 = 0xD0 = 0b11010000
    (4, 2, 1, 2), // 209 = 0xD1 = 0b11010001
    (4, 2, 1, 3), // 210 = 0xD2 = 0b11010010
    (4, 2, 1, 4), // 211 = 0xD3 = 0b11010011
    (4, 2, 2, 1), // 212 = 0xD4 = 0b11010100
    (4, 2, 2, 2), // 213 = 0xD5 = 0b11010101
    (4, 2, 2, 3), // 214 = 0xD6 = 0b11010110
    (4, 2, 2, 4), // 215 = 0xD7 = 0b11010111
    (4, 2, 3, 1), // 216 = 0xD8 = 0b11011000
    (4, 2, 3, 2), // 217 = 0xD9 = 0b11011001
    (4, 2, 3, 3), // 218 = 0xDA = 0b11011010
    (4, 2, 3, 4), // 219 = 0xDB = 0b11011011
    (4, 2, 4, 1), // 220 = 0xDC = 0b11011100
    (4, 2, 4, 2), // 221 = 0xDD = 0b11011101
    (4, 2, 4, 3), // 222 = 0xDE = 0b11011110
    (4, 2, 4, 4), // 223 = 0xDF = 0b11011111
    (4, 3, 1, 1), // 224 = 0xE0 = 0b11100000
    (4, 3, 1, 2), // 225 = 0xE1 = 0b11100001
    (4, 3, 1, 3), // 226 = 0xE2 = 0b11100010
    (4, 3, 1, 4), // 227 = 0xE3 = 0b11100011
    (4, 3, 2, 1), // 228 = 0xE4 = 0b11100100
    (4, 3, 2, 2), // 229 = 0xE5 = 0b11100101
    (4, 3, 2, 3), // 230 = 0xE6 = 0b11100110
    (4, 3, 2, 4), // 231 = 0xE7 = 0b11100111
    (4, 3, 3, 1), // 232 = 0xE8 = 0b11101000
    (4, 3, 3, 2), // 233 = 0xE9 = 0b11101001
    (4, 3, 3, 3), // 234 = 0xEA = 0b11101010
    (4, 3, 3, 4), // 235 = 0xEB = 0b11101011
    (4, 3, 4, 1), // 236 = 0xEC = 0b11101100
    (4, 3, 4, 2), // 237 = 0xED = 0b11101101
    (4, 3, 4, 3), // 238 = 0xEE = 0b11101110
    (4, 3, 4, 4), // 239 = 0xEF = 0b11101111
    (4, 4, 1, 1), // 240 = 0xF0 = 0b11110000
    (4, 4, 1, 2), // 241 = 0xF1 = 0b11110001
    (4, 4, 1, 3), // 242 = 0xF2 = 0b11110010
    (4, 4, 1, 4), // 243 = 0xF3 = 0b11110011
    (4, 4, 2, 1), // 244 = 0xF4 = 0b11110100
    (4, 4, 2, 2), // 245 = 0xF5 = 0b11110101
    (4, 4, 2, 3), // 246 = 0xF6 = 0b11110110
    (4, 4, 2, 4), // 247 = 0xF7 = 0b11110111
    (4, 4, 3, 1), // 248 = 0xF8 = 0b11111000
    (4, 4, 3, 2), // 249 = 0xF9 = 0b11111001
    (4, 4, 3, 3), // 250 = 0xFA = 0b11111010
    (4, 4, 3, 4), // 251 = 0xFB = 0b11111011
    (4, 4, 4, 1), // 252 = 0xFC = 0b11111100
    (4, 4, 4, 2), // 253 = 0xFD = 0b11111101
    (4, 4, 4, 3), // 254 = 0xFE = 0b11111110
    (4, 4, 4, 4), // 255 = 0xFF = 0b11111111
];
