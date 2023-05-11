use diesel::prelude::*;
use crate::schema::joycon_data;

// This is the actual data that is inserted into the database. Each element has the symbol,
// training number, and (oops it should also have 
#[derive(Queryable,Insertable)]
#[diesel(table_name = joycon_data)]
pub struct JoyconData {
    pub symbol: String,
    pub training_num: i32,
    pub time: Option<f32>,
    pub gyro_x: Option<f32>,
    pub gyro_y: Option<f32>,
    pub gyro_z: Option<f32>,
    pub accel_x: Option<f32>,
    pub accel_y: Option<f32>,
    pub accel_z: Option<f32>,
}

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
    fn label(&self, symbol: &str, training_num: i32) -> JoyconData 
    {
        JoyconData {
            symbol: symbol.to_owned(),
            training_num: training_num,
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

pub struct JoyconDataSet {
    pub symbol: String,
    pub training_num: i32,
    pub data_points: Vec<JoyconDataPoint>,
}

impl From<JoyconDataSet> for Vec<JoyconData>
{
    fn from(item: JoyconDataSet) -> Self
    {
        item.data_points.iter().map(|x| x.label(&item.symbol, item.training_num)).collect()
    }
}
/*
use crate::schema::joycon_data;
#[derive(Insertable)]
#[diesel(table_name = joycon_data)]
pub struct NewJoyconData<'a> {
    pub symbol: &'a str,
    pub training_num: i32,
    pub time: Option<f32>,
    pub gyro_x: Option<f32>,
    pub gyro_y: Option<f32>,
    pub gyro_z: Option<f32>,
    pub accel_x: Option<f32>,
    pub accel_y: Option<f32>,
    pub accel_z: Option<f32>,
}

impl<'a> NewJoyconData<'a>{
    pub fn new(symbol: &str, training_num: i32) -> NewJoyconData
    {
        NewJoyconData {
            symbol,
            training_num,
            time: None,
            gyro_x: None,
            gyro_y: None,
            gyro_z: None,
            accel_x: None,
            accel_y: None,
            accel_z: None,
        }
    }
}*/
