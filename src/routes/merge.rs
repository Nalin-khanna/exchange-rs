use actix_web::{post ,web, HttpResponse, Responder};
use tokio::sync::oneshot;
use crate::{AppState, Request, auth_extractor::AuthenticatedUser};
use serde::Deserialize;

#[derive(Deserialize)]
struct Merge {
    market_id: String,
    amount: u64,
}

#[post("/merge")]
pub async fn merge(data : web::Data<AppState> , payload : web::Json<Merge> , username : AuthenticatedUser) -> impl Responder {
    let (tx , rx) = oneshot::channel::<Result<String,String>>();
    let req = Request::MergeStocks { 
        username: username.username, 
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