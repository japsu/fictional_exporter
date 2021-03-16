use crate::metrics::Metric;

pub struct Registry {
    pub metrics: Vec<Metric>,
}

impl Registry {
    pub fn dummy() -> Self {
        Registry {
            metrics: vec![Metric::dummy()],
        }
    }

    pub fn to_prometheus(&self) -> String {
        self.metrics
            .iter()
            .map(|metric| metric.to_prometheus())
            .collect()
    }
}
