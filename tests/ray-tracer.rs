use cucumber::{given, when, then, World};

// These `Tuple` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct RayTracerWorld {
    tuple: Tuple,
}

// Steps are defined with `given`, `when` and `then` attributes.
// #[given(regex = r"^a tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0.9.]+)\)$")]
// fn a_tuple(world: &mut RayTracerWorld, x: f64, y: f64, z: f64, w: f64) {
#[given(regex = r"^a tuple.*$")]
fn a_tuple(world: &mut RayTracerWorld) {
/*    world.tuple.x = x;
    world.tuple.y = y;
    world.tuple.z = z;
    world.tuple.w = w;
    */
}

#[then(regex = r"^a.x = ([-0-9.]+)$")]
fn a_tuples_x(world: &mut RayTracerWorld, x: f64) {
  assert!(world.tuple.x == x);
}


// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(RayTracerWorld::run("tests/features"));
}
