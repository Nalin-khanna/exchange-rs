pub mod routes;
pub use routes::*;
pub mod worker;
pub use worker::processor;
pub use worker::*;
pub mod utils;
pub use utils::*;
pub mod models;
pub use models::*;
use tokio::sync::mpsc;
pub struct AppState {
   pub worker: mpsc::Sender<Request>,
}