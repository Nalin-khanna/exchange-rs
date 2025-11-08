use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    pub username : String ,
    pub password : String , 
    pub balance: u64,
    pub holdings: HashMap<String, UserHoldings>, // market_id → holdings in that market
}
#[derive(Debug)]
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