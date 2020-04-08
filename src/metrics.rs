use itertools::Itertools;
use juniper::{GraphQLEnum, GraphQLObject};
use std::fmt;

#[derive(GraphQLEnum, Debug)]
pub enum MetricType {
    Counter,
    Gauge,
    // TODO: Histogram,
    // TODO: Summary,
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
            }
        )
    }
}

#[derive(GraphQLObject, Debug)]
pub struct Label {
    pub key: String,
    pub value: String,
}

#[derive(GraphQLObject, Debug)]
pub struct Metric {
    pub kind: MetricType,
    pub name: String,
    pub help: String,
    pub labels: Vec<Label>,
    pub value: f64,
}

impl Metric {
    pub fn to_prometheus(&self) -> String {
        let mut result = String::new();

        if !self.help.is_empty() {
            result.push_str(&format!("# HELP {help}\n", help = self.help));
        }

        result.push_str(&format!(
            "# TYPE {name} {kind}\n{name}{{{formatted_labels}}} {value}\n",
            name = self.name,
            kind = self.kind,
            formatted_labels = self.formatted_labels(),
            value = self.value
        ));

        result
    }

    fn formatted_labels(&self) -> String {
        self.labels
            .iter()
            .map(|label| format!("{key}=\"{value}\"", key = label.key, value = label.value))
            .join(",")
    }

    pub fn dummy() -> Self {
        Self {
            name: String::from("foo"),
            help: String::from("It's a foo!"),
            kind: MetricType::Gauge,
            value: 700.0,
            labels: vec![Label {
                key: String::from("foo"),
                value: String::from("bar"),
            }],
        }
    }
}
