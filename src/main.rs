use std::time::Instant;

use openssl::{ecdsa::EcdsaSig, ec::{EcKey, EcGroup}, nid::Nid};

fn main() {
    let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let key = EcKey::generate(&group).unwrap();

    let now = Instant::now();

    for _ in 0..1000 {
        let _ = EcdsaSig::sign(&[1, 2, 3, 4], &key.as_ref());
    }

    println!("sign {}", now.elapsed().as_secs_f64() / 1000f64);

    let signature = EcdsaSig::sign(&[1, 2, 3, 4], &key.as_ref()).unwrap();

    let now = Instant::now();

    for _ in 0..1000 {
        let _ = signature.verify(&[1, 2, 3, 4], &key);
    }

    println!("verify {}", now.elapsed().as_secs_f64() / 1000f64);
}
