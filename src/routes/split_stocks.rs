use actix_web::{post ,web, HttpResponse, Responder};
use tokio::sync::oneshot;
use crate::{AppState, Request};
use serde::Deserialize;
use crate::order::*;

#[derive(Deserialize)]
struct SplitStocks {
    username: String,
    market_id: String,
    amount: u64,
}

#[post("/split_stocks")]
pub async fn split_stocks(data : web::Data<AppState> , payload : web::Json<SplitStocks>) -> impl Responder {
    let (tx , mut rx) = oneshot::channel::<Result<String,String>>();
    let req = Request::SplitStocks  { 
        username: payload.username.clone(), 
        market_id : payload.market_id.clone(),
        amount : payload.amount,
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