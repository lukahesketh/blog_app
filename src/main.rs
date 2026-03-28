mod routes;
use routes::axum_router;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[derive(Clone)]
struct DatabaseConnectStruct {
    db: PgPool,
    jwt_private_key: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = PgPool::connect("postgres://luka@localhost/blog_app_database")
        .await
        .unwrap();

    let jwt_private_key = std::env::var("JWT_SECRET").unwrap();

    let state = DatabaseConnectStruct {
        db: pool,
        jwt_private_key,
    };

    let app = axum_router(state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
