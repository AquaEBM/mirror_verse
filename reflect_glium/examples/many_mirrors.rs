use reflect::{Float, Ray};
use reflect_mirrors::LineSegment;

fn main() {
    // function alias
    let line = LineSegment::new;

    let mirrors = [
        line([[6.69, 1.32], [3.77, 5.08]]),
        line([[7.615, 1.46], [10.635, 4.86]]),
        line([[12.285, 1.195], [14.585, 6.695]]),
        line([[12.255, 5.08], [7.275, 7.6]]),
        line([[5.86, 7.235], [6.42, 3.095]]),
        line([[7.09, 4.01], [7.29, 4.21]]),
        line([[7.76, 5.455], [11.24, 4.675]]),
        line([[10.185, 4.135], [8.005, 4.955]]),
        line([[7.025, 7.69], [7.805, 5.29]]),
        line([[3.56, 4.395], [3.84, 7.415]]),
        line([[2.705, 2.605], [4.045, 0.825]]),
        line([[4.565, 1.385], [8.465, 0.685]]),
        line([[8.6, 1.305], [14.4, 0.525]]),
        line([[13.155, 0.55], [16.655, 3.11]]),
        line([[16.595, 3.645], [17.295, 5.945]]),
        line([[16.585, 6.57], [15.645, 8.57]]),
        line([[14.345, 9.12], [11.485, 9.92]]),
        line([[11.62, 9.72], [4.5, 9.84]]),
        line([[6.51, 10.04], [2.55, 8.32]]),
        line([[3.28, 9.275], [1.44, 6.055]]),
        line([[1.54, 7.33], [1.94, 2.21]]),
        line([[2.045, 3.365], [4.465, 2.145]]),
        line([[11.825, 2.36], [10.365, 3.2]]),
        line([[10.835, 4.225], [12.655, 4.845]]),
        line([[15.575, 3.09], [14.755, 4.97]]),
        line([[14.215, 7.69], [15.435, 7.01]]),
        line([[14.785, 8.6], [11.685, 6.04]]),
        line([[11.12, 6.315], [9.48, 9.495]]),
        line([[5.855, 8.23], [8.435, 8.39]]),
        line([[5.35, 6.205], [5.03, 8.425]]),
        line([[5.635, 5.88], [4.335, 5.16]]),
        line([[2.08, 2.905], [4.76, 4.565]]),
        line([[5.78, 1.395], [8.46, 3.055]]),
        line([[16.1, 1.885], [16.54, 4.345]]),
        line([[16.265, 2.85], [17.845, 4.77]]),
        line([[9.785, 9.865], [12.565, 9.445]]),
        line([[17.21, 7.165], [15.13, 10.145]]),
        line([[10.455, 1.74], [7.395, 1.62]]),
    ];

    let rays = [Ray::new([9., 5.], [1., 2.])];

    reflect_glium::run_simulation(&mirrors, rays, None, Float::EPSILON * 64.)
}
