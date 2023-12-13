use dotenv::dotenv;
use crate::server;
use sqlx;

pub async fn start() {
    // 加载环境变量
    dotenv().ok();

    let _guard = server::log::init();

    let pg = server::db::postgres_connet()
    .await
    .expect("Database connection failed");

    let redis = server::cache::redis_connect()
    .await
    .expect("Redis connection failed");

    sqlx::migrate!()
    .run(&pg)
    .await
    .expect("Migrate failed");

    let app = server::route::init();
    
    let addr = "0.0.0.0:3000";
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Auxmgo ok ! Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}