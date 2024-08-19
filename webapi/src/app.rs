
use axum::Router;

pub fn new() -> Router {
    Router::new()
        .merge(test::create_route())
        .merge(demo::create_route())
}

mod test {
    use axum::{Router, routing::get};
    pub(crate) fn create_route() -> Router {
        Router::new()
            .route("/test", get(test))
    }

    async fn test() ->&'static str {
        "this is a test"
    }
}

mod demo {
    use serde::{Deserialize, Serialize};
    use axum::{Router, routing::post, http::StatusCode, Json, extract};

    pub(crate) fn create_route() -> Router {
        Router::new()
            .route("/demo", post(demo))
    }

    #[derive(Deserialize)]
    struct CreateUser {
        username: String,
    }

    // the output to our `create_user` handler
    #[derive(Serialize)]
    struct User {
        id: u64,
        username: String,
    }
    async fn demo(extract::Json(payload): extract::Json<CreateUser>) -> (StatusCode, Json<User>) {
        let user = User{id: 1001, username: payload.username};

        (StatusCode::CREATED, Json(user))
    }
}

