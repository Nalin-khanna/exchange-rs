use actix_web::{post ,web, HttpResponse, Responder};
use tokio::sync::oneshot;
use crate::{AppState, Request, auth_extractor::AuthenticatedUser};
use serde::Deserialize;
use crate::order::*;

#[derive(Deserialize)]
struct OrderPayload {
    stock_type : StockType , // Option A or Option B (yes or no)
    price : u64,
    quantity : u64,
    ordertype : Ordertype,
    market_id : String
}

#[post("/limitorder")]
pub async fn create_limit_order(data : web::Data<AppState> , payload : web::Json<OrderPayload>  , username : AuthenticatedUser ) -> impl Responder {
    let (tx , mut rx) = oneshot::channel::<Result<String,String>>();
    let req = Request::CreateLimitOrder { 
        username : username.username, 
        stock_type: payload.stock_type.clone(), 
        price: payload.price, 
        quantity:payload.quantity,
        market_id : payload.market_id.clone(),
        ordertype: payload.ordertype.clone(), 
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