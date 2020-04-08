use juniper::{EmptyMutation, Variables};

mod graphql;
mod metrics;
mod registry;
use graphql::{Context, Query, Schema};

fn main() {
    let context = Context;

    let (res, _errors) = juniper::execute(
        "query { dummyMetric { name, value } }",
        None,
        &Schema::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &context,
    )
    .unwrap();

    println!("{:?}", res.as_object_value().unwrap())
}
