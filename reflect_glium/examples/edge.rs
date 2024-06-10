use reflect::{Float, Ray};
use reflect_mirrors::Simplex;

fn main() {
    let mirror = Simplex::new([[1., 0.], [0., 1.99999]]);
    let rays = [Ray::new([0., 0.], [1., 0.])];
    reflect_glium::run_simulation(&mirror, rays, None, Float::EPSILON * 64.)
}