use actix_web::{post ,web, HttpResponse, Responder};
use tokio::sync::oneshot;
use crate::{AppState, Request};
use serde::Deserialize;
use crate::auth::*;

#[derive(Deserialize)]
pub struct SigninPayload{
    username : String, // Each username has to be unique
    password : String
}
#[derive(serde::Serialize)]
pub struct AuthResponse {
    token: String,
    msg : String
}

#[post("/signin")]
pub async fn signin (data : web::Data<AppState> , payload : web::Json<SigninPayload>) -> impl Responder {
    let (tx , rx) = oneshot::channel::<Result<String,String>>();
    let req = Request::Signin { 
        username: payload.username.clone(), 
        password: payload.password.clone(), 
        resp: tx 
    };
    if let Err(_) = data.worker.send(req).await {
        return HttpResponse::InternalServerError().body("Background worker creashed");
    }
    match rx.await {
        Ok(Ok(msg)) => {
            let token = create_jwt(&msg);
            match token {
                Ok(token) => HttpResponse::Ok().json(AuthResponse{token , msg}),
                Err(_) => HttpResponse::InternalServerError().body("Error in signing in")
            }
        },
        Ok(Err(err)) => HttpResponse::BadRequest().body(err),
        Err(_) => HttpResponse::InternalServerError().body("No response from worker"),
    }
}