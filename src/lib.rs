/*!
A port of [`d3-ticks`](https://observablehq.com/@d3/d3-ticks), part of the JavaScript D3 plotting library.

Generates an array of nicely rounded values between two numbers for which are ideal for positioning axis labels and grid-lines.
```
let ticks = axis_ticks::ticks(-0.125, 0.25, 10);

assert_eq!(
    ticks,
    [-0.15, -0.1, -0.05, 0.0, 0.05, 0.1, 0.15, 0.2, 0.25]
);
```
*/

use num_traits::{
    cast::FromPrimitive,
    float::{Float, FloatConst},
};

pub fn ticks<T: Float + FloatConst + FromPrimitive>(start: T, stop: T, count: usize) -> Vec<T> {
    if start == stop && count > 0 {
        return vec![start];
    }

    let reverse = stop < start;
    let (start, stop) = if reverse {
        (stop, start)
    } else {
        (start, stop)
    };

    let step = tick_increment(start, stop, count);
    if step.is_zero() || !step.is_finite() {
        return vec![];
    }

    let mut ticks = if step.is_sign_positive() {
        let start: T = (start / step).ceil();
        let stop: T = (stop / step).floor();
        let n = (stop - start + T::from_f64(1.0).unwrap())
            .ceil()
            .to_usize()
            .unwrap();
        let mut ticks = vec![T::from_f64(0.0).unwrap(); n];
        for i in 0..n {
            ticks[i] = (start + T::from_usize(i).unwrap()) * step;
        }
        ticks
    } else {
        let step = step * T::from_f64(-1.0).unwrap();
        let start = (start * step).floor();
        let stop = (stop * step).ceil();
        let n = (stop - start + T::from_f64(1.0).unwrap())
            .ceil()
            .to_usize()
            .unwrap();
        let mut ticks = vec![T::from_f64(0.0).unwrap(); n];
        for i in 0..n {
            ticks[i] = (start + T::from_usize(i).unwrap()) / step;
        }
        ticks
    };

    if reverse {
        ticks.reverse()
    }

    ticks
}

fn tick_increment<T: Float + FloatConst + FromPrimitive>(start: T, stop: T, count: usize) -> T {
    let step = (stop - start) / T::from_usize(count).unwrap();
    let power = (step.ln() / T::LN_10()).floor();
    let error = step / T::from_f64(10.0).unwrap().powf(power);

    let v = if error >= T::from_f64(50.0).unwrap().sqrt() {
        T::from_f64(10.0).unwrap()
    } else if error >= T::from_f64(10.0).unwrap().sqrt() {
        T::from_f64(5.0).unwrap()
    } else if error >= T::from_f64(2.0).unwrap().sqrt() {
        T::from_f64(2.0).unwrap()
    } else {
        T::from_f64(1.0).unwrap()
    };

    if power >= T::from_f64(0.0).unwrap() {
        v * T::from_f64(10.0).unwrap().powf(power)
    } else {
        (T::from_f64(-1.0).unwrap()
            * T::from_f64(10.0)
                .unwrap()
                .powf(power * T::from_f64(-1.0).unwrap()))
            / v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_vec_if_any_argument_is_nan() {
        assert_eq!(ticks(f32::NAN, 1.0, 1), []);
        assert_eq!(ticks(0.0, f32::NAN, 1), []);
        assert_eq!(ticks(f32::NAN, f32::NAN, 1), []);
    }

    #[test]
    fn returns_the_empty_vec_if_start_equal_stop() {
        assert_eq!(ticks(1.0, 1.0, 0), []);
    }

    #[test]
    fn returns_the_empty_vec_if_count_is_not_positive() {
        assert_eq!(ticks(0.0, 1.0, 0), []);
    }

    #[test]
    fn returns_approximately_count_plus_1_ticks_when_start_less_than_stop() {
        assert_eq!(
            ticks(0.0f32, 1.0f32, 10),
            [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
        );
        assert_eq!(
            ticks(0.0f64, 1.0f64, 10),
            [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
        );
        assert_eq!(
            ticks(0.0, 1.0, 8),
            [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
        );
        assert_eq!(ticks(0.0, 1.0, 7), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(ticks(0.0, 1.0, 6), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(ticks(0.0, 1.0, 5), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(ticks(0.0, 1.0, 4), [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
        assert_eq!(ticks(0.0, 1.0, 3), [0.0, 0.5, 1.0]);
        assert_eq!(ticks(0.0, 1.0, 2), [0.0, 0.5, 1.0]);
        assert_eq!(ticks(0.0, 1.0, 1), [0.0, 1.0]);
        assert_eq!(
            ticks(0.0, 10.0, 10),
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        );
        assert_eq!(
            ticks(0.0, 10.0, 9),
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        );
        assert_eq!(
            ticks(0.0, 10.0, 8),
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        );
        assert_eq!(ticks(0.0, 10.0, 7), [0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
        assert_eq!(ticks(0.0, 10.0, 6), [0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
        assert_eq!(ticks(0.0, 10.0, 5), [0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
        assert_eq!(ticks(0.0, 10.0, 4), [0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
        assert_eq!(ticks(0.0, 10.0, 3), [0.0, 5.0, 10.0]);
        assert_eq!(ticks(0.0, 10.0, 2), [0.0, 5.0, 10.0]);
        assert_eq!(ticks(0.0, 10.0, 1), [0.0, 10.0]);
        assert_eq!(
            ticks(-10.0, 10.0, 10),
            [-10.0, -8.0, -6.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 8.0, 10.0]
        );
        assert_eq!(
            ticks(-10.0, 10.0, 9),
            [-10.0, -8.0, -6.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 8.0, 10.0]
        );
        assert_eq!(
            ticks(-10.0, 10.0, 8),
            [-10.0, -8.0, -6.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 8.0, 10.0]
        );
        assert_eq!(
            ticks(-10.0, 10.0, 7),
            [-10.0, -8.0, -6.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 8.0, 10.0]
        );
        assert_eq!(ticks(-10.0, 10.0, 6), [-10.0, -5.0, 0.0, 5.0, 10.0]);
        assert_eq!(ticks(-10.0, 10.0, 5), [-10.0, -5.0, 0.0, 5.0, 10.0]);
        assert_eq!(ticks(-10.0, 10.0, 4), [-10.0, -5.0, 0.0, 5.0, 10.0]);
        assert_eq!(ticks(-10.0, 10.0, 3), [-10.0, -5.0, 0.0, 5.0, 10.0]);
        assert_eq!(ticks(-10.0, 10.0, 2), [-10.0, 0.0, 10.0]);
        assert_eq!(ticks(-10.0, 10.0, 1), [0.0]);
    }

    #[test]
    fn some_more_complex_tests() {
        assert_eq!(
            ticks(0.0, 1.0, 20),
            [
                0.0, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7,
                0.75, 0.8, 0.85, 0.9, 0.95, 1.0
            ]
        );

        assert_eq!(
            ticks(0.125, 0.25, 5),
            [0.12, 0.14, 0.16, 0.18, 0.2, 0.22, 0.24, 0.26]
        );

        assert_eq!(
            ticks(0.125, 0.25, 10),
            [0.12, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25]
        );

        assert_eq!(
            ticks(-0.125, 0.25, 10),
            [-0.15, -0.1, -0.05, 0.0, 0.05, 0.1, 0.15, 0.2, 0.25]
        );
    }
}
