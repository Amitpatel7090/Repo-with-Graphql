
use rocket::{response::content, routes, State};
use async_graphql::{http::playground_source};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse, GraphQLQuery};
use async_graphql::{EmptySubscription, Schema};
use rocket_contrib::json;
use reqwest::{Client, RequestBuilder};



use handler::{Query, Mutation, ProjectSchema};
mod handler;


#[rocket::get("/")]
pub async fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_handler(
    schema: &State<ProjectSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::launch]
fn rocket() -> _ {
    let schema = handler::ProjectSchema::build(Query, Mutation, async_graphql::EmptySubscription)
        .finish();

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_handler, graphql_playground])
}
