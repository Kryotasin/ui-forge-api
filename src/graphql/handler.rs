use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::db::mongo::MongoDb;
use crate::graphql::AppSchema;

pub async fn graphql_handler(
    schema: web::Data<AppSchema>,
    db: web::Data<MongoDb>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut query = req.into_inner();
    query = query.data(db.get_ref().clone());
    schema.execute(query).await.into()
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/api/graphql"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}