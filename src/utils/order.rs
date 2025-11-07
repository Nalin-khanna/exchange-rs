
use chrono::{DateTime, Utc};
use std::collections::{ BTreeMap , VecDeque};
#[derive(Debug , Clone )]
pub struct Order {
    price : u64, 
    quantity : u64,
    option: Option,
    username : String,
    timestamp: DateTime<Utc>,
    ordertype: Ordertype,
    
}
#[derive(Debug , Clone )]
pub struct Trade {
    from : String,
    to : String ,
    trade_qty : u64 ,
    trade_price : u64
}
#[derive(Debug , Clone )]
pub enum Option {
    OptionA ,
    OptionB 
}
#[derive(Debug , Clone)]
pub enum Ordertype{
    Buy,
    Sell
}
#[derive(Debug)]
pub struct OrderBook {
    buy: BTreeMap<u64, VecDeque<Order>>,
    sell: BTreeMap<u64, VecDeque<Order>>,
}
impl OrderBook {
    pub fn new () -> Self{
        Self { buy: BTreeMap::new(), sell: BTreeMap::new() }
    }
    pub fn add_limit_order(
        &mut self,
        mut order : Order
    )-> Vec<Trade>{
        let mut trades = vec![];
        match order.ordertype {
            Ordertype::Buy => {
                while let Some((&lowest_sell_price , queue)) = self.sell.iter_mut().next(){
                    if order.price >= lowest_sell_price && order.quantity > 0{
                        if let Some (mut sell_order) = queue.pop_front(){
                            let trade_qty = order.quantity.min(sell_order.quantity);  // minimum quantity out of buy order and sell order popped from queue
                            order.quantity -= trade_qty;            //  minimum qty can only be matched
                            sell_order.quantity -= trade_qty;

                            trades.push(Trade{
                                from : order.username.clone(),
                                to : sell_order.username.clone(),
                                trade_qty,
                                trade_price : lowest_sell_price
                            });
                            if sell_order.quantity > 0 {          // if there is still qty left , push it back to front of queue
                                queue.push_front(sell_order);
                            }
                        }else{
                            self.sell.remove(&lowest_sell_price);
                        }
                    }else{
                        break
                    }
                }
                if order.quantity > 0 {
                    self.buy.entry(order.price).or_insert_with(VecDeque::new).push_back(order);  // after the loop , if buy order has qty left , push it to buy BTREE
                }
            }
            Ordertype::Sell => {
                while let Some((&highest_buy_price, queue)) = self.buy.iter_mut().next_back() {
                    if order.price <= highest_buy_price && order.quantity > 0 {
                        if let Some(mut buy_order) = queue.pop_front() {
                            let trade_qty = order.quantity.min(buy_order.quantity);
                            order.quantity -= trade_qty;
                            buy_order.quantity -= trade_qty;

                            trades.push(Trade { 
                                from: order.username.clone(), 
                                to: buy_order.username.clone(),
                                trade_qty, 
                                trade_price: highest_buy_price 
                            });

                            if buy_order.quantity > 0 {
                                queue.push_front(buy_order);
                            }
                        } else {
                            self.buy.remove(&highest_buy_price);
                        }
                    } else {
                        break; // no matching buy
                    }
                }

                if order.quantity > 0 {
                    self.sell.entry(order.price).or_insert_with(VecDeque::new).push_back(order);
                }
            }
        }
        trades
    }
    
    pub fn execute_market_order(&mut self , username : String , ordertype : Ordertype , mut quantity : u64 ) -> Vec<Trade> {
        let mut trades = vec![];

        match ordertype {
            Ordertype::Buy => {
                while quantity > 0 {
                    if let Some((&lowest_sell_price, queue)) = self.sell.iter_mut().next() {
                        if let Some(mut sell_order) = queue.pop_front() {
                            let trade_qty = quantity.min(sell_order.quantity);
                            quantity -= trade_qty;
                            sell_order.quantity -= trade_qty;

                            trades.push(
                                Trade{
                                    from : username.clone(),
                                    to : sell_order.username.clone(),
                                    trade_qty,
                                    trade_price : lowest_sell_price
                                }
                            );

                            if sell_order.quantity > 0 {
                                queue.push_front(sell_order);
                            }
                        } else {
                            self.sell.remove(&lowest_sell_price);
                        }
                    } else {
                        break; // no sells left
                    }
                }
            }

            Ordertype::Sell => {
                while quantity > 0 {
                    if let Some((&highest_buy_price, queue)) = self.buy.iter_mut().next_back() {
                        if let Some(mut buy_order) = queue.pop_front() {
                            let trade_qty = quantity.min(buy_order.quantity);
                            quantity -= trade_qty;
                            buy_order.quantity -= trade_qty;

                            trades.push(Trade { from: username.clone(), to: buy_order.username.clone(), trade_qty, trade_price: highest_buy_price });
                            
                            if buy_order.quantity > 0 {
                                queue.push_front(buy_order);
                            }
                        } else {
                            self.buy.remove(&highest_buy_price);
                        }
                    } else {
                        break; // no buys left
                    }
                }
            }
        }

        trades
    }
}