type ScaledBasisBlade = u32;

#[test]
#[allow(unused_variables)]
fn it_works() {
    // blades
    let scalar : ScaledBasisBlade = 0b0;   // a scalar
    let e1 : ScaledBasisBlade = 0b1;   // a vector e1
    let e2 : ScaledBasisBlade = 0b10;  // a vector e2
    let e1_wedge_e2 : ScaledBasisBlade = 0b11;  // a bivector

    let e1_wedge_e2 = e1 ^ e2;
    println!("{}", e1);
    println!("{}", e2);
    println!("{}", e1^e2);  // outer "wedge" product
}
