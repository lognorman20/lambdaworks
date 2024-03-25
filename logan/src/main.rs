use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve,
        traits::IsEllipticCurve,
    },
};

fn main() {
    // To compute the public key, follow the equation `K = k * g` where `k` is
    // the private key as an integer
    let g = BLS12381Curve::generator();
    let int_private_key = u64::from_str_radix("6C616D6264617370", 16).unwrap();

    // Add the generator point to itself `int_private_key` times (multiplication == repetitive addition)
    let public_key = g.operate_with_self(int_private_key);

    // Check the public key is a valid subgroup
    assert!(public_key.is_in_subgroup());
    println!("Successfully generated a public key...");
}
