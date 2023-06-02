pub trait Average {
    fn average(dataset: &[Self]) -> Self;
}

impl Average for f32
{
    fn average(dataset: &[f32]) -> f32
    {
        let sum = dataset.iter().sum();
        return sum / dataset.length();
    }
}
