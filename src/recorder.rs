use std::time::Instant;
use crate::joycon_data_set::JoyconDataSet;
use crate::joycon_data_point::JoyconDataPoint;
use joycon_rs::prelude::*;
use std::io;

pub struct Recorder { }

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
    pub async fn record_sample(training_num: i32) -> Result<JoyconDataSet, RecorderError>
    {
        println!("Enter the symbol to record: ");
        let mut symbol = String::new();

        // Probably shouldn't use expect here teehee
        io::stdin()
            .read_line(&mut symbol)
            .expect("Failed to read line");
        
        let (tx, rx) = std::sync::mpsc::channel();

        let manager = JoyConManager::get_instance();
    
        let output = std::thread::spawn(move || {
            println!("Press the ZR button to start recording.");
            while let Ok(message) = rx.recv()
            {
                if message.common.pushed_buttons.contains(Buttons::ZR)
                {
                    break;
                }
            }
            
            let start_time = Instant::now();
            let output_data = Vec<JoyconDataPoint>::new();
            while let Ok(message) = rx.recv() {
                // Repeat until the ZR button is released.
                let time_diff = Instant::now() - start_time;
                // data for some reason contains *3* frames of axis data.
                // Not sure we can do anything with them. Just use the first
                // frame for now.
                let data_point: JoyconDataPoint = message.extra.data[0].into();
                data_point.time = Some(time_diff);
                if !message.common.pushed_buttons.contains(Buttons::ZR)
                {
                    break;
                }
            }
            // Return the whole data set.
            JoyconDataSet {
                symbol: symbol,
                training_num: training_num,
                data_points: output_data,
            }
        });

        let devices = {
            let lock = manager.lock();
            match lock {
                Ok(manager) => manager.new_devices(),
                Err(_) => unreachable!(),
            }
        };

        devices.iter().try_for_each::<_, JoyConResult<()>>(|d| {
            let driver = SimpleJoyConDriver::new(&d)?;
            let standard_full_mode = StandardFullMode::new(driver)?;
            let tx = tx.clone();

            std::thread::spawn(move || loop {
                tx.send(standard_full_mode.read_input_report()).unwrap();
            });

            Ok(())
        })?;

        // Return the final output.
        output.join()
    }
}
