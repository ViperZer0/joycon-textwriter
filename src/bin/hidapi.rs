use joycon_rs::prelude::HidApi;

fn main()
{
    let api = HidApi::new().unwrap();
    for device in api.device_list()
    {
        println!("{:?}", device);
    }
}
