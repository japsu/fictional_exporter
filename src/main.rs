mod metrics;
use metrics::{Label, Metric, MetricType};

fn main() {
    let metric = Metric {
        name: String::from("foo"),
        help: String::from("It's a foo!"),
        kind: MetricType::Gauge,
        value: 700.0,
        labels: vec![Label {
            key: String::from("foo"),
            value: String::from("bar"),
        }],
    };
    println!("{}", metric.to_prometheus());
}
