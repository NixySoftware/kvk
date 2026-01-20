//! Rust OpenAPI client for [Kamer van Koophandel](https://www.kvk.nl/).
//!
//! For more information see <https://developers.kvk.nl/>.
//!
//! ## OpenAPI specification versions:
//! - Basisprofiel - 1.4.0
//! - Mutatieservice - 1.0.9
//! - Naamgeving API - 1.1.1
//! - Open Data Basis Bedrijfsgegevens - 1.1.0
//! - Open Data Jaarrekeningen - 1.0
//! - Vestigingsprofiel - 1.4.0
//! - Zoeken - 2.1.0

#[cfg(feature = "basisprofiel")]
pub mod basisprofiel;
#[cfg(feature = "mutatieservice")]
pub mod mutatieservice;
#[cfg(feature = "naamgeving")]
pub mod naamgeving;
#[cfg(feature = "open-data-basis")]
pub mod open_data_basis;
#[cfg(feature = "open-data-jaarrekeningen")]
pub mod open_data_jaarrekeningen;
#[cfg(feature = "vestigingsprofiel")]
pub mod vestigingsprofiel;
#[cfg(feature = "zoeken")]
pub mod zoeken;
