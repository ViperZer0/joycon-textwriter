use crate::fraction::Fraction;
use std::fmt::{Display, Formatter, Result};

pub struct Resample {}

#[derive(Error, Debug)]
enum ResampleError
{
    ZeroSampleSize,
    UpsampleError,
}

impl Display for ResampleError
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        match(self)
        {
            ZeroSampleSize => write!(f, "Sample size of 0 was specified."),
            UpsampleError => write!(f, "Number of samples specified greater than source number of samples. Upsampling is currently not supported, only downsampling."),
        }
    }
}

    
impl Resample
{
    // Only returns a new vector if we can actually downsample the dataset.
    pub fn resample<T: Average>(dataset: &[T], num_samples: usize) -> Result<Vec<T>, ResampleError>
    {
        if num_samples == 0
        {
            return Err(ResampleError::ZeroSampleSize);
        }
        else if dataset.len() <= num_samples
        {
            return Err(ResampleError::UpsampleError);
        }
        else
        {
            let ratio = Self::ratio(dataset.len(), num_samples);
            let size = 1 + ratio.numerator - ratio.denom;
            let new_vec = dataset.windows(size).map(|x| T::average(&x)).collect();
            return Ok(new_vec);
        }
    }

    fn ratio(size_1: usize, size_2: usize) -> Fraction<usize>
    {
        let mut f = Fraction {
            numerator: size_1,
            denom: size_2,
        };
        f.reduce_mut();
        return f;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_ratio_1()
    {
        let frac = Resample::ratio(2, 1);
        assert_eq!(2, frac.numerator);
        assert_eq!(1, frac.denom);
    }

    #[test]
    fn test_ratio_2()
    {
        let frac = Resample::ratio(3, 2);
        assert_eq!(3, frac.numerator);
        assert_eq!(2, frac.denom);
    }

    #[test]
    fn test_ratio_same()
    {
        let frac = Resample::ratio(5, 5);
        assert_eq!(1, frac.numerator);
        assert_eq!(1, frac.denom);
    }

    #[test]
    fn test_ratio_simplify()
    {
        let frac = Resample::ratio(4, 2);
        assert_eq!(2, frac.numerator);
        assert_eq!(1, frac.denom);
    }

    #[test]
    fn test_resample()
    {
        let vec = vec!([1.0, 2.0, 3.0, 4.0]);
        let result = Resample::resample(vec, 2);
        assert!(result.is_ok());
        assert_eq!(vec!([1.5, 3.5]), result.unwrap());
    }

    #[test]
    fn test_resample_5_to_2()
    {
        let vec = vec!([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = Resample::resample(vec, 2);
        assert!(result.is_ok());
        assert_eq!(vec!([2.5, 3.5]), result.unwrap());
    }
    
    #[test]
    fn test_resample_zero_error()
    {
        let vec = vec!([1.0, 2.0]);
        let result = Resample::resample(vec, 0);
        assert!(result.is_err());
        match result.unwrap_err()
        {
            ZeroSampleSize => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_resample_upsample_error()
    {
        let vec = vec!([1.0, 2.0]);
        let result = Resample::resample(vec, 3);
        assert!(result.is_err());
        match result.unwrap_err();
        {
            UpsampleError => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_resample_exact()
    {
        let vec = vec!([1.0, 2.0]);
        let result = Resample::resample(vec, 2);
        assert!(result.is_ok());
        assert_eq!(vec, result.unwrap());
    }
}
