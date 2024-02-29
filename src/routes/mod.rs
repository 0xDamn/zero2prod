mod health_check;
mod subscriptions;

use axum::{
    routing::{get, post},
    Router,
};

pub fn app() -> Router {
    // build our application with a single route
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub use crate::run;
pub use health_check::*;
pub use subscriptions::*;
