mod graphql;
mod metrics;
mod registry;

use juniper::EmptyMutation;
use warp::Filter;

use graphql::{Context, Query, Schema};
use registry::Registry;

#[tokio::main]
async fn main() {
    ::std::env::set_var("RUST_LOG", "fictional_exporter");
    env_logger::init();

    let log = warp::log("fictional_exporter");

    // log::info!("Listening on 127.0.0.1:8080");

    let state = warp::any().map(move || Context {
        registry: Registry::dummy(),
    });

    let graphql = warp::path("graphql").and(juniper_warp::make_graphql_filter(
        Schema::new(Query, EmptyMutation::<Context>::new()),
        state.boxed(),
    ));

    let graphiql = warp::get2()
        .and(warp::path("graphiql"))
        .and(juniper_warp::graphiql_filter("/graphql"));

    let metrics = warp::get2()
        .and(warp::path("metrics"))
        .and(state)
        .map(|state: Context| state.registry.to_prometheus());

    warp::serve(graphiql.or(graphql).or(metrics).with(log)).run(([127, 0, 0, 1], 8080))
}
