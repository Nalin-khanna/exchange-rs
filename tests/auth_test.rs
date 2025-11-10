use exchange_rs::{
    models::request::Request,
    worker::processor::spawn_background_worker,
    utils::{hash_password}, 
};
use tokio::sync::{mpsc::Sender, oneshot};

async fn signup_user(
    tx: &Sender<Request>,
    user: &str,
    pass: &str,
) -> Result<String, String> {
    let (resp_tx, resp_rx) = oneshot::channel();
    // Mimic the route: hash the password before sending
    let hashed_pass = hash_password(pass);

    let req = Request::Signup {
        username: user.to_string(),
        password: hashed_pass,
        resp: resp_tx,
    };
    tx.send(req).await.expect("Test worker send failed");
    resp_rx.await.expect("Test worker response failed")
}

async fn signin_user(
    tx: &Sender<Request>,
    user: &str,
    pass: &str,
) -> Result<String, String> {
    let (resp_tx, resp_rx) = oneshot::channel();
    
    let req = Request::Signin {
        username: user.to_string(),
        password: pass.to_string(), // Send plaintext, as per your route
        resp: resp_tx,
    };

    tx.send(req).await.expect("Test worker send failed");
    resp_rx.await.expect("Test worker response failed")
}

#[tokio::test]
async fn test_auth_flow() {
    //  Start the worker
    let tx = spawn_background_worker();

    //  Test successful signup
    let res_ok = signup_user(&tx, "user1", "pass123").await;
    assert_eq!(res_ok, Ok("user1".to_string()));

    //  Test duplicate signup
    let res_dup = signup_user(&tx, "user1", "pass456").await;
    assert!(res_dup.is_err());
    assert!(res_dup
        .unwrap_err()
        .contains("Username already exists"));

    //  Test signin with wrong password
    let res_wrong_pass = signin_user(&tx, "user1", "wrongpass").await;
    assert!(res_wrong_pass.is_err());
    assert_eq!(res_wrong_pass.unwrap_err(), "Invalid password");

    //  Test signin with correct password
    let res_correct_pass = signin_user(&tx, "user1", "pass123").await;
    assert_eq!(res_correct_pass, Ok("user1".to_string()));

    //  Test signin with non-existent user
    let res_no_user = signin_user(&tx, "user_does_not_exist", "pass123").await;
    assert!(res_no_user.is_err());
    assert_eq!(res_no_user.unwrap_err(), "User not found");
}