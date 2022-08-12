use std::fmt;
use std::ops;
use std::str;

/*
struct Measure per una sola misura
*/

pub struct Measure<T: Copy> {
    pub value: T,
    pub error: T
}

impl<T: Copy> Copy for Measure<T> {}
impl<T: Copy> Clone for Measure<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: str::FromStr + Copy> str::FromStr for Measure<T> {
    type Err = <T as str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.trim().trim_matches(|p| p == '(' || p == ')').split('±').collect();
        let val: T = input[0].parse()?;
        let er: T = input[1].parse()?;
        Ok(Measure::<T>{value: val, error: er})
    }
}

impl<T: Copy + ops::Div> Measure<T> {
    fn relative_error(&self) -> <T as ops::Div>::Output {
        return self.error / self.value;
    }
}

impl<T: Copy + fmt::Display> fmt::Display for Measure<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.error)
    }
}

impl<T: Copy + ops::Add<Output = T>> ops::Add for Measure<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            error: self.error + other.error
        }
    }
}

impl<T: Copy + ops::Add<Output = T> + ops::Sub<Output = T>> ops::Sub for Measure<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            error: self.error + other.error
        }
    }
}

/*
impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>> ops::Mul<Measure<T>> for Measure<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
            error: (self.relative_error() + other.relative_error()) * (self.value * other.value)
        }
    }
}
*/

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>> ops::Div for Measure<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
            error: (self.relative_error() + other.relative_error()) * (self.value * other.value)
        }
    }
}

impl<T: Copy + ops::Mul<U, Output = T>, U: Copy> ops::Mul<U> for Measure<T> {
    type Output = Self;
    fn mul(self, other: U) -> Self::Output {
        Self {
            value: self.value * other,
            error: self.error * other
        }
    }
}