use std::fmt;
use std::ops;
use std::str;
use std::num;

pub struct Measure<T> {
    pub value: T,
    pub error: T
}

impl str::FromStr for Measure<f32> {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.trim().trim_matches(|p| p == '(' || p == ')').split('±').collect();
        let val: f32 = input[0].parse()?;
        let er: f32 = input[1].parse()?;
        Ok(Measure::<f32>{value: val, error: er})
    }
}

impl<T: Copy + ops::Div> Measure<T> {
    fn relative_error(&self) -> <T as ops::Div>::Output {
        return self.error / self.value;
    }
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

impl<T: ops::Add<Output = T> + ops::Sub<Output = T>> ops::Sub for Measure<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            error: self.error + other.error
        }
    }
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>> ops::Mul for Measure<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
            error: (self.relative_error() + other.relative_error()) * (self.value * other.value)
        }
    }
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>> ops::Div for Measure<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
            error: (self.relative_error() + other.relative_error()) * (self.value * other.value)
        }
    }
}