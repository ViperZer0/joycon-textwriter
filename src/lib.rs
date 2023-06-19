//use diesel::query_builder::{DeleteStatement, IncompleteInsertStatement};

pub mod models;
pub mod schema;
pub mod joycon_data_set;
pub mod joycon_data_point;
pub mod fraction;
pub mod average;
pub mod recorder;
pub mod resample;
pub mod database_connection;
pub mod controller;
pub mod neural_network;
/*
pub struct JoyconRecorder
{
}

impl JoyconRecorder
{
    pub async fn record_dataset(symbol: &str, training_num: i32) -> JoyconDataSet
    {
        let manager = JoyconManager::get_instance();

        let devices = {
            let lock = manager.lock();
            match lock {
                Ok(manager) => manager.new_devices();
                Err(_) => unreachable!();
            }
        };

        let mut data_points: Vec<JoyconDataPoint> = vec!();
        

        todo!();
    }
*/
