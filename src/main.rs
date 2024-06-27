use askama::Template;
use axum::{response::Html, routing::get, Router};

#[derive(Template)] // this will generate the code...
#[template(path = "homepage.html")]
pub struct HomepageTemplate {}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    better_panic::install();

    // build our application with a route
    let app = Router::new().route("/", get(homepage));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn homepage() -> Html<String> {
    Html(HomepageTemplate {}.render().unwrap())
}
