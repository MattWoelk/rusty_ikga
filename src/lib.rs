#![allow(dead_code)]
type ScaledBasisBlade = u32;

struct Scalar {
    magnitude: f64,
}

// TODO: have two types of vectors
// one that stores x and y
// and another that stores magnitude and direction
// a function will return whichever is convenient
// and they would both implement the same trait
// so either can be used in any function
// The purpose of this would be to increase efficiency
// by only requiring a conversion between the two
// when absolutely necessary
// and would be optimized by the compiler
// because it would use the type system.
struct Vector {
    x: Scalar,
    y: Scalar,
}

// TODO: store one scalar for each of:
// e_12 e_23 e_31
// ALTERNATIVELY:
// ??? a magnitude and two angles ???
// ??? a Vector and a magnitude ???
struct Bivector {
    magnitude: Scalar,
    vector: Vector,
}

struct MultiVector {
    scalar: Scalar,
    vector: Vector,
    bivector: Bivector,
}

//type Vector = [f64];
// maybe Vector should be a trait that tuples of different lengths can have?

//fn add2(a:(f32,f32), b:(f32,f32)) -> (f32,f32) {
//    (a.0 + b.0, a.1 + b.1)
//}
//
//fn add_vector(a:&[f32], b:&[f32]) -> [f32; 3] {
//    println!("{:?}", a);
//    for (aye, bee) in a.iter().zip(b.iter()).collect {
//    }
//}

#[test]
#[allow(unused_variables)]
fn it_works() {
    // blades
    let scalar : ScaledBasisBlade = 0b0;   // a scalar
    let e1 : ScaledBasisBlade = 0b1;   // a vector e1
    let e2 : ScaledBasisBlade = 0b10;  // a vector e2
    let e1_wedge_e2 : ScaledBasisBlade = 0b11;  // a bivector

    assert_eq!(e1_wedge_e2, e1 ^ e2);  // outer "wedge" product


    //let scalar = [];
    let e1 = [1];
    let e2 = [1, 0];
    //let multivector = [];  // causes complier error
}
