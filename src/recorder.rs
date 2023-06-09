use std::time::Instant;
use std::thread::JoinHandle;
use crate::joycon_data_set::JoyconDataSet;
use crate::joycon_data_point::{JoyconDataPoint, axisdata_to_joycon_data_point};
use joycon_rs::prelude::*;
use joycon_rs::joycon::input_report_mode::StandardInputReport;
use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use std::sync::mpsc::{Sender, Receiver};
use std::io;

pub struct Recorder {
    tx_thread: Option<JoinHandle<()>>,
    rx_thread: Option<JoinHandle<Result<JoyconDataSet, RecorderError>>>
}

#[derive(Debug)]
pub enum RecorderError {
    ReceiverThreadUninitialized,
    TransmitThreadUninitialized,
    TransmitThreadError,
    ReceiverError(crossbeam_channel::RecvError),
    JoyConError(JoyConError),
}

impl From<JoyConError> for RecorderError
{
    fn from(item: JoyConError) -> RecorderError
    {
        RecorderError::JoyConError(item)
    }
}

impl From<crossbeam_channel::RecvError> for RecorderError
{
    fn from(item: crossbeam_channel::RecvError) -> RecorderError
    {
        RecorderError::ReceiverError(item)
    }
}

impl Recorder
{
    pub fn new() -> Self
    {
        Self
        {
            tx_thread: None,
            rx_thread: None,
        }
    }

    pub fn get_sample(&mut self) -> Result<JoyconDataSet, RecorderError>
    {
        self.set_up_joycon();
        let tx_result = self.tx_thread.take();
        if let Some(tx_thread) = tx_result
        {
            if tx_thread.join().is_err()
            {
                return Err(RecorderError::TransmitThreadError);
            }
        }
        else
        {
            return Err(RecorderError::TransmitThreadUninitialized);
        }

        let result = self.rx_thread.take();
        if result.is_some()
        {
            return result.unwrap().join().unwrap();
        }
        else
        {
            return Err(RecorderError::ReceiverThreadUninitialized);
        }
    }

    // Get the symbol and the training number from the user.
    fn get_symbol() -> String
    {
        let mut symbol = String::new();
        loop
        {
            println!("Enter the symbol to train: ");
            if io::stdin().read_line(&mut symbol).is_err()
            {
                println!("Something went wrong! Try again!");
                continue;
            }
            return symbol;
        }
    }

    fn get_training_num() -> i32
    {
        return 0;
    }

    fn set_up_joycon(&mut self) -> Sender<JoyConResult<StandardInputReport<IMUData>>>
    {
        let symbol = Self::get_symbol();
        let training_num = Self::get_training_num();
        let (tx, rx) = std::sync::mpsc::channel();
        self.rx_thread = Some(std::thread::spawn(move || Self::record_sample(&symbol, training_num, rx)));
        self.set_up_transmit_thread(tx.clone());
        return tx;
    }

    fn record_sample(symbol: &str, training_num: i32, rx: Receiver<JoyConResult<StandardInputReport<IMUData>>>) -> Result<JoyconDataSet, RecorderError>
    {
        println!("Press the ZR button to start recording.");
        // Receiver error.
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

    fn set_up_transmit_thread(&mut self, tx: Sender<JoyConResult<StandardInputReport<IMUData>>>) -> Result<(),RecorderError>
    {
        let manager = JoyConManager::get_instance();
        
        let devices = {
            let lock = manager.lock();
            match lock {
                Ok(manager) => manager.new_devices(),
                Err(_) => unreachable!(),
            }
        };

        // instead of iter, we only want one device.
        let device = devices.recv()?;
        let driver = SimpleJoyConDriver::new(&device)?;
        let joycon = StandardFullMode::new(driver)?;
        self.tx_thread = Some(std::thread::spawn(move || {
            loop {
                tx.send(joycon.read_input_report()).unwrap();
            }
        }));
        Ok(())
    }
}
