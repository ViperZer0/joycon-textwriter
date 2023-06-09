use std::time::Instant;
use std::thread::JoinHandle;
use crate::joycon_data_point::{JoyconDataPoint, axisdata_to_joycon_data_point};
use joycon_rs::prelude::*;
use joycon_rs::joycon::input_report_mode::StandardInputReport;
use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use std::sync::mpsc::{Sender, Receiver};
pub struct Recorder {
    rx_thread: Option<JoinHandle<Result<Vec<JoyconDataPoint>, RecorderError>>>,
    tx_thread: Option<JoinHandle<()>>
}

#[derive(Debug)]
pub enum RecorderError {
    ReceiverThreadUninitialized,
    TransmitThreadUninitialized,
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
            rx_thread: None,
            tx_thread: None,
        }
    }

    pub fn get_sample(&mut self) -> Result<Vec<JoyconDataPoint>, RecorderError>
    {
        self.set_up_joycon();
        let result = self.rx_thread.take();
        if result.is_some()
        {
            let result = result.unwrap().join();
            // Close down tx_thread.
            let tx_thread = self.tx_thread.take();
            if tx_thread.is_some()
            {
                tx_thread.unwrap().join();
            }
            else
            {
                return Err(RecorderError::TransmitThreadUninitialized);
            }

            // Check result
            if result.is_err()
            {
                panic!("Something went wrong joining the recieve thread");
            }
            return result.unwrap();
        }
        else
        {
            return Err(RecorderError::ReceiverThreadUninitialized);
        }
    }

    fn set_up_joycon(&mut self)
    {
        let (tx, rx) = std::sync::mpsc::channel();
        self.set_up_transmit_thread(tx).expect("Something went wrong in setting up the transmit thread!");
        self.rx_thread = Some(std::thread::spawn(move || Self::record_sample(rx)));
    }

    fn record_sample(rx: Receiver<JoyConResult<StandardInputReport<IMUData>>>) -> Result<Vec<JoyconDataPoint>, RecorderError>
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
        Ok(output_data)
    }

    fn set_up_transmit_thread(&mut self, tx: Sender<JoyConResult<StandardInputReport<IMUData>>>) -> Result<(),RecorderError>
    {
        let manager = JoyConManager::get_instance();

        let mut devices = {
            let lock = manager.lock();
            match lock {
                Ok(manager) => manager.managed_devices(),
                Err(_) => unreachable!()
            }
        };

        

        // We just want one device. We'll use pop here.
        let device = devices.pop();
        if device.is_none()
        {
            panic!("No JoyConDevices found!");
        }
        let device = SimpleJoyConDriver::new(&device.unwrap())?;
        let device = StandardFullMode::new(device)?;
        self.tx_thread = Some(std::thread::spawn(move || {
            loop {
                let result = tx.send(device.read_input_report());
                if result.is_err()
                {
                    break;
                }
            }
        }));
        Ok(())
   }
}
