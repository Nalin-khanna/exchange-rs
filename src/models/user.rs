use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    pub username : String ,
    pub password : String , 
    pub balance: u64,
    pub holdings: HashMap<String, UserHoldings>, // market_id → holdings in that market
}
impl User {
    pub fn get_holdings(&self, market_id: &str) -> UserHoldings {
    self.holdings.get(market_id).cloned().unwrap_or_default()
}
}
#[derive(Debug,Default , Clone)]
pub struct UserHoldings {
    pub stock_a : u64,
    pub stock_b : u64
}
impl UserHoldings{
    pub fn new (&mut self){
        self.stock_a = 0 ;
        self.stock_b = 0;
    }
}