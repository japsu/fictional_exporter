use juniper::{graphql_object, EmptyMutation, FieldResult};

use crate::metrics::Metric;
use crate::registry::Registry;

pub struct Context {
    pub registry: Registry,
}
impl juniper::Context for Context {}

pub struct Query;
graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str { "1.0" }

    field dummyMetric(&executor) -> FieldResult<Metric> {
        Ok(Metric::dummy())
    }

    field allMetrics(&executor) -> FieldResult<&Vec<Metric>> {
        let context = executor.context();
        Ok(&context.registry.metrics)
    }
});

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;
