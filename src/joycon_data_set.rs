use crate::fraction::Fraction;
use crate::models::JoyconData;
use crate::average::Average;
use std::default::Default;

#[derive(Default)]
pub struct JoyconDataPoint {
    pub time: Option<f32>,
    pub gyro_x: Option<f32>,
    pub gyro_y: Option<f32>,
    pub gyro_z: Option<f32>,
    pub accel_x: Option<f32>,
    pub accel_y: Option<f32>,
    pub accel_z: Option<f32>,
}

impl JoyconDataPoint
{
    fn label(&self, symbol: &str, training_num: i32, sample_num: i32) -> JoyconData 
    {
        JoyconData {
            symbol: symbol.to_owned(),
            training_num: training_num,
            sample_num: sample_num,
            time: self.time,
            gyro_x: self.gyro_x,
            gyro_y: self.gyro_y,
            gyro_z: self.gyro_z,
            accel_x: self.accel_x,
            accel_y: self.accel_y,
            accel_z: self.accel_z,
        }
    }
}

impl Average for JoyconDataPoint
{
    // Get the average of a slice of data points and return a single point.
    fn average(dataset: &[JoyconDataPoint]) -> JoyconDataPoint
    {
        // Some of these may be missing, so we want to count how many we actually have.
        let mut time_samples: usize = 0;
        let mut gyro_x_samples: usize = 0;
        let mut gyro_y_samples: usize = 0;
        let mut gyro_z_samples: usize = 0;
        let mut accel_x_samples: usize = 0;
        let mut accel_y_samples: usize = 0;
        let mut accel_z_samples: usize = 0;

        let mut sum = JoyconDataPoint::default();
        // Sum up the slice's points.
        dataset.iter().for_each(|x| add_points(&mut sum, &x, &mut time_samples,
                                         &mut gyro_x_samples, &mut gyro_y_samples,
                                         &mut gyro_z_samples, &mut accel_x_samples,
                                         &mut accel_y_samples, &mut accel_z_samples));

        // Get and return the average.
        JoyconDataPoint {
            time: Some(sum.time.unwrap_or_default() / time_samples as f32),
            gyro_x: Some(sum.gyro_x.unwrap_or_default() / gyro_x_samples as f32),
            gyro_y: Some(sum.gyro_y.unwrap_or_default() / gyro_y_samples as f32),
            gyro_z: Some(sum.gyro_z.unwrap_or_default() / gyro_z_samples as f32),
            accel_x: Some(sum.accel_x.unwrap_or_default() / accel_x_samples as f32),
            accel_y: Some(sum.accel_y.unwrap_or_default() / accel_y_samples as f32),
            accel_z: Some(sum.accel_z.unwrap_or_default() / accel_z_samples as f32),
        }
    }
}

pub struct JoyconDataSet {
    pub symbol: String,
    pub training_num: i32,
    pub data_points: Vec<JoyconDataPoint>,
}

// Add two points while checking for None values for each struct.
// Only add to the number of samples if a Some value is found.
fn add_points(sum_point: &mut JoyconDataPoint, other_point: &JoyconDataPoint,
              time_samples: &mut usize, gyro_x_samples: &mut usize,
              gyro_y_samples: &mut usize, gyro_z_samples: &mut usize,
              accel_x_samples: &mut usize, accel_y_samples: &mut usize,
              accel_z_samples: &mut usize)
{
    if let Some(val) = other_point.time
    {
        // This should default to 0 if the sum_point value is None.
        sum_point.time = Some(sum_point.time.unwrap_or_default() + val);
        *time_samples += 1;
    }
    if let Some(val) = other_point.gyro_x
    {
        sum_point.gyro_x = Some(sum_point.gyro_x.unwrap_or_default() + val);
        *gyro_x_samples += 1;
    }
    if let Some(val) = other_point.gyro_y
    {
        sum_point.gyro_y = Some(sum_point.gyro_y.unwrap_or_default() + val);
        *gyro_y_samples += 1;
    }
    if let Some(val) = other_point.gyro_z
    {
        sum_point.gyro_z = Some(sum_point.gyro_z.unwrap_or_default() + val);
        *gyro_z_samples += 1;
    }
    if let Some(val) = other_point.accel_x
    {
        sum_point.accel_x = Some(sum_point.accel_x.unwrap_or_default() + val);
        *accel_x_samples += 1;
    }
    if let Some(val) = other_point.accel_y
    {
        sum_point.accel_y = Some(sum_point.accel_y.unwrap_or_default() + val);
        *accel_y_samples += 1;
    }
    if let Some(val) = other_point.accel_z
    {
        sum_point.accel_z = Some(sum_point.accel_z.unwrap_or_default() + val);
        *accel_z_samples += 1;
    }
}

impl From<&JoyconDataSet> for Vec<JoyconData>
{
    fn from(item: &JoyconDataSet) -> Self
    {
        let mut index = -1;
        // maybe we want to sort this?
        // Add an incrementing index.
        item.data_points.iter().map(|x| { index += 1; x.label(&item.symbol, item.training_num, index)}).collect()
    }
}

impl JoyconDataSet
{
    //TODO: this can probably actually be super generic for testing
    pub fn resample(self: &mut Self, num_samples: usize) -> &Self
    {
        if self.data_points.len() <= num_samples
        {
            return self;
        }
        else
        {
            let ratio = Self::ratio(self.data_points.len(), num_samples);
            // The size of a window to get the average of is 1 + the difference between the
            // numerator and the denominator. A ratio of 7:5 takes the average of 3 elements.
            let size = 1 + ratio.numerator - ratio.denom;
            self.data_points = self.data_points.windows(size).map(|x| JoyconDataPoint::average(&x)).collect();
            return self;
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
    fn test_add_points_normal() {
        let mut time_samples: usize = 0;
        let mut gyro_x_samples: usize = 0;
        let mut gyro_y_samples: usize = 0;
        let mut gyro_z_samples: usize = 0;
        let mut accel_x_samples: usize = 0;
        let mut accel_y_samples: usize = 0;
        let mut accel_z_samples: usize = 0;

        let mut point_1 = JoyconDataPoint {
            time: Some(0.0),
            gyro_x: Some(0.0),
            gyro_y: Some(0.0),
            gyro_z: Some(0.0),
            accel_x: Some(0.0),
            accel_y: Some(0.0),
            accel_z: Some(0.0),
        };

        let point_2 = JoyconDataPoint {
            time: Some(1.0),
            gyro_x: Some(1.0),
            gyro_y: Some(1.0),
            gyro_z: Some(1.0),
            accel_x: Some(1.0),
            accel_y: Some(1.0),
            accel_z: Some(1.0),
        };

        add_points(&mut point_1, &point_2, &mut time_samples,
                   &mut gyro_x_samples, &mut gyro_y_samples,
                   &mut gyro_z_samples, &mut accel_x_samples,
                   &mut accel_y_samples, &mut accel_z_samples);

        assert_eq!(1.0, point_1.time.unwrap());
        assert_eq!(1.0, point_1.gyro_x.unwrap());
        assert_eq!(1.0, point_1.gyro_y.unwrap());
        assert_eq!(1.0, point_1.gyro_z.unwrap());
        assert_eq!(1.0, point_1.accel_x.unwrap());
        assert_eq!(1.0, point_1.accel_y.unwrap());
        assert_eq!(1.0, point_1.accel_z.unwrap());
        assert_eq!(1, time_samples);
        assert_eq!(1, gyro_x_samples);
        assert_eq!(1, gyro_y_samples);
        assert_eq!(1, gyro_z_samples);
        assert_eq!(1, accel_x_samples);
        assert_eq!(1, accel_y_samples);
        assert_eq!(1, accel_z_samples);
    }

    #[test]
    fn add_points_missing_vals()
    {
 
        let mut time_samples: usize = 0;
        let mut gyro_x_samples: usize = 0;
        let mut gyro_y_samples: usize = 0;
        let mut gyro_z_samples: usize = 0;
        let mut accel_x_samples: usize = 0;
        let mut accel_y_samples: usize = 0;
        let mut accel_z_samples: usize = 0;

        let mut point_1 = JoyconDataPoint::default();

        let point_2 = JoyconDataPoint {
            time: Some(1.0),
            gyro_x: Some(1.0),
            gyro_y: Some(1.0),
            gyro_z: Some(1.0),
            accel_x: Some(1.0),
            accel_y: Some(1.0),
            accel_z: Some(1.0),
        };

        add_points(&mut point_1, &point_2, &mut time_samples,
                   &mut gyro_x_samples, &mut gyro_y_samples,
                   &mut gyro_z_samples, &mut accel_x_samples,
                   &mut accel_y_samples, &mut accel_z_samples);

        assert_eq!(1.0, point_1.time.unwrap());
        assert_eq!(1.0, point_1.gyro_x.unwrap());
        assert_eq!(1.0, point_1.gyro_y.unwrap());
        assert_eq!(1.0, point_1.gyro_z.unwrap());
        assert_eq!(1.0, point_1.accel_x.unwrap());
        assert_eq!(1.0, point_1.accel_y.unwrap());
        assert_eq!(1.0, point_1.accel_z.unwrap());
        assert_eq!(1, time_samples);
        assert_eq!(1, gyro_x_samples);
        assert_eq!(1, gyro_y_samples);
        assert_eq!(1, gyro_z_samples);
        assert_eq!(1, accel_x_samples);
        assert_eq!(1, accel_y_samples);
        assert_eq!(1, accel_z_samples);
    }

    #[test]
    fn add_points_missing_vals_2()
    {
        let mut time_samples: usize = 0;
        let mut gyro_x_samples: usize = 0;
        let mut gyro_y_samples: usize = 0;
        let mut gyro_z_samples: usize = 0;
        let mut accel_x_samples: usize = 0;
        let mut accel_y_samples: usize = 0;
        let mut accel_z_samples: usize = 0;

        let mut point_1 = JoyconDataPoint {
            time: Some(0.0),
            gyro_x: Some(0.0),
            gyro_y: Some(0.0),
            gyro_z: Some(0.0),
            accel_x: Some(0.0),
            accel_y: Some(0.0),
            accel_z: Some(0.0),
        };

        let point_2 = JoyconDataPoint::default();

        add_points(&mut point_1, &point_2, &mut time_samples,
                   &mut gyro_x_samples, &mut gyro_y_samples,
                   &mut gyro_z_samples, &mut accel_x_samples,
                   &mut accel_y_samples, &mut accel_z_samples);

        assert_eq!(0.0, point_1.time.unwrap());
        assert_eq!(0.0, point_1.gyro_x.unwrap());
        assert_eq!(0.0, point_1.gyro_y.unwrap());
        assert_eq!(0.0, point_1.gyro_z.unwrap());
        assert_eq!(0.0, point_1.accel_x.unwrap());
        assert_eq!(0.0, point_1.accel_y.unwrap());
        assert_eq!(0.0, point_1.accel_z.unwrap());
        assert_eq!(0, time_samples);
        assert_eq!(0, gyro_x_samples);
        assert_eq!(0, gyro_y_samples);
        assert_eq!(0, gyro_z_samples);
        assert_eq!(0, accel_x_samples);
        assert_eq!(0, accel_y_samples);
        assert_eq!(0, accel_z_samples);
    }
}
