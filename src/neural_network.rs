use neuronika::nn;
use neuronika::optim;

struct NeuralNetwork {
    lin1: nn::Linear,
    lin2: nn::Linear,
    lin3: nn::Linear,
    status: nn::ModelStatus,
}

impl NeuralNetwork {
    fn new(input_features: usize, output_features: usize) -> Self {
        let mut status = ModelStatus::default();
        Self {
            // (input features, output features)
            // We essentially want this to be 
            // number of data points per symbol * number of measurements per data points per data
            // point (7?)
            // Output features is number of symbols.

            lin1: status.register(nn::Linear::new(input_features, 100)),
            lin2: status.register(nn::Linear::new(100,100)),
            lin3: status.register(nn::Linear::new(100, output_features)),
            status,
        }
    }

    fn parameters(&self) -> Vec<Param> {
        self.status.parameters()
    }

    fn forward<I, T, U>(&self, input: I) -> VarDiff<impl Data<Dim=Ix2> + Forward, impl Gradient<Dim = Ix2> + Overwrite + Backward>
        where 
            I: MatMatMuT<Learnable<Ix2>>,
            I::Output: Into<VarDiff<T, U>>,
            T: Data<Dim = Ix2> + Forward,
            U: Gradient<Dim = Ix2> + Backward + Overwrite,
        {
            let out1 = self.lin1.forward(input).relu();
            let out2 = self.lin2.forward(out1).relu();
            let out3 = self.lin3.forward(out2);
            out3
        }
}

struct Optimizer {
    optimizer: optim::SGD,
}

impl Optimizer
{
    fn new(parameters: Vec<Param>, learning_rate: f32) -> Self
    {
        Self
        {
            optimizer: optim::SGD::new(parameters, learning_rate, optim::L2::new(0.0))
        }
    }
}
