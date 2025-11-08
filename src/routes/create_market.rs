use actix_web::{post ,web, HttpResponse, Responder};
use tokio::sync::oneshot;
use crate::{AppState, Request};
use serde::Deserialize;
use crate::order::*;

#[derive(Deserialize)]
struct CreateMarketPayload {
    username : String, 
    market_name : String
}

#[post("/create_market")]
pub async fn create_market(data : web::Data<AppState> , payload : web::Json<CreateMarketPayload>) -> impl Responder {
    let (tx , mut rx) = oneshot::channel::<Result<String,String>>();
    let req = Request::CreateMarket { 
        username: payload.username.clone(), 
        market_name : payload.market_name.clone(),
        resp: tx
    };
    if let Err(_) = data.worker.send(req).await {
        return HttpResponse::InternalServerError().body("Background worker creashed");
    }
    match rx.await {
        Ok(Ok(msg)) => HttpResponse::Ok().body(msg),
        Ok(Err(err)) => HttpResponse::BadRequest().body(err),
        Err(_) => HttpResponse::InternalServerError().body("No response from worker"),
    }
}