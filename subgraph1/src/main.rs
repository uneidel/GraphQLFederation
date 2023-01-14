use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use warp::{http::Response as HttpResponse, Filter, Rejection, Reply};
mod model;
use async_graphql_warp::GraphQLResponse;
use model::PCNModel::Query;
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

    
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("/lambda-url/subgraph1/graphql")      // NOTE this will not work deployed on lambda              
                    .finish(),
            )
    });

    let graphql_post_auth = warp::header::optional::<String>("token")
        .and(shared::auth::with_auth(            
            CONFIG.to_owned().security.requiredrole,
            CONFIG.to_owned().security.pk,
        ))
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(with_context);

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let cgraphiql = shared::warp::enable(CONFIG.security.playground).and(graphiql);
    let graphauth = shared::warp::enable(CONFIG.security.jwt_enabled).and(graphql_post_auth);
    let graph = shared::warp::enable(!CONFIG.security.jwt_enabled).and(graphql_post);

    let routes = cgraphiql
        .or(graphauth)
        .or(graph)
        .recover(shared::auth::error::handle_rejection);

    let warp_service = warp::service(routes);
    warp_lambda::run(warp_service)
        .await
        .expect("An error occured");
}

async fn with_context(
    option: Option<String>,
    claims: shared::auth::Claims,
    (schema, request): (
        Schema<Query, EmptyMutation, EmptySubscription>,
        async_graphql::Request,
    ),
) -> Result<GraphQLResponse, Infallible> {
    println!("Claims : {:?}", claims);
    // Do Customer Specific
    // let request = request.data(sub).data(user);
    let response = schema.execute(request).await;
    Ok::<_, Infallible>(GraphQLResponse::from(response))
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
