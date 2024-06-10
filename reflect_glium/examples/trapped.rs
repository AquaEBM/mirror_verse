use reflect::{Float, Ray};
use reflect_mirrors::LineSegment;

fn main() {
    let reflection_cap = std::env::args()
        .nth(1)
        .map(|s| {
            s.parse()
                .expect("reflection cap must be a positive integer")
        })
        .unwrap_or(300);

    let mirrors = [
        LineSegment::new([[2., 0.], [0., 2.]]),
        LineSegment::new([[0., 2.], [-2., 0.]]),
        LineSegment::new([[2., 0.], [0., -2.]]),
        LineSegment::new([[0., -2.], [-2., 0.]]),
    ];

    let rays = [Ray::new([1., 0.33212], [1., 1.2])];

    reflect_glium::run_simulation(&mirrors, rays, Some(reflection_cap), Float::EPSILON * 64.);
}