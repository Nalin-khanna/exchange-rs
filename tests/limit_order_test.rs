use exchange_rs::{
    models::order::* , models::request::Request, utils::hash_password, worker::processor::spawn_background_worker 
};
use tokio::sync::{mpsc::Sender, oneshot};


async fn create_limit_order(
    tx : &Sender<Request>,
    username : &str ,
    stock_type : StockType,
    price : u64,
    quantity : u64,
    ordertype : Ordertype,
    market_id : &str
) -> Result<String, String> {
    let (resp_tx, resp_rx) = oneshot::channel::<Result<String,String>>();
    let req = Request::CreateLimitOrder { 
        username: username.to_string(), 
        stock_type: stock_type.clone(), 
        price: price, 
        quantity:quantity,
        market_id : market_id.to_string(),
        ordertype: ordertype.clone(), 
        resp: resp_tx
    };
    tx.send(req).await.expect("Test worker send failed");
    resp_rx.await.expect("Test worker response failed")
}

#[tokio::test]
async fn test_limit_order_flow() {
    let tx = spawn_background_worker();
}


