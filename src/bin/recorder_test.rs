use crate::recorder::Recorder;

#[tokio::main]
async fn main()
{
    let recording = Recorder::record_sample(0).await;
    println!("{:?}", recording.expect("This better not throw an error!!!");
}
