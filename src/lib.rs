#![allow(dead_code)]

use std::ops::Add;

type ScaledBasisBlade = u32;

// TODO: abstract these all into k-blades somehow.
// OR just give them a trait called k-blade that lets them be used together easily.

type Scalar = f64;

static UNIT_SCALAR:Scalar = 1.;


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
struct V3(Scalar, Scalar, Scalar);

//TODO: use Vec<T> instead of an array like this.
struct Vctr<'a>(
    &'a [Scalar]
);

//fn add_vctr(left:&Vctr, right:&Vctr) -> Vctr {
//}


impl Add<V3> for V3 {
    type Output = V3;

    fn add(self, _rhs: V3) -> V3 {
        return V3(
            self.0 + _rhs.0,
            self.1 + _rhs.1,
            self.2 + _rhs.2,
        );
    }
}

static X_UNIT_VECTOR:V3 = V3(1., 0., 0.);
static Y_UNIT_VECTOR:V3 = V3(0., 1., 0.);
static Z_UNIT_VECTOR:V3 = V3(0., 0., 1.);


// TODO: store one scalar for each of:
// e_12 e_23 e_31
// ALTERNATIVELY:
// ??? a magnitude and two angles ???
// ??? a V3 and a magnitude ???
struct Bivector {
    magnitude: Scalar,
    vector: V3,
}

struct MultiVector {
    scalar: Scalar,
    vector: V3,
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

/// The following is for 2 dimensions only
enum TwoDirections {
    X,
    Y,
}

enum OneBiVectorThing {
    XY,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector2 {
    x: Scalar,
    y: Scalar,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BiVector2 {
    xy: Scalar,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MultiThing2 {
    scalar: Scalar,
    vector: Vector2,
    bivector: BiVector2,
}

fn plus2(a: Vector2, b: Vector2) -> Vector2 {
    let x = a.x + b.x;
    let y = a.y + b.y;
    Vector2 {
        x: x,
        y: y,
    }
}

fn minus2(a: Vector2, b: Vector2) -> Vector2 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    Vector2 {
        x: x,
        y: y,
    }
}

fn dot2(a: Vector2, b:Vector2) -> Scalar {
    (a.x * b.x) + (a.y * b.y)
}

fn wedge2(a: Vector2, b:Vector2) -> BiVector2 {
    BiVector2 {
        xy: wedge2_magnitude(a, b)
    }
}

/// This is the same as the magnitude of the cross product.
fn wedge2_magnitude(a: Vector2, b: Vector2) -> Scalar {
    (a.x * b.y) - (a.y * b.x)
}

#[test]
fn test_thing() {
    let a = Vector2{x:1., y:2.};
    let b = Vector2{x:3., y:-2.};
    assert_eq!(plus2(a, b), Vector2{x:4., y:0.});
    assert_eq!(minus2(a, b), Vector2{x:-2., y:4.});
    assert_eq!(dot2(a, b), -1.);
    assert_eq!(wedge2(a, b), BiVector2{xy: wedge2_magnitude(a, b)});
    assert_eq!(wedge2_magnitude(a, b), -8.);
}

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

    let s0 = 4.;
    let s1 = -5.;
    assert_eq!(s0 + s1, -1.);

    let v0 = V3(4., -7., 2.);
    let v1 = V3(-5., 3., 3.);
    //assert_eq!(v0 + v1, V3(
    //    -1.,
    //    -4.,
    //    5.,
    //));
    assert_eq!(take_array(&[1, 2, 3]), &[1, 2, 3]);
}


fn take_array(arr:&[i32]) -> &[i32] {
    return arr;
}
