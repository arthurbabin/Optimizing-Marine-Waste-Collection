use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        eye: &Eye,
    ) -> Self {
        Self {
            nn: nn::Network::from_weights(
                &Self::topology(eye),
                chromosome,
            ),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            // The Input Layer
            nn::LayerTopology {
                neurons: eye.cells(),
            },

            // The Hidden Layer
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },

            // The Output Layer to control both speed and rotation
            // of the collector
            nn::LayerTopology { neurons: 2 },
        ]
    }


}
