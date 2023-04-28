use diesel::prelude::*;

#[derive(Queryable)]
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

#[derive(Insertable)]
#[diesel(table_name = crate::schema::joycon_data)]
//pub struct NewJoyconData<'a> {
pub struct NewJoyconData {
    //pub symbol: &'a str,
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
/*
impl<'a> NewJoyconData<'a>{
    pub fn new(symbol: &str, training_num: i32) -> NewJoyconData
    */
impl NewJoyconData {
    pub fn new(symbol: &str, training_num: i32) -> NewJoyconData {
        NewJoyconData {
            symbol: symbol.to_string(),
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
}
