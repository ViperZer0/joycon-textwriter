use crate::average::Average;
use crate::models::JoyconData;
use joycon_rs::joycon::input_report_mode::standard_full_mode::AxisData;

#[derive(Default, Debug)]
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
    pub fn label(&self, symbol: &str, training_num: i32, sample_num: i32) -> JoyconData 
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

    // Add two points while checking for None values for each struct.
    // Only add to the number of samples if a Some value is found.
    pub fn add_point(&mut self, other_point: &JoyconDataPoint,
              time_samples: &mut usize, gyro_x_samples: &mut usize,
              gyro_y_samples: &mut usize, gyro_z_samples: &mut usize,
              accel_x_samples: &mut usize, accel_y_samples: &mut usize,
              accel_z_samples: &mut usize)
    {
        if let Some(val) = other_point.time
        {
            // This should default to 0 if the self value is None.
            self.time = Some(self.time.unwrap_or_default() + val);
            *time_samples += 1;
        }
        if let Some(val) = other_point.gyro_x
        {
            self.gyro_x = Some(self.gyro_x.unwrap_or_default() + val);
            *gyro_x_samples += 1;
        }
        if let Some(val) = other_point.gyro_y
        {
            self.gyro_y = Some(self.gyro_y.unwrap_or_default() + val);
            *gyro_y_samples += 1;
        }
        if let Some(val) = other_point.gyro_z
        {
            self.gyro_z = Some(self.gyro_z.unwrap_or_default() + val);
            *gyro_z_samples += 1;
        }
        if let Some(val) = other_point.accel_x
        {
            self.accel_x = Some(self.accel_x.unwrap_or_default() + val);
            *accel_x_samples += 1;
        }
        if let Some(val) = other_point.accel_y
        {
            self.accel_y = Some(self.accel_y.unwrap_or_default() + val);
            *accel_y_samples += 1;
        }
        if let Some(val) = other_point.accel_z
        {
            self.accel_z = Some(self.accel_z.unwrap_or_default() + val);
            *accel_z_samples += 1;
        }
    }
}


// A special converter that takes an extra argument
pub fn axisdata_to_joycon_data_point(item: &AxisData, time_elapsed: f32) -> JoyconDataPoint
{
    JoyconDataPoint
    {
        time: Some(time_elapsed),
        gyro_x: Some(item.gyro_1.into()),
        gyro_y: Some(item.gyro_2.into()),
        gyro_z: Some(item.gyro_3.into()),
        accel_x: Some(item.accel_x.into()),
        accel_y: Some(item.accel_y.into()),
        accel_z: Some(item.accel_z.into()),
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
        dataset.iter().for_each(|x| sum.add_point(&x, &mut time_samples,
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

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_add_point_normal() {
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

        point_1.add_point(&point_2, &mut time_samples,
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
    fn add_point_missing_vals()
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

        point_1.add_point(&point_2, &mut time_samples,
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
    fn add_point_missing_vals_2()
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

        point_1.add_point(&point_2, &mut time_samples,
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
