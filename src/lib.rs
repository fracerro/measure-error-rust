use std::ops;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Measure<T> {
    pub value: T,
    pub error: T
}

impl<T> Measure<T> where 
    T: Copy + ops::Div<T, Output = T> {
    fn relative_error(&self) -> T {
        self.error / self.value
    }
}

impl<T> fmt::Display for Measure<T> where 
    T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.error)
    }
}

impl<T> ops::Add<Measure<T>> for Measure<T> where
    T: ops::Add<T, Output = T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Measure {
            value: self.value + other.value,
            error: self.error + other.error
        }
    }
}

impl<T> ops::Sub<Measure<T>> for Measure<T> where
    T: ops::Add<T, Output = T> + ops::Sub<T, Output = T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Measure {
            value: self.value - other.value,
            error: self.error + other.error
        }
    }
}

impl<T> ops::Mul<Measure<T>> for Measure<T> where
    T: Copy + ops::Add<T, Output = T> + ops::Div<T, Output = T> + ops::Mul<T, Output = T> {
    type Output = Measure<T>;
    fn mul(self, other: Self) -> Self {
        let val = self.value * other.value;
        let err = val * (self.relative_error() + other.relative_error());
        Measure {
            value: val,
            error: err
        }
    }
}

impl<T> ops::Mul<T> for Measure<T> where
    T: Copy + ops::Mul<T, Output = T> {
    type Output = Measure<T>;
    fn mul(self, other: T) -> Self {
        Measure {
            value: self.value * other,
            error: self.error * other
        }
    }
}

impl<T> ops::Div<Measure<T>> for Measure<T> where
    T: Copy + ops::Add<T, Output = T> + ops::Div<T, Output = T> + ops::Mul<T, Output = T> {
    type Output = Measure<T>;
    fn div(self, other: Self) -> Self {
        let val = self.value / other.value;
        let err = val * (self.relative_error() + other.relative_error());
        Measure {
            value: val,
            error: err
        }
    }
}

impl<T> ops::Div<T> for Measure<T> where
    T: Copy + ops::Div<T, Output = T> {
    type Output = Measure<T>;
    fn div(self, other: T) -> Self {
        Measure {
            value: self.value / other,
            error: self.error / other
        }
    }
}

// ----------------

// more measures of the same thing
#[derive(Debug, Clone)]
pub struct Dataset<T> {
    data: Vec<Measure<T>>
}

impl<T> ops::Index<usize> for Dataset<T> {
    type Output = Measure<T>;
    fn index(&self, index: usize) -> &Measure<T> {
        return &self.data[index];
    }
}

impl<T> ops::IndexMut<usize> for Dataset<T> {
    fn index_mut(&mut self, index: usize) -> &mut Measure<T> {
        return &mut self.data[index];
    }
}

impl<T> Dataset<T> where 
    T: Copy + ops::Add<T, Output = T> + ops::Div<usize, Output = T> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn avarage(&self) -> Option<T> {
            if self.len() == 0 {
                None
            } else {
                let mut sum: T = self[0].value;
                for i in 1..self.len() {
                    sum = sum + self[i].value;
                }
                
                Some(sum / self.len())
            }
        }
}

// ----------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m: Measure<f32> = Measure {
            value: 2.0,
            error: 0.1
        };
        let p: Measure<f32> = Measure {
            value: 4.0,
            error: 0.5
        };
        println!("{}", m / std::f32::consts::PI);
        println!("{}", p / std::f32::consts::PI);

        let mut prova: Dataset<f32> = Dataset {
            data: Vec::from([m, p])
        };
        prova[0].error = 0.3;
        println!("{}", prova[0]);
        println!("{:?}", prova);
    }
}