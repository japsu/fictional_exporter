use juniper::{graphql_object, EmptyMutation, FieldResult};

use crate::metrics::Metric;

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;
graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str { "1.0" }

    field dummyMetric(&executor) -> FieldResult<Metric> {
        Ok(Metric::dummy())
    }
});

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;
