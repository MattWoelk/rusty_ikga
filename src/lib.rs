#![allow(dead_code)]

use std::ops::Add;

type ScaledBasisBlade = u32;

// TODO: abstract these all into k-blades somehow.
// OR just give them a trait called k-blade that lets them be used together easily.

#[derive(Debug, PartialEq, PartialOrd)]
struct Scalar(f64);

impl Scalar {
    pub fn new(m:f64) -> Scalar {
        Scalar(m)
    }
}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, _rhs: Scalar) -> Scalar {
        return Scalar(self.0 + _rhs.0);
    }
}

static UNIT_SCALAR:Scalar = Scalar(1.);


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
//
// TODO: Think about how 3-dimensional vectors can fit into all this.
#[derive(Debug, PartialEq, PartialOrd)]
struct Vector {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Vector {
    pub fn new(x:f64, y:f64, z:f64) -> Vector {
        Vector{
            x: Scalar(x),
            y: Scalar(y),
            z: Scalar(z),
        }
    }
}


impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        return Vector{
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        };
    }
}

static X_UNIT_VECTOR:Vector = Vector{x:Scalar(1.), y:Scalar(0.), z:Scalar(0.)};
static Y_UNIT_VECTOR:Vector = Vector{x:Scalar(0.), y:Scalar(1.), z:Scalar(0.)};
static Z_UNIT_VECTOR:Vector = Vector{x:Scalar(0.), y:Scalar(0.), z:Scalar(1.)};


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

    let s0 = Scalar(4.);
    let s1 = Scalar(-5.);
    assert_eq!(s0 + s1, Scalar(-1.));

    let v0 = Vector::new(4., -7., 2.);
    let v1 = Vector::new(-5., 3., 3.);
    //assert_eq!(v0 + v1, Vector{
    //    x:Scalar(-1.),
    //    z:Scalar(-4.),
    //    y:Scalar(5.),
    //});
    assert_eq!(take_array(&[1, 2, 3]), &[1, 2, 3]);
}


fn take_array(arr:&[i32]) -> &[i32] {
    return arr;
}
