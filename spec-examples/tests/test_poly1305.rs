use hacspec::prelude::*;

use hacspec_examples::chacha20_poly1305::chacha20::*;
use hacspec_examples::chacha20_poly1305::poly1305::*;

#[test]
fn basic_test() {
    let key = Key::random();
    let iv = IV::random();
    let m = ByteSeq::random(40);
    poly_mac(&m, key, iv);

    // RFC 7539 Test Vectors
    let msg = ByteSeq::from_public_slice(&[
        0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x20, 0x46,
        0x6f, 0x72, 0x75, 0x6d, 0x20, 0x52, 0x65, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20, 0x47,
        0x72, 0x6f, 0x75, 0x70
    ]);
    let k = Key::from_public_slice(&[
        0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5, 0x06,
        0xa8, 0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd, 0x4a, 0xbf, 0xf6, 0xaf, 0x41, 0x49,
        0xf5, 0x1b
    ]);
    let expected = Tag::from_native_slice(&[
        0xa8, 0x06, 0x1d, 0xc1, 0x30, 0x51, 0x36, 0xc6, 0xc2, 0x2b, 0x8b, 0xaf, 0x0c, 0x01, 0x27,
        0xa9
    ]);
    let computed = poly(&msg, k);
    assert_eq!(expected, computed)
}
