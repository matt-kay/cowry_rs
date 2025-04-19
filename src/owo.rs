use crate::error::OwoError;
use crate::traits::BatchOperations;
use crate::{Currency, RoundingMode};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A Money type that uses minor units (e.g. cents, kobo).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owo {
    pub amount: i64,
    pub currency: Currency,
}

impl Owo {
    /// Create a new `Owo`.
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    ///
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo_ngn = Owo::new(500,ngn);
    ///
    /// let usd = Currency::new("USD", "$", 2);
    /// let owo_usd = Owo::new(100,usd);
    ///
    /// let jpy = Currency::new("JPY", "¥", 0);
    /// let owo_jpy = Owo::new(200,jpy);
    ///
    /// let btc = Currency::new("BTC", "₿", 8);
    /// let owo_btc = Owo::new(200,btc);
    ///
    /// assert_eq!(owo_ngn.format(),"₦5.00");
    /// assert_eq!(owo_usd.format(),"$1.00");
    /// assert_eq!(owo_jpy.format(),"¥200");
    /// assert_eq!(owo_btc.format(),"₿0.00000200");
    /// ```
    pub fn new(amount: i64, currency: Currency) -> Owo {
        Owo { amount, currency }
    }

    // Helper for rounding based on precision
    fn round_amount(&self, raw: f64) -> i64 {
        self.round_amount_with_mode(raw, RoundingMode::Nearest)
    }

    // Helper for rounding based on precision with rounding mode
    fn round_amount_with_mode(&self, raw: f64, mode: RoundingMode) -> i64 {
        let factor = 10i64.pow(self.currency.precision as u32) as f64;
        let scaled = raw * factor;
        let rounded = match mode {
            RoundingMode::Nearest => scaled.round(),
            RoundingMode::Floor => scaled.floor(),
            RoundingMode::Ceil => scaled.ceil(),
        };
        rounded as i64
    }

    /// Format the money into a display string.
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    ///
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(500,ngn);
    ///
    /// assert_eq!(owo.format(),"₦5.00");
    /// ```
    pub fn format(&self) -> String {
        let precision = self.currency.precision as usize;
        let divisor = 10i64.pow(precision as u32);
        let whole = self.amount / divisor;
        let fraction = (self.amount.abs() % divisor) as usize;
        let format_precision = match precision {
            0 => format!(""),
            1.. => format!(".{:0width$}", fraction, width = precision),
        };
        format!("{}{}{}", self.currency.symbol, whole, format_precision)
    }

    /// Returns the raw amount in minor units.
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(500,ngn);
    /// assert_eq!(owo.get_amount(),500);
    /// ```
    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    /// Returns the currency code (e.g., "NGN")
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(500,ngn);
    /// assert_eq!(owo.get_currency(),"NGN");
    /// ```
    pub fn get_currency(&self) -> &str {
        self.currency.code.as_str()
    }

    /// Returns the precision (e.g., 2 for NGN)
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(500,ngn);
    /// assert_eq!(owo.get_precision(),2);
    /// ```
    pub fn get_precision(&self) -> u8 {
        self.currency.precision
    }

    /// Deserialize from JSON string
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    ///
    /// let json_str = r#"{"amount": 500,"currency": { "code": "EUR","symbol": "€","precision": 2  }}"#;
    ///
    /// let owo = Owo::from_json(json_str).unwrap();
    ///
    /// assert_eq!(owo.amount, 500);
    /// assert_eq!(owo.currency.code, "EUR");
    /// assert_eq!(owo.currency.precision, 2);
    /// ```
    pub fn from_json(json_str: &str) -> Result<Owo, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Serialize to JSON string
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(500,ngn);
    ///
    /// let json = owo.to_json().unwrap();
    /// assert_eq!(json, r#"{"amount":500,"currency":{"code":"NGN","symbol":"₦","precision":2}}"#);
    /// ```
    pub fn to_json(&self) -> Result<String, OwoError> {
        serde_json::to_string(self).map_err(OwoError::from)
    }

    /// Compares `self` and `rhs` for equality
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo1 = Owo::new(500,ngn.clone());
    /// let owo2 = Owo::new(500,ngn.clone());
    ///
    /// assert!(owo1.eq(&owo2));
    /// ```
    pub fn eq(&self, rhs: &Self) -> bool {
        self.currency == rhs.currency && self.amount == rhs.amount
    }

    /// Checks if `self` is less than `rhs`
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo1 = Owo::new(500,ngn.clone());
    /// let owo2 = Owo::new(700,ngn.clone());
    ///
    /// assert!(owo1.lt(&owo2));
    /// ```
    pub fn lt(&self, rhs: &Self) -> bool {
        self.currency == rhs.currency && self.amount < rhs.amount
    }

    /// Checks if `self` is greater than `rhs`
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo1 = Owo::new(1000,ngn.clone());
    /// let owo2 = Owo::new(700,ngn.clone());
    ///
    /// assert!(owo1.gt(&owo2));
    /// ```
    pub fn gt(&self, rhs: &Self) -> bool {
        self.currency == rhs.currency && self.amount > rhs.amount
    }

    /// Rounds the amount to the specified precision of the currency.
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let mut owo = Owo::new(1247, ngn);
    /// owo.round_to_precision();
    ///
    /// assert_eq!(owo.get_amount(), 1247);
    /// ```
    pub fn round_to_precision(&mut self) {
        let raw = self.amount as f64 / 10f64.powi(self.currency.precision as i32);
        self.amount = self.round_amount(raw);
    }

    /// Multiplies the amount by a scalar
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(1230,ngn);
    ///
    /// assert_eq!(owo.multiply(1.5).get_amount(),1845);
    /// ```
    pub fn multiply(&self, scalar: f64) -> Owo {
        self.multiply_with_mode(scalar, RoundingMode::Nearest)
    }

    /// Divides the amount by a scalar
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(1000,ngn);
    ///
    /// assert_eq!(owo.divide(4.5).get_amount(),222);
    /// ```
    pub fn divide(&self, scalar: f64) -> Owo {
        self.divide_with_mode(scalar, RoundingMode::Nearest)
    }

    /// Returns a Owo representing a given percentage of the amount by a scalar
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(1000,ngn);
    ///
    /// assert_eq!(owo.percentage(0.5).get_amount(),5);
    /// ```
    pub fn percentage(&self, percent: f64) -> Owo {
        self.percentage_with_mode(percent, RoundingMode::Nearest)
    }

    /// Multiplies the amount by a scalar with rounding mode
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(105,ngn);  // ₦1.05
    /// // 1.05 * 2.5 = 2.625 → 2.63 (half up)
    ///  let rounding_mode_nearest_even_half_up = owo.multiply_with_mode(2.5, RoundingMode::Nearest);
    ///
    /// // 1.05 * 2.5 = 2.625 → floor = 2.62
    /// let rounding_mode_floor_positive = owo.multiply_with_mode(2.5, RoundingMode::Floor);
    ///
    ///  // 1.05 * 2.5 = 2.625 → ceil = 2.63
    /// let rounding_mode_ceil_positive = owo.multiply_with_mode(2.5, RoundingMode::Ceil);
    ///
    /// // 1.05 * -2.5 = -2.625 → floor = -2.63
    /// let rounding_mode_floor_negative = owo.multiply_with_mode(-2.5, RoundingMode::Floor);
    ///
    /// // 1.05 * -2.5 = -2.625 → ceil = -2.62
    /// let rounding_mode_ceil_negative = owo.multiply_with_mode(-2.5, RoundingMode::Ceil);
    ///
    /// assert_eq!(rounding_mode_nearest_even_half_up.get_amount(), 263); //₦2.63
    /// assert_eq!(rounding_mode_floor_positive.get_amount(), 262);
    /// assert_eq!(rounding_mode_ceil_positive.get_amount(), 263);
    /// assert_eq!(rounding_mode_floor_negative.get_amount(), -263);
    /// assert_eq!(rounding_mode_ceil_negative.get_amount(), -262);
    /// ```
    pub fn multiply_with_mode(&self, scalar: f64, mode: RoundingMode) -> Owo {
        let raw = (self.amount as f64 / 10f64.powi(self.currency.precision as i32)) * scalar;

        Owo {
            amount: self.round_amount_with_mode(raw, mode),
            currency: self.currency.clone(),
        }
    }

    /// Divides the amount by a scalar with rounding mode
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(105,ngn);  // ₦1.05
    /// // 1.05 / 2.8 = 0.375 → 0.38
    ///  let rounding_mode_nearest_even_half_up = owo.divide_with_mode(2.8, RoundingMode::Nearest);
    ///
    /// // 1.05 / 2.8 = 0.375 → floor = 0.37
    /// let rounding_mode_floor_positive = owo.divide_with_mode(2.8, RoundingMode::Floor);
    ///
    ///  // 1.05 / 2.8 = 0.375 → ceil = 0.38
    /// let rounding_mode_ceil_positive = owo.divide_with_mode(2.8, RoundingMode::Ceil);
    ///
    /// // 1.05 / -2.8 = -0.375 → floor = -0.38
    /// let rounding_mode_floor_negative = owo.divide_with_mode(-2.8, RoundingMode::Floor);
    ///
    /// // 1.05 / -2.8 = -0.375 → ceil = -0.37
    /// let rounding_mode_ceil_negative = owo.divide_with_mode(-2.8, RoundingMode::Ceil);
    ///
    /// assert_eq!(rounding_mode_nearest_even_half_up.get_amount(), 38); //₦0.38
    /// assert_eq!(rounding_mode_floor_positive.get_amount(), 37);
    /// assert_eq!(rounding_mode_ceil_positive.get_amount(), 38);
    /// assert_eq!(rounding_mode_floor_negative.get_amount(), -38);
    /// assert_eq!(rounding_mode_ceil_negative.get_amount(), -37);
    /// ```
    pub fn divide_with_mode(&self, scalar: f64, mode: RoundingMode) -> Owo {
        let raw = (self.amount as f64 / 10f64.powi(self.currency.precision as i32)) / scalar;
        Owo {
            amount: self.round_amount_with_mode(raw, mode),
            currency: self.currency.clone(),
        }
    }

    /// Returns a Owo representing a given percentage of the amount by a scalar with rounding mode
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    /// let owo = Owo::new(105,ngn);  // ₦1.05
    /// // 28% of 1.05 = 2.94 → 3
    ///  let rounding_mode_nearest_even_half_up = owo.percentage_with_mode(2.8, RoundingMode::Nearest);
    ///
    /// // 28% of 1.05 = 2.94 → floor = 2
    /// let rounding_mode_floor_positive = owo.percentage_with_mode(2.8, RoundingMode::Floor);
    ///
    ///  // 28% of 1.05 = 2.94 → ceil = 3
    /// let rounding_mode_ceil_positive = owo.percentage_with_mode(2.8, RoundingMode::Ceil);
    ///
    /// // -28% of 1.05 = -2.94 → floor = -3
    /// let rounding_mode_floor_negative = owo.percentage_with_mode(-2.8, RoundingMode::Floor);
    ///
    /// // -28% of 1.05 = -2.94 → ceil = -2
    /// let rounding_mode_ceil_negative = owo.percentage_with_mode(-2.8, RoundingMode::Ceil);
    ///
    /// assert_eq!(rounding_mode_nearest_even_half_up.get_amount(), 3); //₦3.00
    /// assert_eq!(rounding_mode_floor_positive.get_amount(), 2);
    /// assert_eq!(rounding_mode_ceil_positive.get_amount(), 3);
    /// assert_eq!(rounding_mode_floor_negative.get_amount(), -3);
    /// assert_eq!(rounding_mode_ceil_negative.get_amount(), -2);
    /// ```
    pub fn percentage_with_mode(&self, percent: f64, mode: RoundingMode) -> Owo {
        let raw =
            (self.amount as f64 / 10f64.powi(self.currency.precision as i32)) * (percent / 100.0);
        Owo {
            amount: self.round_amount_with_mode(raw, mode),
            currency: self.currency.clone(),
        }
    }
}

// Addition
impl Add for Owo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.currency, rhs.currency, "Currency mismatch in Add");
        Self {
            amount: self.amount + rhs.amount,
            currency: self.currency,
        }
    }
}

// Subtraction
impl Sub for Owo {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.currency, rhs.currency, "Currency mismatch in Sub");
        Self {
            amount: self.amount - rhs.amount,
            currency: self.currency,
        }
    }
}

// Multiplication by scalar
impl Mul<i64> for Owo {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            amount: self.amount * rhs,
            currency: self.currency,
        }
    }
}

// Division by scalar
impl Div<i64> for Owo {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            amount: self.amount / rhs,
            currency: self.currency,
        }
    }
}

impl Neg for Owo {
    type Output = Owo;
    fn neg(self) -> Owo {
        Owo {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

impl fmt::Display for Owo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl PartialEq for Owo {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}

impl PartialOrd for Owo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency != other.currency {
            None
        } else {
            self.amount.partial_cmp(&other.amount)
        }
    }
}


impl BatchOperations for Vec<Owo> {
    /// Returns a collection of Owo representing the amount multiply by a scalar
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    ///
    /// let items = vec![Owo::new(1000,ngn.clone()),Owo::new(500,ngn.clone()),Owo::new(200,ngn.clone())];
    ///
    /// //multiply every item by 1.5
    /// assert_eq!(items.multiply_all(1.5),vec![Owo::new(1500,ngn.clone()),Owo::new(750,ngn.clone()),Owo::new(300,ngn.clone())]);
    /// ```
    fn multiply_all(&self, scalar: f64) -> Vec<Owo> {
        self.iter().map(|c| c.multiply(scalar)).collect()
    }

    /// Returns a collection of Owo representing the amount divided by a scalar
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    ///
    /// let items = vec![Owo::new(1000,ngn.clone()),Owo::new(500,ngn.clone()),Owo::new(200,ngn.clone())];
    ///
    /// //divide every item by 5
    /// assert_eq!(items.divide_all(5.0),vec![Owo::new(200,ngn.clone()),Owo::new(100,ngn.clone()),Owo::new(40,ngn.clone())]);
    /// ```
    fn divide_all(&self, scalar: f64) -> Vec<Owo> {
        self.iter().map(|c| c.divide(scalar)).collect()
    }

    /// Returns a collection of Owo representing a given percentage of the amount
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    ///
    /// let items = vec![Owo::new(1000,ngn.clone()),Owo::new(500,ngn.clone()),Owo::new(200,ngn.clone())];
    ///
    /// // 50 % of every item
    /// assert_eq!(items.percentage_all(50.0),vec![Owo::new(500,ngn.clone()),Owo::new(250,ngn.clone()),Owo::new(100,ngn.clone())]);
    /// ```
    fn percentage_all(&self, percent: f64) -> Vec<Owo> {
        self.iter().map(|c| c.percentage(percent)).collect()
    }

    /// Returns a collection of Owo representing the amount multiply by a scalar
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    ///
    /// let items = vec![Owo::new(1000,ngn.clone()),Owo::new(500,ngn.clone()),Owo::new(200,ngn.clone())];
    ///
    /// //multiply every item by 1.5
    /// assert_eq!(items.multiply_all_with_mode(1.5,RoundingMode::Ceil),vec![Owo::new(1500,ngn.clone()),Owo::new(750,ngn.clone()),Owo::new(300,ngn.clone())]);
    /// ```
    fn multiply_all_with_mode(&self, scalar: f64, mode: RoundingMode) -> Vec<Owo> {
        self.iter()
            .map(|c| c.multiply_with_mode(scalar, mode))
            .collect()
    }

    /// Returns a collection of Owo representing the amount divided by a scalar with rounding mode
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    ///
    /// let items = vec![Owo::new(1000,ngn.clone()),Owo::new(500,ngn.clone()),Owo::new(200,ngn.clone())];
    ///
    /// //divide every item by 5
    /// assert_eq!(items.divide_all_with_mode(5.0,RoundingMode::Ceil),vec![Owo::new(200,ngn.clone()),Owo::new(100,ngn.clone()),Owo::new(40,ngn.clone())]);
    /// ```
    fn divide_all_with_mode(&self, scalar: f64, mode: RoundingMode) -> Vec<Owo> {
        self.iter()
            .map(|c| c.divide_with_mode(scalar, mode))
            .collect()
    }

    /// Returns a collection of Owo representing a given percentage of the amount with rounding mode
    ///
    /// #Example
    /// ```
    /// # use cowry::prelude::*;
    /// let ngn = Currency::new("NGN", "₦", 2);
    ///
    /// let items = vec![Owo::new(1000,ngn.clone()),Owo::new(500,ngn.clone()),Owo::new(200,ngn.clone())];
    ///
    /// // 50 % of every item
    /// assert_eq!(items.percentage_all_with_mode(50.0,RoundingMode::Ceil),vec![Owo::new(500,ngn.clone()),Owo::new(250,ngn.clone()),Owo::new(100,ngn.clone())]);
    /// ```
    fn percentage_all_with_mode(&self, scalar: f64, mode: RoundingMode) -> Vec<Owo> {
        self.iter()
            .map(|c| c.percentage_with_mode(scalar, mode))
            .collect()
    }
}
