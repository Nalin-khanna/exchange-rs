use crate::{User, UserHoldings, order::*};
use nanoid::nanoid;
#[derive(Debug)]
pub struct Market {
    pub market_id : String,
    pub created_by : String,
    pub market_name : String,
    pub stock_a: OrderBook,
    pub stock_b: OrderBook,
    pub trades : Vec<Trade>
}

impl Market {
    pub fn initialise_market (market_name : String , username : String) -> Self{ 
        let market = Market{
            market_id : nanoid!(),
            created_by : username,
            market_name ,
            stock_a : OrderBook::new(),
            stock_b : OrderBook::new(),
            trades : vec![]
        };
        market
    }
    pub fn add_limit_order(&mut self , order : Order) -> Vec<Trade> {
        match order.option {
            Option::OptionA => { 
               let mut v =  self.stock_a.add_limit_order(order );
               let v2 = v.clone();
               self.trades.append(&mut v);
               v2
            }
            Option::OptionB => {
               let mut v =  self.stock_b.add_limit_order(order );
               let v2 = v.clone();
               self.trades.append(&mut v);
               v2
            }
        }
        
    }
    pub fn execute_market_order(&mut self , username : String , ordertype : Ordertype , quantity : u64, option : Option  )-> Vec<Trade>{
        let mut v = match option {
            Option::OptionA => {
                self.stock_a.execute_market_order(username, ordertype, quantity)
            }
            Option::OptionB => {
                self.stock_b.execute_market_order(username, ordertype, quantity )
            }
        };
        let v2 = v.clone();
        self.trades.append(&mut v);
        v2
    }
}