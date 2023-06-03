use std::time::Instant;
pub struct Recorder { }

pub enum RecorderError {}

impl Recorder
{
    pub async fn record_sample() -> Result<JoyconDataSet, RecorderError>
    {
        let (tx, rx) = std::sync::mpsc::channel();


        let manager = JoyConManager::get_instance();
    
        let output = std::thread::spawn(move || {
            let output_data = Vec<JoyconDataPoint>::new();
            while let Ok(message) = rx.recv() {
                // Repeat until the ZR button is released.
                if !message.common.pushed_buttons.contains(Buttons::ZR)
                {
                    break;
                }
                
                

            }

        let devices = {
            let lock = manager.lock();
            match lock {
                Ok(manager) => manager.new_devices(),
                Err(_) => unreachable!(),
            }
        };

        devices.iter().try_for_each::_, JoyConResult<()>>)(|d| {
            let driver = SimpleJoyConDriver::new(&d)?;
            let standard_full_mode = StandardFullMode::new(driver)?;
            let tx = tx.clone();

            std::thread::spawn(move || loop {
                tx.send(standard_full_mode.read_input_report()).unwrap();
            });

            Ok(())
        })?;
    }
}

impl From<AxisData> for JoyconDataPoint
{
    fn from(item: AxisData) -> JoyconDataPoint
    {
        JoyconDataPoint {
            gyro_x: Some(item.gyro_1),
            gyro_y: Some(item.gyro_2),
            gyro_z: Some(item.gyro_3),
            accel_x: Some(item.accel_x),
            accel_y: Some(item.accel_y),
            accel_z: Some(item.accel_z),
        }
    }
}
