---

# 💸 Cowry — A Precise Money Type in Rust

`Cowry` is a robust and flexible money type written in Rust, designed to represent and manipulate monetary values accurately using minor units (like cents, kobo, or satoshi). It supports multiple currencies, rounding modes, arithmetic operations, formatting, and serialization.

---

---

`Owo` is a Rust struct for handling money in minor units (like cents, kobo, etc.). It supports safe arithmetic, precision rounding, formatting, serialization, and batch operations.

---

## ✨ Features

- 📐 Minor-unit precision (e.g., ₦1.50 is stored as `150`)
- 💱 Multi-currency support
- ➕ Arithmetic operations (add, subtract, multiply, divide)
- 🎯 Rounding with customizable modes (Nearest, Floor, Ceil)
- 🧾 Serialization and deserialization with `serde`
- 🧮 Percentage calculations
- 💬 Pretty printing and formatting
- 📦 Batch operations

---

## 🧰 Usage

### 💵 Creating a new `Owo` instance

```rust
use cowry::prelude::*;

let ngn = Currency::new("NGN", "₦", 2);
let owo_ngn = Owo::new(500,ngn);

let usd = Currency::new("USD", "$", 2);
let owo_usd = Owo::new(100,usd);

let jpy = Currency::new("JPY", "¥", 0);
let owo_jpy = Owo::new(200,jpy);

let btc = Currency::new("BTC", "₿", 8);
let owo_btc = Owo::new(200,btc);

assert_eq!(owo_ngn.format(),"₦5.00");
assert_eq!(owo_usd.format(),"$1.00");
assert_eq!(owo_jpy.format(),"¥200");
assert_eq!(owo_btc.format(),"₿0.00000200");
```

### 📖 Formatting

```rust
use cowry::prelude::*;

let ngn = Currency::new("NGN", "₦", 2);
let owo = Owo::new(500,ngn);

assert_eq!(owo.format(),"₦5.00");
```
### 📦 Deserialization

```rust
use cowry::prelude::*;
    
let json_str = r#"{"amount": 500,"currency": { "code": "EUR","symbol": "€","precision": 2  }}"#;
    
let owo = Owo::from_json(json_str).unwrap();
    
assert_eq!(owo.amount, 500);
assert_eq!(owo.currency.code, "EUR");
assert_eq!(owo.currency.precision, 2);
```

### 📦 Serialization

```rust
use cowry::prelude::*;
let ngn = Currency::new("NGN", "₦", 2);
let owo = Owo::new(500,ngn);
    
let json = owo.to_json().unwrap();
assert_eq!(json, r#"{"amount":500,"currency":{"code":"NGN","symbol":"₦","precision":2}}"#);
```

### 🔢 Arithmetic

```rust
use cowry::prelude::*;

let usd = Currency::new("USD", "$", 2);
let price_1 = Owo::new(1000,usd.clone()); //$10.00
let price_2 = Owo::new(2000,usd.clone()); //$20.00

let result = price_1 + price_2; //$30.00

```

### 🎯 Rounding Modes

```rust
let value = Owo::new(105, ngn); // ₦1.05
let rounded = value.multiply_with_mode(2.5, RoundingMode::Nearest); // ₦2.63
```

---

## 🏗️ Struct Overview

```rust
pub struct Owo {
    pub amount: i64,
    pub currency: Currency,
}
```

- `amount`: The value in **minor units**
- `currency`: The currency type (e.g., NGN, USD), includes precision settings

---

## 🛠️ Core Methods

### 💰 Creation & Parsing
- `new(amount: i64, currency: Currency) -> Owo`  
  Create a new `Owo` instance.

- `from_json(json_str: &str) -> Result<Owo, serde_json::Error>`  
  Deserialize a JSON string to `Owo`.

- `to_json(&self) -> Result<String, OwoError>`  
  Serialize the `Owo` instance to JSON.

---

### 📏 Value Access & Info

- `get_amount() -> i64`  
  Returns the internal amount in minor units.

- `get_currency() -> &str`  
  Returns the currency code as a string.

- `get_precision() -> u8`  
  Gets the number of decimal places based on the currency.

- `format() -> String`  
  Returns a human-readable string like `"₦1,200.00"`.

---

### 🔁 Rounding

- `round_to_precision(&mut self)`  
  Rounds the internal amount to the currency’s default precision.

- `round_amount(raw: f64) -> i64`  
  Rounds a float value based on currency precision (internal use).

- `round_amount_with_mode(raw: f64, mode: RoundingMode) -> i64`  
  Rounds with a specific mode like `RoundHalfUp`, `RoundDown`, etc.

---

## ➗ Arithmetic Methods

### 📌 Basic

- `multiply(scalar: f64) -> Owo`  
- `divide(scalar: f64) -> Owo`  
- `percentage(percent: f64) -> Owo`  

### 🧮 With Rounding Modes

- `multiply_with_mode(scalar: f64, mode: RoundingMode) -> Owo`  
- `divide_with_mode(scalar: f64, mode: RoundingMode) -> Owo`  
- `percentage_with_mode(percent: f64, mode: RoundingMode) -> Owo`  

---

## 📚 Comparison

- `eq(rhs: &Self) -> bool`  
- `lt(rhs: &Self) -> bool`  
- `gt(rhs: &Self) -> bool`  

Implements `PartialEq` and `PartialOrd` traits as well.

---

## ⚙️ Operator Overloading

You can use arithmetic operators directly with `Owo`:

- `+`, `-`, `*`, `/`, unary `-`



---

## 🧵 Batch Operations (`Vec<Owo>`)

The `BatchOperations` trait is implemented for `Vec<Owo>`:

- `multiply_all(scalar: f64) -> Vec<Owo>`
- `divide_all(scalar: f64) -> Vec<Owo>`
- `percentage_all(percent: f64) -> Vec<Owo>`

All have `_with_mode(...)` versions to support rounding strategy.




---

## 📚 Examples


```rust
let price = Owo::new(5000, Currency::NGN); // ₦50.00
let discounted = price.percentage(10.0); // ₦5.00
println!("{}", discounted.format()); // "₦5.00"
```

---

## 🧪 Testing

You can run tests to validate functionality:

```bash
cargo test
```

---

## 🛠️ Dependencies

- [`serde`](https://crates.io/crates/serde) for JSON serialization
- `Currency`, `RoundingMode`, and `OwoError` implementations

---

## 📦 Integration

You can use `Owo` as part of your domain-driven design, financial models, or event-sourced accounting systems.

---

## 📄 License

MIT or Apache-2.0

---

## 📝 TODOs / Future Enhancements

- Currency conversion
- Support for locale-aware formatting
- Overflow-safe arithmetic
- Decimal support for display formatting

---








