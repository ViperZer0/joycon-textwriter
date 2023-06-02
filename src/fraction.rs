use std::ops::Rem;
use std::ops::Div;
pub struct Fraction<T>
{
    pub numerator: T,
    pub denom: T
}

impl<T: Rem<Output = T> + PartialEq + Div<Output = T> + Default + Copy> Fraction<T>
{
    pub fn gcf(&self) -> T
    {
        let mut a = self.numerator;
        let mut b = self.denom;
        while b != T::default()
        {
            let temp = b;
            b = a.rem(b);
            a = temp;
        }
        return a;
    }

    pub fn reduce_mut(&mut self)
    {
        let factor = self.gcf();
        self.numerator = self.numerator / factor;
        self.denom = self.denom / factor;
    }

    /*
    pub fn reduce(&self) -> Self
    {
        let new = self.clone();
        new.reduce_mut();
        // Can't do this.
        new
    }
    */
}
