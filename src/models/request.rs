use crate::order::*;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Request {
    Signup {
        username: String,
        password : String,
        resp: oneshot::Sender<Result<String, String>>
    },
    Signin {
        username: String,
        password : String,
        resp: oneshot::Sender<Result<String, String>>
    },
    CreateLimitOrder{
        username : String,
        stock_type : StockType , // Option A or Option B (yes or no)
        price : u64,
        quantity : u64,
        ordertype : Ordertype,
        market_id : String, 
        resp: oneshot::Sender<Result<String, String>>
    },
    CreateMarketOrder {
    username: String,
    stock_type: StockType,
    quantity: u64,
    ordertype: Ordertype,
    market_id : String,
    resp: oneshot::Sender<Result<String, String>>,
    },
    CreateMarket{
        username : String,
        market_name : String,
        resp: oneshot::Sender<Result<String, String>>
    }
}
