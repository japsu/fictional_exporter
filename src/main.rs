mod graphql;
mod metrics;
mod registry;

use juniper::{EmptyMutation, Variables};

use graphql::{Context, Query, Schema};
use registry::Registry;

fn main() {
    let context = Context {
        registry: Registry::dummy(),
    };

    let (res, _errors) = juniper::execute(
        "query { allMetrics { name, labels { key, value } } }",
        None,
        &Schema::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &context,
    )
    .unwrap();

    println!("{:?}", res.as_object_value().unwrap())
}
