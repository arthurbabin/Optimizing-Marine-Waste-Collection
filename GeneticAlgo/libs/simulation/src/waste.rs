use crate::*;

#[derive(Debug)]
pub struct Waste {
    pub(crate) position: na::Point2<f32>,
}

impl Waste {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
