use exchange_rs::{
    UserDetails, models::request::Request, utils::hash_password, worker::processor::spawn_background_worker 
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

async fn new_market(
    tx: &Sender<Request>,
    user: &str,
    market_name: &str,
) -> Result<String , String> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let req = Request::CreateMarket { username: user.to_string(), market_name: market_name.to_string(), resp: resp_tx };
    tx.send(req).await.expect("Test worker send failed");
    resp_rx.await.expect("Test worker response failed")
}

async fn split_stocks (
    tx: &Sender<Request>,
    user: &str,
    market_id: &str,
    amount : u64
) -> Result<String, String> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let req = Request::SplitStocks { username: user.to_string(), market_id: market_id.to_string(), amount, resp: resp_tx };
    tx.send(req).await.expect("Test worker send failed");
    resp_rx.await.expect("Test worker response failed")
}

async fn get_user_details (
    tx : &Sender<Request>,
    username :  &str,
)-> Result<UserDetails, String>{
    let (resp_tx, resp_rx) = oneshot::channel();
    let req = Request::UserDetails { username: username.to_string(), resp: resp_tx };
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

    // second user signup 
    let res_ok = signup_user(&tx, "user2", "pass345").await;
    assert_eq!(res_ok, Ok("user2".to_string()));

    // second user signin 
    let res_correct_pass = signin_user(&tx, "user2", "pass345").await;
    assert_eq!(res_correct_pass, Ok("user2".to_string()));

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

    // check balance of user after signup 
    let res_bal = get_user_details(&tx, "user1").await;
    assert!(res_bal.is_ok(), "Getting user details failed: {:?}", res_bal.err());
    let details = res_bal.unwrap();
    assert_eq!(details.balance, 5000);

    //  Test signin with non-existent user
    let res_no_user = signin_user(&tx, "user_does_not_exist", "pass123").await;
    assert!(res_no_user.is_err());
    assert_eq!(res_no_user.unwrap_err(), "User not found");

    //test create market 
    let res_market = new_market(&tx, "user1", "market_name").await;
    assert!(res_market.is_ok());
    let market_id = res_market.unwrap();
    assert!(!market_id.is_empty());

    // test create market with non-existing user 
    let res_no_market = new_market(&tx, "user3", "market_name").await;
    assert!(res_no_market.is_err());

    // test split stocks 
    let res_split = split_stocks(&tx, "user2", &market_id, 100).await;
    assert_eq!(res_split, Ok("Minted 100 of Stock A and B".to_string()));

    // check user stock holdings after split 
    let res_bal = get_user_details(&tx, "user2").await;
    assert!(res_bal.is_ok(), "Getting user details failed: {:?}", res_bal.err());
    let details2 = res_bal.unwrap();
    assert!(details2.holdings.get(&market_id).is_some());
    let holdings = details2.holdings.get(&market_id).unwrap();
    assert!(holdings.stock_a == 100 && holdings.stock_b == 100);
    
}