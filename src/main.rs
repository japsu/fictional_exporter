use im::ordmap::OrdMap;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
enum MetricType {
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

#[derive(Serialize, Deserialize, Debug)]
enum MetricState {
    Stable,
    Meandering { target_value: f64 },
}

#[derive(Serialize, Deserialize, Debug)]
struct Metric<'a> {
    kind: MetricType,
    name: &'a str,
    help: &'a str,
    labels: OrdMap<String, String>,
    state: MetricState,
    value: f64,
}

impl Metric<'_> {
    fn to_prometheus(&self) -> String {
        return format!(
            "# HELP {help}\n# TYPE {name} {kind}\n{name}{{{formatted_labels}}} {value}",
            name = self.name,
            help = self.help,
            kind = self.kind,
            formatted_labels = "foo = \"bar\"",
            value = self.value
        );
    }
}

fn main() {
    let metric = Metric {
        name: "foo",
        help: "It's a foo!",
        kind: MetricType::Gauge,
        labels: OrdMap::new(),
        state: MetricState::Meandering {
            target_value: 700.0,
        },
        value: 700.0,
    };
    println!("{}", serde_json::to_string(&metric).unwrap());
    println!("{}", metric.to_prometheus());
}
