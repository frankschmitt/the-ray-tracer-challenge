// These `Tuple` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    return Tuple { x: x, y: y, z: z, w: 0.0};
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    return Tuple { x: x, y: y, z: z, w: 1.0};
}



