use utoipa::OpenApi;

use crate::{entity::rules::Model, handlers::rule::CreateRule};

use crate::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::rule::create_rule
    ), 
    components(
        schemas(Model,CreateRule),
    ),
    tags(
        (name = "rules", description = "Rule management API")
    ),
    security(
        ("BearerAuth" = [])
    )
)]

pub struct ApiDoc;
