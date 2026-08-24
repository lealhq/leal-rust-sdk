//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Stores**
//! - **Cards**
//! - **Customers**
//! - **Customer Cards**
//! - **Locations**
//! - **Posters**
//! - **Rewards**
//! - **Status**

use crate::{ApiError, ClientConfig};

pub mod cards;
pub mod customer_cards;
pub mod customers;
pub mod locations;
pub mod posters;
pub mod rewards;
pub mod status;
pub mod stores;
pub struct LealClient {
    pub config: ClientConfig,
    pub stores: StoresClient,
    pub cards: CardsClient,
    pub customers: CustomersClient,
    pub customer_cards: CustomerCardsClient,
    pub locations: LocationsClient,
    pub posters: PostersClient,
    pub rewards: RewardsClient,
    pub status: StatusClient,
}

impl LealClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            stores: StoresClient::new(config.clone())?,
            cards: CardsClient::new(config.clone())?,
            customers: CustomersClient::new(config.clone())?,
            customer_cards: CustomerCardsClient::new(config.clone())?,
            locations: LocationsClient::new(config.clone())?,
            posters: PostersClient::new(config.clone())?,
            rewards: RewardsClient::new(config.clone())?,
            status: StatusClient::new(config.clone())?,
        })
    }
}

pub use cards::CardsClient;
pub use customer_cards::CustomerCardsClient;
pub use customers::CustomersClient;
pub use locations::LocationsClient;
pub use posters::PostersClient;
pub use rewards::RewardsClient;
pub use status::StatusClient;
pub use stores::StoresClient;
