use std::time::Instant;
use std::thread::Thread;
use crate::joycon_data_set::JoyconDataSet;
use crate::joycon_data_point::{JoyconDataPoint, axisdata_to_joycon_data_point};
use joycon_rs::prelude::*;
use joycon_rs::joycon::input_report_mode::StandardInputReport;
use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use std::sync::mpsc::{Sender, Receiver};

pub struct Recorder {
    sender_thread: Thread,
    receiver_thread: Thread,
}

pub enum RecorderError {
    JoyConError(JoyConError),
}

impl From<JoyConError> for RecorderError
{
    fn from(item: JoyConError) -> RecorderError
    {
        RecorderError::JoyConError(item)
    }
}

impl Recorder
{
    async fn set_up_joycon(&mut self) -> (Sender<JoyConResult<StandardInputReport<IMUData>>>, Receiver<JoyConResult<StandardInputReport<IMUData>>>)
    {
        
    }

    async fn record_sample(symbol: &str, training_num: i32, rx: Receiver<JoyConResult<StandardInputReport<IMUData>>>) -> Result<JoyconDataSet, RecorderError>
    {
        println!("Press the ZR button to start recording.");
        // Reciever error.
        while let Ok(message) = rx.recv()
        {
            if message.is_err()
            {
                return Err(message.unwrap_err().into());
            }

            if message.unwrap().common.pushed_buttons.contains(Buttons::ZR)
            {
                break;
            }
        }
            
        let start_time = Instant::now();
        let mut output_data: Vec<JoyconDataPoint> = Vec::new();
        while let Ok(message) = rx.recv() {
            if message.is_err()
            {
                return Err(message.unwrap_err().into());
            }

            let input_report = message.unwrap();
            // Repeat until the ZR button is released.
            let time_diff = Instant::now() - start_time;
            // data for some reason contains *3* frames of axis data.
            // Not sure we can do anything with them. Just use the first
            // frame for now.
            // May have to mess with some data types.
            let data_point = axisdata_to_joycon_data_point(&input_report.extra.data[0], time_diff.as_millis() as f32);
            output_data.push(data_point);
            if !input_report.common.pushed_buttons.contains(Buttons::ZR)
            {
                break;
            }
        }
        // Return the whole data set.
        Ok(JoyconDataSet {
            symbol: symbol.to_owned(),
            training_num: training_num,
            data_points: output_data,
        })
    }
}
