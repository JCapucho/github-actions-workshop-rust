use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(root, create_user), components(schemas(User, CreateUser)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// basic handler that responds with a static string
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Returns a friendly greeting", body = String),
    ),
)]
async fn root() -> &'static str {
    "Hello, World!"
}

/// Create new user
///
/// Creates a new user in the database
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = CREATED, description = "The user was created", body = User),
    ),
)]
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

/// the input to our `create_user` handler
#[derive(Deserialize, ToSchema)]
struct CreateUser {
    username: String,
}

/// the output to our `create_user` handler
#[derive(Serialize, ToSchema)]
struct User {
    id: u64,
    username: String,
}
