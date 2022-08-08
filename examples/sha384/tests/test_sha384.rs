use hacspec_lib::*;
use hacspec_sha384::*;

#[test]
fn test_sha384() {
    let msg = ByteSeq::from_public_slice(b"hello world");
    let expected = "fdbd8e75a67f29f701a4e040385e2e23986303ea10239211af907fcbb83578b3e417cb71ce646efd0819dd8c088de1bd";
    let result = sha384(&msg);
    assert_eq!(expected, result.to_hex());
}

#[test]
fn test_empty() {
    let msg = ByteSeq::from_public_slice(b"");
    let expected = "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b";
    let result = sha384(&msg);
    assert_eq!(expected, result.to_hex());
}
