use rand::{Rng,RngCore};

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
struct Layer{
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) ->  Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer | layer.propagate(inputs))
    }

    pub fn random(
        rng: &mut dyn rand::RngCore,
        layers: &[LayerTopology]
        ) ->  Self {
        assert!(layers.len() > 1); //Network with more than 1 layer

        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(rng,layers[0].neurons, layers[1].neurons)
            })
        .collect();

        Self { layers}
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        use std::iter::once;

        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::from_weights(
                    layers[0].neurons,
                    layers[1].neurons,
                    &mut weights,
                )
            })
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }}
}

impl Layer {
    fn new(neurons: Vec<Neuron>) -> Self {
        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) ->  Vec<f32> {
        self.neurons
            .iter()
            .map(|neurons| neurons.propagate(&inputs))
            .collect()
    }

    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize
        ) ->  Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng,input_neurons))
            .collect();

        Self { neurons }
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

impl Neuron {
    fn new(bias: f32, weights: Vec<f32>) -> Self {
        Self { bias, weights }
    }
    pub fn propagate(&self, inputs: &[f32]) ->  f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input,weight)| input * weight)
            .sum::<f32>(); //turbofish to allow to provide an explicit argument

        (self.bias + output).max(0.0)
    }

    pub fn random(
        rng: &mut dyn rand::RngCore,
        output_size: usize
        ) ->  Self {
        let bias = rng.gen_range(-1.0..=1.0); //beware of the closed interval

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    pub fn from_weights(
        output_neurons: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test_neurons_weights_bias() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            approx::assert_relative_eq!(neuron.bias, -0.6255188);
            approx::assert_relative_eq!(neuron.weights.as_slice(), [
                0.67383957, 
                0.8181262, 
                0.26284897, 
                0.5238807
            ].as_ref());
        }
    }

    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
            ]);

            let actual = network.weights();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(
                actual.as_slice(),
                expected.as_slice(),
            );
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn test() {
            let layers = &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
            ];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights();

            approx::assert_relative_eq!(
                actual.as_slice(),
                weights.as_slice(),
            );
        }
    }
}
