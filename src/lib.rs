#![allow(dead_code)]

type Scalar = f64;

enum Dimensions2 {
    X,
    Y,
}

enum BiVectors2 {
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

enum Dimensions3 {
    X,
    Y,
    Z,
}

enum BiVectors3 {
    XY,
    YZ,
    ZX,
}

enum TriVectors3 {
    XYZ,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector3 {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BiVector3 {
    xy: Scalar,
    yz: Scalar,
    zx: Scalar,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TriVector3 {
    xyz: Scalar,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MultiThing3 {
    scalar: Scalar,
    vector: Vector2,
    bivector: BiVector2,
    trivector: TriVector3,
}

fn magnitude2(a: Vector2) -> Scalar {
    (a.x.powf(2.) + a.y.powf(2.)).sqrt()
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

// TODO: generalize this to use Multithing3 instead of just Vector3
fn wedge2(a: Vector2, b:Vector2) -> BiVector2 {
    BiVector2 {
        xy: wedge2_magnitude(a, b)
    }
}

/// This is the same as the magnitude of the cross product.
fn wedge2_magnitude(a: Vector2, b: Vector2) -> Scalar {
    (a.x * b.y) - (a.y * b.x)
}

fn magnitude3(a: Vector3) -> Scalar {
    (a.x.powf(2.) + a.y.powf(2.) + a.z.powf(2.)).sqrt()
}

fn plus3(a: Vector3, b: Vector3) -> Vector3 {
    let x = a.x + b.x;
    let y = a.y + b.y;
    let z = a.z + b.z;
    Vector3 {
        x: x,
        y: y,
        z: z,
    }
}

fn minus3(a: Vector3, b: Vector3) -> Vector3 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;
    Vector3 {
        x: x,
        y: y,
        z: z,
    }
}

fn dot3(a: Vector3, b:Vector3) -> Scalar {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

fn wedge3(a: Vector3, b:Vector3) -> BiVector3 {
    let xy = (a.x * b.y) - (a.y * b.x);
    let yz = (a.z * b.x) - (a.x * b.z);
    let zx = (a.y * b.z) - (a.z * b.y);
    BiVector3 {
        xy: xy,
        yz: yz,
        zx: zx,
    }
}

/// This is the same as the magnitude of the cross product.
fn wedge3_magnitude(a: Vector3, b: Vector3) -> Scalar {
    let xy = (a.x * b.y) - (a.y * b.x);
    let yz = (a.z * b.x) - (a.x * b.z);
    let zx = (a.y * b.z) - (a.z * b.y);
    let vec = Vector3 {
        x: xy,
        y: yz,
        z: zx,
    };
    magnitude3(vec)
}

#[test]
fn test_thing() {
    let a = Vector2{x:0., y:2.};
    let b = Vector2{x:3., y:-2.};
    assert_eq!(magnitude2(a), 2.);
    assert_eq!(plus2(a, b), Vector2{x:3., y:0.});
    assert_eq!(minus2(a, b), Vector2{x:-3., y:4.});
    assert_eq!(dot2(a, b), -4.);
    assert_eq!(wedge2(a, b), BiVector2{xy: wedge2_magnitude(a, b)});
    assert_eq!(wedge2_magnitude(a, b), -6.);

    let a = Vector3{x:1., y:2., z:2.};
    let b = Vector3{x:3., y:-2., z:26.};
    assert_eq!(magnitude3(a), 3.);
    assert_eq!(plus3(a, b), Vector3{x:4., y:0., z:28.});
    assert_eq!(minus3(a, b), Vector3{x:-2., y:4., z:-24.});
    assert_eq!(dot3(a, b), 51.);
    assert_eq!(wedge3(a, b), BiVector3{
        xy: -8.,
        yz: -20.,
        zx: 56.,
    });
    assert_eq!(wedge3_magnitude(a, b), 60.);
}
