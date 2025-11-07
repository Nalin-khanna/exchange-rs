#![allow(unused_variables , unused_mut, unused_parens , dead_code)]
use tokio::sync::{mpsc, oneshot};
use std::collections::{HashMap , BTreeMap , VecDeque};
use chrono::{DateTime, Utc};
use crate::utils::*;
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
        option : Option , // Option A or Option B (yes or no)
        price : u64,
        quantity : u64,
        ordertype : Ordertype,
        resp: oneshot::Sender<Result<String, String>>
    },
    CreateMarketOrder {
    username: String,
    option: Option,
    quantity: u64,
    ordertype: Ordertype,
    resp: oneshot::Sender<Result<String, String>>,
}
}
#[derive(Debug)]

pub struct User {
    username : String ,
    password : String , 
    balance: u64
}


pub fn spawn_background_worker () -> mpsc::Sender<(Request)>{
    let (tx , mut rx) = mpsc::channel::<(Request)>(30);
    tokio::spawn(async move {
        let mut users : HashMap<String, User> = HashMap::new();  //  Hashmap of all users
        let mut orderbooks: HashMap<Option, OrderBook> = HashMap::new();  // OrderBooks is hashmap for option a and b 's orderbook
        orderbooks.insert(Option::OptionA , OrderBook::new());
        orderbooks.insert(Option::OptionB , OrderBook::new());
        loop { 
            match rx.recv().await {
                Some(req) => {
                    match req {
                        Request::Signup { username, password,  resp } => {
                            match users.get(&username){
                                Some(user) => {
                                    let _ = resp.send(Err("Username already exists , use a different username ".to_string()));
                                }
                                 None => {
                                    // balance on signup is given = 5000 
                                    users.insert(username.clone(), User { username : username.clone(), password , balance : 5000});
                                    let _ = resp.send(Ok(username));
                                 }
                            }
                        }
                        Request::Signin { username, password, resp } => {
                            match users.get(&username) {
                                Some(user) => {
                                    if verify_password(&password, &user.password) {
                                        println!("User signed in: {}", username); 
                                        // Send Ok with the username
                                        let _ = resp.send(Ok(username));
                                    } else {
                                        let _ = resp.send(Err("Invalid password".to_string()));
                                    }
                                }
                                None => {
                                    // User not found
                                    let _ = resp.send(Err("User not found".to_string()));
                                }
                            }
                        }
                        Request::CreateLimitOrder { username, option, price, resp, quantity , ordertype } => {
                            if let Some(book) = orderbooks.get_mut(&option){
                                let mut order = Order{
                                    price,
                                    quantity,
                                    option,
                                    username,
                                    timestamp : Utc::now(),
                                    ordertype
                                };
                                let trades = book.add_limit_order(order);
                                let msg = if trades.is_empty() {
                                    "Order placed, waiting to be matched.".to_string()
                                } else{
                                    format!("{:?}", trades)
                                };
                                let _ = resp.send(Ok(msg));
                            }else{
                                let _ = resp.send(Err("Invalid option".to_string()));
                            }
                        }
                        Request::CreateMarketOrder { username, option, quantity, ordertype, resp } => {
                            if let Some(book) = orderbooks.get_mut(&option){
                                let trades = book.execute_market_order(username.clone(), ordertype, quantity);
                                let msg = if trades.is_empty() {
                                    "Order placed, waiting to be matched.".to_string()
                                } else{
                                    format!("{:?}", trades)
                                };
                                let _ = resp.send(Ok(msg));
                            } else {
                                let _ = resp.send(Err("Invalid option".to_string()));
                            }
                        }
                    }
                }
                None => break
            }
        }
    });
    tx
}

