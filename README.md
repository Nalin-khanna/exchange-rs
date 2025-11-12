# Polymarket-rs 
Polymarket-rs is a high-performance, in-memory prediction market platform written in Rust. It uses an asynchronous, single-threaded actor model to manage state (users, markets, and order books) without locks, ensuring safe and fast concurrent operations. The core of the application is a single background worker (an "actor") that processes all requests sequentially via an mpsc channel. This design avoids the need for Mutex or RwLock, guaranteeing data consistency and eliminating race conditions. 
>Great docs - https://doc.rust-lang.org/book/ch16-02-message-passing.html

**Architecture**
1. The application follows the Actor Model, which separates the web handlers from the application state. 
2. Actix Web Handlers (or other clients) receive HTTP requests. 
3. Handlers create a Request enum (e.g., CreateMarket, CreateLimitOrder).
4. The Request is sent to the worker via a tokio::sync::mpsc::Sender . Here, the actix handlers running on different threads are multiple producers and our worker thread is the single consumer that processes the requests sent by these producers sequentially. 
5. A single Background Worker (worker_loop) runs in its own task, processing messages from the mpsc channel one by one. 
6. The worker holds the entire application State (User map, Market map) in its local scope.
7. The worker modifies the state and sends a response back to the handler via a tokio::sync::oneshot::Sender 

> Oneshot docs - https://docs.rs/oneshot/latest/oneshot/

**Core Data Structures** 
1. *User*: Stores balance and a HashMap<MarketId, UserHoldings>.

2. *UserHoldings*: Stores stock_a and stock_b (shares for each outcome) counts for a specific market. 

3. *Market*: Stores its ID, creator, name, winning_outcome, is_settled, a list of trades, and two OrderBooks (one for stock_a and one for stock_b). 

4. *OrderBook*: The core matching engine, containing BTreeMaps for buy and sell VecDeque<Order>s for a single outcome. 

5. *Order*: A user's instruction to buy or sell a specific outcome share. 

6. *Trade*: The result of a matched buy and sell order. 

**Features** 
1. *User Management*: Secure signup and signin with password hashing. 
2. *Market Creation*: Users can create new, distinct prediction markets. 
3. *Share Minting*: A SplitStocks function to seed user accounts with shares for each outcome (StockA and StockB).
4. *Detailed Order Book*: BTreeMap-based order books for efficient price-level management. Tracks bids (buys) and asks (sells) separately for each outcome.
5. *Limit Orders*: Place limit orders (CreateLimitOrder) that are either booked or matched. Handles partial fills. Funds and shares are locked immediately. Provides price improvement refunds for buyers.
6. *Market Orders*: Execute market orders (ExecuteMarketOrder) that fill against the book. 
7. *State Management*: All user balances and share holdings (stock_a, stock_b) are updated atomically after trades.
8. *Concurrency Safe*: All state-mutating logic is fully encapsulated within the single-threaded actor.


##  API Reference

> **Base URL:** https://shark-app-iypfs.ondigitalocean.app

All routes communicate with the background worker via a message-passing (actor) channel. Responses are in JSON format.

---

### User Management

#### `POST /signup`
Create a new user.

**Request:**
```json
{
  "username": "user1",
  "password": "secret123"
}
```

**Response:**
```json
{
  "msg": "user1"
}
```

#### `POST /signin`
Authenticate an existing user.

**Request:**
```json
{
  "username": "user1",
  "password": "secret123"
}
```

**Response:**
```json
{
  "token": "jwt_token_here",
  "msg": "user1"
}
```

---

### Market Management

#### `POST /create_market`
Create a new prediction market.

**Request:**
```json
{
  "market_name": "Will BTC be above $100k by 2026?"
}
```

**Response:**
```json
{
  "market_id": "abc123xyz"
}
```

---

### Split (Mint) Stocks

#### `POST /split_stocks`
Mint Stock A and Stock B for a given market by locking collateral from user balance.

**Request:**
```json
{
  "market_id": "abc123xyz",
  "amount": 100
}
```

**Response:**
```json
{
  "status": "success",
  "data": "Minted 100 of Stock A and B"
}
```

---

### Orders

#### `POST /limitorder`
Create a limit order for a given outcome.

**Request:**
```json
{
  "stock_type": "StockA",
  "market_id": "abc123xyz",
  "price": 45,
  "quantity": 10,
  "ordertype": "Buy"
}
```

**Note:** `stock_type` can be `"StockA"` or `"StockB"`. `ordertype` can be `"Buy"` or `"Sell"`.

**Response (Matched):**
```json
[
  {
    "from": "user1",
    "to": "user2",
    "trade_qty": 10,
    "trade_price": 45,
    "stock_type": "StockA"
  }
]
```

**Response (Not Matched):**
```json
{
  "msg": "Order placed waiting to be matched"
}
```

#### `POST /marketorder`
Execute a market order that fills against the best available limit orders.

**Request:**
```json
{
  "market_id": "abc123xyz",
  "stock_type": "StockA",
  "quantity": 10,
  "ordertype": "Buy"
}
```

**Note:** `stock_type` can be `"StockA"` or `"StockB"`. `ordertype` can be `"Buy"` or `"Sell"`.

**Response:**
```json
[
  {
    "from": "user1",
    "to": "user2",
    "trade_qty": 10,
    "trade_price": 45,
    "stock_type": "StockA"
  }
]
```

---

### Query Endpoints

#### `GET /user_details`
Fetch user details including balance and holdings.

**Response:**
```json
{
  "balance": 1000,
  "holdings": {
    "stock_a": 50,
    "stock_b": 50
  }
}
```

#### `GET /get_orderbook`
Fetch the complete order book for a specific market.

**Response:**
```json
{
  "stock_a": {
    "buy": {
      "45": [
        {
          "price": 45,
          "quantity": 10,
          "stock_type": "StockA",
          "username": "user1",
          "timestamp": "2025-11-12T10:30:00Z",
          "ordertype": "Buy",
          "market_id": "abc123xyz"
        }
      ]
    },
    "sell": {
      "55": [
        {
          "price": 55,
          "quantity": 10,
          "stock_type": "StockA",
          "username": "user2",
          "timestamp": "2025-11-12T10:31:00Z",
          "ordertype": "Sell",
          "market_id": "abc123xyz"
        }
      ]
    }
  },
  "stock_b": {
    "buy": {
      "40": [
        {
          "price": 40,
          "quantity": 5,
          "stock_type": "StockB",
          "username": "user3",
          "timestamp": "2025-11-12T10:32:00Z",
          "ordertype": "Buy",
          "market_id": "abc123xyz"
        }
      ]
    },
    "sell": {
      "60": [
        {
          "price": 60,
          "quantity": 5,
          "stock_type": "StockB",
          "username": "user4",
          "timestamp": "2025-11-12T10:33:00Z",
          "ordertype": "Sell",
          "market_id": "abc123xyz"
        }
      ]
    }
  }
}
```

**Structure:**
- Orders are grouped by `stock_type` (`stock_a` or `stock_b`)
- Each stock type has `buy` and `sell` sides
- Orders are keyed by price level (as strings)
- Each price level contains an array of order objects
**Structure:**
- Orders are grouped by `stock_type` (`stock_a` or `stock_b`)
- Each stock type has `buy` and `sell` sides
- Orders are keyed by price level (as strings)
- Each price level contains an array of order objects
