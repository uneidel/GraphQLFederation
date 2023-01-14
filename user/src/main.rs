use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_warp::GraphQLResponse;
use warp::{http::Response as HttpResponse, Filter, Rejection, Reply};
mod model;
use model::Model::User;
use model::Model::Mutation;
use std::convert::Infallible;

#[macro_use]
extern crate lazy_static;
mod settings;
use shared;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[tokio::main]
async fn main() {

    let cfg_route = warp::path("local")
        .and(with_cfg(CONFIG.clone()))
        .and_then(cfg_handler);

    let schema = Schema::build(User, Mutation, EmptySubscription).data(CONFIG.clone()).finish();
    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("/lambda-url/user/graphql")                    
                    .finish(),
            )
    });

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<User, Mutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );
//TODO remove graphiql when ENV PROD
    let routes = graphiql
        .or(cfg_route)
        .or(graphql_post)
        .recover(shared::auth::error::handle_rejection);

    let warp_service = warp::service(routes);
    warp_lambda::run(warp_service)
        .await
        .expect("An error occured");
}

async fn cfg_handler(cfg: settings::Settings) -> Result<impl Reply, Rejection> {
    Ok(format!(
        "Running on port: {} with url: {}",
        cfg.server.port, cfg.server.url
    ))
}



fn with_cfg(
    cfg: settings::Settings,
) -> impl Filter<Extract = (settings::Settings,), Error = Infallible> + Clone {
    warp::any().map(move || cfg.clone())
}
