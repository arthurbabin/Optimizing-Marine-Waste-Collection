use crate::*;

#[derive(Debug)]
pub struct Sea{
    pub(crate) collectors: Vec<Collector>,
    pub(crate) wastes: Vec<Waste>,
}

impl Sea {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let collectors = (0..40)
            .map(|_| Collector::random(rng))
            .collect();

        let wastes = (0..60)
            .map(|_| Waste::random(rng))
            .collect();

        Self { collectors, wastes}
    }

    pub fn collectors(&self) -> &[Collector] {
        &self.collectors
    }

    pub fn wastes(&self) -> &[Waste] {
        &self.wastes
    }
}
