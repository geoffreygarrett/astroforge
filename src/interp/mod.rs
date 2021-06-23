mod algo;

use ndarray::Array2;
use ndarray::Array1;



struct Interp1DBuilder {}


enum Interpolant1D {
    Linear,
    Nearest,
}

struct Interpolant {
    method: Interpolant1D,
    y_span: [Array2<f64>; 2],
    x_span: [f64; 2],
}

impl Interpolant {
    fn new(method: Interpolant1D, y_span: [Array2<f64>; 2], x_span: [f64; 2]) -> Self {
        Self {
            method,
            y_span,
            x_span,
        }
    }
}


trait Interpolator1D<T> {
    fn set_y(&self, y: Array2<T>) -> &self {
        self.y = y;
        self
    }
    fn set_x(&self, x: Array1<T>) -> &self {
        self.x = x;
        self
    }
    fn interp_value(value: f64) -> Array1<f64>;
    fn interp_array(array: Array1<f64>) -> Array2<f64>;
}

struct Linear<T> {
    y: Array2<T>,
    x: Array1<T>,
}

impl<T> Linear<T> {
    fn new(x: Array1<f64>, y: Array2<f64>) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T> Interpolator1D<T> for Linear<T> {
    fn interp_value(value: f64) -> Array1<f64> {
        todo!()
    }

    fn interp_array(array: Array1<f64>) -> Array2<f64> {
        todo!()
    }
}


// struct Interp1D {
//
// }
//
// struct Interp2D {
//
// }

enum Interp1D {
    NearestNeighbour,
    Linear,
    Cubic,
}

enum Interp2D {
    NearestNeighbour,
    Bilinear,
    Bicubic,
}


#[cfg(test)]
mod test {
    use crate::interp::Interp1D::Linear;
    use nalgebra::Vector1;
    use ndarray::Array1;
    use crate::interp::Interp1D;

    #[test]
    fn test_linear() {
        let x = Array1::new([1., 2., 3., 4.]);
        let y = Array2::new(
            [
                [1., 2., 3., 4.],
                [1., 2., 3., 4.],
                [1., 2., 3., 4.],
                [1., 2., 3., 4.]
            ]);

        let interpolator = Linear::new()
            .method(Interp1D::Linear)
            // .error(false)
            .set_y(y)
            .set_x(x)
            .build();

        let result1 = interpolator.interp_value(2.5);
        let result2 = interpolator.interp_array(array![2.5, 2.9]);
    }
}