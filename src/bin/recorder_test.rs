use joycon_typer::recorder::Recorder;

fn main()
{
    let recording = Recorder::new().get_sample();
    println!("{:?}", recording.expect("This better not throw an error!!!"));
}
