extern crate anyhow;

pub mod ser;
pub mod tte;

#[cfg(feature = "ps_address")]
mod sp_client;

pub mod net {
    #[cfg(any(feature = "dfinance_address", feature = "libra_address"))]
    pub use dnclient::blocking::{get_resource, client::DnodeRestClient as NodeClient};
    #[cfg(feature = "ps_address")]
    pub use super::sp_client::*;
}
