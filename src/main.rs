use actix_web::{ web, App,  HttpServer};
use exchange_rs::get_orderbook::get_orderbook;
use exchange_rs::processor::*;
use exchange_rs::routes::*;
use dotenvy;
use exchange_rs::AppState;
use exchange_rs::user_details::user_details;
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let worker = spawn_background_worker();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(AppState{worker : worker.clone()}))
            .service(signup)
            .service(signin)
            .service(create_limit_order)
            .service(create_market_order)
            .service(split_stocks)
            .service(merge)
            .service(get_orderbook)
            .service(user_details)
            .service(create_market)
    })
    .bind(bind_addr)?
    .run()
    .await
}