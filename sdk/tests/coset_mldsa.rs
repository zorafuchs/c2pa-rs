use coset::{cbor::value::Value, CborSerializable, CoseSign1};

#[test]
fn coset_accepts_ml_dsa_65_alg() {
    // Protected header: { 1 (alg): -49 (ML-DSA-65) }, encoded as a bstr
    let mut protected = Vec::new();
    coset::cbor::ser::into_writer(
        &Value::Map(vec![(Value::Integer(1.into()), Value::Integer((-49).into()))]),
        &mut protected,
    )
    .unwrap();

    // COSE_Sign1 = [protected, unprotected, payload, signature]
    let sign1 = Value::Array(vec![
        Value::Bytes(protected),
        Value::Map(vec![]),
        Value::Bytes(b"payload".to_vec()),
        Value::Bytes(vec![0u8; 16]), // dummy signature; only parsing is tested
    ]);
    let mut bytes = Vec::new();
    coset::cbor::ser::into_writer(&sign1, &mut bytes).unwrap();

    match CoseSign1::from_slice(&bytes) {
        Ok(s) => println!("parsed OK, alg = {:?}", s.protected.header.alg),
        Err(e) => panic!("coset rejects alg -49: {e:?}"),
    }
}