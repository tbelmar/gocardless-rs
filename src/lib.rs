//! # Unofficial GoCardless Rust SDK
//! 
//! [![Crates.io Version](https://img.shields.io/crates/v/gocardless-unofficial)](https://crates.io/crates/gocardless-unofficial)
//! [![docs.rs](https://img.shields.io/docsrs/gocardless-unofficial)](https://docs.rs/gocardless-unofficial)
//! 
//! An unofficial rust library to interact with the [GoCardless Bank Account Data API](https://gocardless.com/bank-account-data/).
//! 
//! ## Usage
//! 
//! Add the following to `Cargo.toml`
//! ```toml
//! [dependencies]
//! gocardless-unofficial = "0.1"
//! ```
//! 
//! ```rs
//! use gocardless_unofficial::Client;
//! 
//! #[tokio::main]
//! pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let secret_id = std::env::var("GOCARDLESS_SECRET_ID").unwrap();
//!     let secret_key = std::env::var("GOCARDLESS_SECRET_KEY").unwrap();
//! 
//!     let client = Client::new(secret_id, secret_key).await?;
//! 
//!     // use client to interact with GoCardless!
//! 
//!     Ok(())
//! }
//! ```
//! 
//! See [here](/examples) for more examples.
//! 
//! ## Authorization
//! 
//! Head to the [User Secrets](https://bankaccountdata.gocardless.com/user-secrets/) page, generate a new user secret and copy both the secret ID and secret key.
//! 
//! Next, pass the secret ID and secret key to the `Client::new` constructor as `String`!

mod model;
pub use model::*;

mod client;
pub use client::*;