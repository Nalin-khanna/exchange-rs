#![allow(unused_variables , unused_mut, unused_parens , dead_code)]
use tokio::sync::{mpsc, oneshot};
use std::collections::{HashMap };
use chrono::{Utc};
use crate::utils::*;
use crate::models::*;

pub fn spawn_background_worker () -> mpsc::Sender<(Request)>{
    let (tx , mut rx) = mpsc::channel::<(Request)>(30);
    tokio::spawn(async move {
        let mut users : HashMap<String, User> = HashMap::new();  //  Hashmap of all users
        let mut markets : HashMap<String , Market> = HashMap::new();
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
                                    users.insert(username.clone(), User { username : username.clone(), password , balance : 5000 , holdings : HashMap::new()});
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
                        Request::CreateLimitOrder { username, option, price, resp, quantity , ordertype , market_id} => {

                            if let Some(market) = markets.get_mut(&market_id){

                                let mut order = Order{
                                    price,
                                    quantity,
                                    option,
                                    username,
                                    timestamp : Utc::now(),
                                    ordertype
                                };
                                let mut trades = market.add_limit_order(order );

                                //  balance update of both the parties done here 
                                for trade in trades.iter_mut(){
                                    let seller_name = &trade.from;
                                    let buyer_name = &trade.to;
                                    // handling self-trade 
                                    if seller_name == buyer_name{
                                        continue;
                                    }
                                   if let [Some(buyer) , Some(seller)] = users.get_disjoint_mut([buyer_name , seller_name]){

                                    buyer.balance -= trade.trade_price * trade.trade_qty; //balance update
                                    seller.balance += trade.trade_price *trade.trade_qty; 

                                    let buyer_holdings = buyer.holdings.get_mut(&market_id).unwrap();
                                    let seller_holdings = seller.holdings.get_mut(&market_id).unwrap();

                                    match trade.stock_type {

                                        Option::OptionA => {
                                            buyer_holdings.stock_a += trade.trade_qty; //stock update
                                            seller_holdings.stock_a -= trade.trade_qty;
                                        }
                                        Option::OptionB => {
                                            buyer_holdings.stock_b += trade.trade_qty;
                                            seller_holdings.stock_b -= trade.trade_qty;
                                        }
                                    }
                                   }
                                }
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

                        Request::CreateMarketOrder { username, option, quantity, ordertype, resp , market_id  } => {
                            let user = users.get_mut(&username).unwrap();
                            let user_holdings = user.holdings.get_mut(&market_id).unwrap();
                            if let Some(market) = markets.get_mut(&market_id){
                                let trades = market.execute_market_order(username.clone(), ordertype, quantity , option );
                                for trade in trades.iter(){
                                    let buyer_name = &trade.to;
                                    let seller_name = &trade.from;
                                    // handling self-trade 
                                    if seller_name == buyer_name{
                                        continue;
                                    }
                                    if let [Some(buyer) , Some(seller)] = users.get_disjoint_mut([buyer_name,seller_name]){
                                        buyer.balance -= trade.trade_price * trade.trade_qty; //balance update
                                        seller.balance += trade.trade_price *trade.trade_qty; 
                                        let buyer_holdings = buyer.holdings.get_mut(&market_id).unwrap();
                                        let seller_holdings = seller.holdings.get_mut(&market_id).unwrap();

                                        match trade.stock_type {
                                            Option::OptionA => {
                                                buyer_holdings.stock_a += trade.trade_qty;
                                                seller_holdings.stock_a -= trade.trade_qty;
                                        }
                                            Option::OptionB => {
                                                buyer_holdings.stock_b += trade.trade_qty;
                                                seller_holdings.stock_b -= trade.trade_qty;
                                        }
                                        }
                                    }
                                }
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
                        Request::CreateMarket { username, market_name,resp } => {
                            let market = Market::initialise_market(market_name, username);
                            markets.insert(market.market_id.clone(), market);
                        }
                    }
                }
                None => break
            }
        }
    });
    tx
}

