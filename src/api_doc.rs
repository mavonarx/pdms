use crate::handlers::user_handlers::*;
use crate::models::user::User;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        add_user_handler,delete_user_handler
    ),
    components(
        schemas(User, CreateUserRequest)
    ),
    tags (
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
