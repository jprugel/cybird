use crate::math::Root;
use num::{Num, One, pow};
use std::ops::Mul;

struct Vector2<T: Num> {
    x: T,
    y: T,
}

trait Magnitude {
    type Output;

    fn magnitude(self) -> Self::Output;
}

impl<T> Magnitude for Vector2<T>
where
    T: Num + Clone,
    T: Root<Output = T>,
    T: Mul<Output = T>,
    T: One<Output = T>,
{
    type Output = T;

    fn magnitude(self) -> Self::Output {
        (pow(self.x, 2) + pow(self.y, 2)).sqrt()
    }
}

trait Distance {
    type Output;

    fn distance(self, other: Self) -> Self::Output;
}

impl<T> Distance for Vector2<T>
where
    T: Num + Clone,
    T: Root<Output = T>,
    T: Mul<Output = T>,
    T: One<Output = T>,
{
    type Output = T;

    fn distance(self, other: Self) -> Self::Output {
        (pow(self.x - other.x, 2) + pow(self.y - other.y, 2)).sqrt()
    }
}
