pub trait Average {
    fn average(dataset: &[Self]) -> Self where Self: Sized;
}

impl Average for f32
{
    fn average(dataset: &[f32]) -> f32
    {
        let sum: f32 = dataset.iter().sum();
        return sum / dataset.len() as f32;
    }
}
