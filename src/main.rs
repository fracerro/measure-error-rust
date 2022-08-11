// use std::ops;
use std::fmt;
use std::ops;

struct Measure<T> {
    value: T,
    error: T
}

impl<T: fmt::Display> fmt::Display for Measure<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.error)
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Measure<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            error: self.error + other.error
        }
    }
}

fn main() {
    let m1 = Measure::<f32> {
        value: 10.58,
        error: 0.018
    };
    let m2 = Measure::<f32> {
        value: 6.00,
        error: 0.300
    };

    println!("{}", m1 + m2);
}
