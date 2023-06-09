use joycon_typer::recorder::Recorder;
use joycon_typer::resample;
use joycon_typer::joycon_data_set::JoyconDataSet;
fn main()
{
    let recording = Recorder::new().get_sample();
    let result = recording.expect("This better not throw an error!!!");

    println!("{}", result);
    println!("Resampling to 10 data points >:)");
    let result2 = JoyconDataSet{
        data_points: resample::resample(&(result.data_points), 10).unwrap(),
        ..result
    };
    println!("{}", result2);
}
