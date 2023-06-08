use crate::models::JoyconData;
use crate::joycon_data_point::JoyconDataPoint;

#[derive(Debug)]
pub struct JoyconDataSet {
    pub symbol: String,
    pub training_num: i32,
    pub data_points: Vec<JoyconDataPoint>,
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

