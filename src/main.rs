use actix_web::{ web, App,  HttpServer};
use exchange_rs::processor::*;
use exchange_rs::routes::*;


use exchange_rs::AppState;
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let worker = spawn_background_worker();
    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(AppState{worker : worker.clone()}))
            .service(signup)
            .service(signin)
            .service(create_limit_order)
            .service(create_market_order)
            .service(split_stocks)
            .service(merge)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}