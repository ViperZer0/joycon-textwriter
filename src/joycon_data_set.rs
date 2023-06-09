use crate::models::JoyconData;
use crate::joycon_data_point::JoyconDataPoint;
use std::fmt;

#[derive(Debug)]
pub struct JoyconDataSet {
    pub symbol: String,
    pub training_num: i32,
    pub data_points: Vec<JoyconDataPoint>,
}

impl fmt::Display for JoyconDataSet
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "Symbol: {}\n", self.symbol)?;
        write!(f, "Training Num: {}\n", self.training_num)?;
        write!(f, "Time\tGyro X\tGyro Y\tGyro Z\tAccel X\tAccel Y\tAccel Z\n")?;
        for item in self.data_points.iter()
        {
            write!(f, "{}\n", item)?;
        }
        Ok(())
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

