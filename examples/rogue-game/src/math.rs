use num::integer::Roots;

pub trait Root {
    type Output;

    fn sqrt(self) -> Self::Output;
}

impl Root for f64 {
    type Output = f64;

    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

impl Root for f32 {
    type Output = f32;

    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

impl Root for i32 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for u32 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for i64 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for u64 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for i128 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for u128 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for i8 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for u8 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for i16 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}

impl Root for u16 {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Roots::sqrt(&self)
    }
}
