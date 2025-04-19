//! # Cowry 🐄💰
//!
//! A financial math library with support for currencies, precise rounding, and
//! batch operations over monetary values using `Owo`.

pub mod currency;
pub mod error; 
pub mod owo;
pub mod rounding;
pub mod traits; 

pub use crate::currency::Currency;
pub use crate::owo::Owo;
pub use crate::traits::BatchOperations;
pub use crate::rounding::RoundingMode;

// Setup prelude module
pub mod prelude {
    //! The Cowry Prelude
    //!
    //! ```
    //! use cowry::prelude::*;
    //! ```

    pub use crate::Currency;
    pub use crate::Owo;
    pub use crate::RoundingMode;
    pub use crate::BatchOperations;
}
