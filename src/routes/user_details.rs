use actix_web::{get , web, HttpResponse, Responder};
use tokio::sync::oneshot;
use crate::{AppState, Request, UserDetails, auth_extractor::AuthenticatedUser};
use serde::Deserialize;
use crate::hash::*;


#[get("/user_details")]
pub async fn signup(data : web::Data<AppState> , username : AuthenticatedUser) -> impl Responder {
    let (tx ,  rx) = oneshot::channel::<Result<UserDetails,String>>();
    let req = Request::UserDetails  { 
        username: username.username, 
        resp: tx 
    };
    if let Err(_) = data.worker.send(req).await {
        return HttpResponse::InternalServerError().body("Background worker creashed");
    }
    match rx.await {
        Ok(Ok(msg)) => HttpResponse::Ok().json(msg),
        Ok(Err(err)) => HttpResponse::BadRequest().body(err),
        Err(_) => HttpResponse::InternalServerError().body("No response from worker"),
    }
    
}