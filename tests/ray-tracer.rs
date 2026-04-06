use cucumber::{given, when, then, World};
use the_ray_tracer_challenge::tuple::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct RayTracerWorld {
    tuple: Tuple,
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(regex = r"^a ← tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
fn a_tuple(world: &mut RayTracerWorld, x: f64, y: f64, z: f64, w: f64) {
    world.tuple.x = x;
    world.tuple.y = y;
    world.tuple.z = z;
    world.tuple.w = w;
}

#[then(regex = r"^a.x = ([-0-9.]+)$")]
fn a_tuples_x(world: &mut RayTracerWorld, x: f64) {
  assert!(world.tuple.x == x);
}

#[then(regex = r"^a.y = ([-0-9.]+)$")]
fn a_tuples_y(world: &mut RayTracerWorld, y: f64) {
  assert!(world.tuple.y == y);
}

#[then(regex = r"^a.z = ([-0-9.]+)$")]
fn a_tuples_z(world: &mut RayTracerWorld, z: f64) {
  assert!(world.tuple.z == z);
}

#[then(regex = r"^a.w = ([-0-9.]+)$")]
fn a_tuples_w(world: &mut RayTracerWorld, w: f64) {
  assert!(world.tuple.w == w);
}

#[then(regex = r"^a is a (point|vector)$")]
fn a_tuples_type(world: &mut RayTracerWorld, t: String) {
  if (t == "point") {
    // Points have w = 1
    assert!(world.tuple.w == 1.0);
  }
  else {
    // Vectors have w = 0
    assert!(world.tuple.w == 0.0);
  }
}

#[then(regex = r"^a is not a (point|vector)$")]
fn a_tuples_type_not(world: &mut RayTracerWorld, t: String) {
  if (t == "point") {
    // Points have w = 1
    assert!(world.tuple.w != 1.0);
  }
  else {
    // Vectors have w = 0
    assert!(world.tuple.w != 0.0);
  }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(regex = r"^p ← point\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
fn p_point(world: &mut RayTracerWorld, x: f64, y: f64, z: f64) {
  world.tuple = point(x,y,z);
}


// Steps are defined with `given`, `when` and `then` attributes.
#[given(regex = r"^v ← vector\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
fn v_vector(world: &mut RayTracerWorld, x: f64, y: f64, z: f64) {
  world.tuple = vector(x,y,z);
}

#[then(regex = r"^p = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
fn p_is_a_tuple(world: &mut RayTracerWorld, x: f64, y: f64, z: f64, w: f64) {
    assert!(world.tuple.x == x);
    assert!(world.tuple.y == y);
    assert!(world.tuple.z == z);
    assert!(world.tuple.w == w);
}

#[then(regex = r"^v = tuple\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)$")]
fn v_is_a_tuple(world: &mut RayTracerWorld, x: f64, y: f64, z: f64, w: f64) {
    assert!(world.tuple.x == x);
    assert!(world.tuple.y == y);
    assert!(world.tuple.z == z);
    assert!(world.tuple.w == w);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(RayTracerWorld::run("tests/features"));
}
