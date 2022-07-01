// g & h            => Binary AND
// g | h            => Binary OR
// g ^ h            => BInary XOR
// g ~ h            => Binary one's complement
// g << h           => Binary shift left
// g >> h           => Binary shift right

let (g, h) = (0x1, 0x2);

let bitwise_add = g & h; // => 0
let bitwise_or = g | h;  // => 3
let bitwise_xor = g ^ h; // => 3
let right_shift = g >> 2; // => 0
let left_shift = h << 4;  // => 32