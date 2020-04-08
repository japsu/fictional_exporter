use crate::metrics::Metric;

pub struct Registry {
    metrics: Vec<Metric>,
}

impl Registry {
    pub fn dummy() -> Self {
        Registry {
            metrics: vec![Metric::dummy()],
        }
    }
}
