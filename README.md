# Cowry Lab 💸

> A Rust micro-library for money, currency math, and precision control.

[![CI](https://github.com/matt-kay/cowry_lab/actions/workflows/ci.yml/badge.svg)](https://github.com/matt-kay/cowry_lab/actions)
[![Docs.rs](https://docs.rs/cowry_lab/badge.svg)](https://docs.rs/cowry_lab)
[![Crates.io](https://img.shields.io/crates/v/cowry_lab.svg)](https://crates.io/crates/cowry_lab)

---

**Cowry Lab** gives you easy, accurate, and expressive currency operations in Rust:

```rust
use cowry_lab::{Owo, Currency};

let ngn = Currency::new("NGN", "₦", 2);
let items = vec![
    Owo::new(1000, ngn.clone()),
    Owo::new(500, ngn.clone()),
];

let taxed = items.percentage_all(7.5);


## Cowry - Function Categories

## 🧱 **Core Construction & Accessors**
These functions are fundamental to creating and accessing monetary values.

| Function              | Description |
|-----------------------|-------------|
| `Cowry::new`          | Creates a new Cowry instance with amount, currency, and precision. |
| `Cowry::get_amount()` | Returns the raw amount in the smallest currency unit (e.g., cents). |
| `Cowry::to_object()`  | Returns a struct version of the Cowry object. |
| `Cowry::from_object()`| Creates a Cowry from a JSON-like object. |
| `Cowry::from_json()`  | Parses a Cowry instance from a JSON string. |
| `Cowry::to_json()`    | Serializes Cowry into a JSON string. |

---

## 🔢 **Arithmetic Operations**
Used for performing basic mathematical operations on monetary values.

| Function              | Description |
|-----------------------|-------------|
| `add()`               | Adds another Cowry to this one. |
| `subtract()`          | Subtracts another Cowry from this one. |
| `multiply(f64)`       | Multiplies the amount by a scalar. |
| `divide(f64)`         | Divides the amount by a scalar. |
| `percentage(u8)`      | Returns a Cowry representing a given percentage of the amount. |

---

## 📦 **Allocation & Distribution**
Functions for splitting amounts.

| Function              | Description |
|-----------------------|-------------|
| `allocate([u32])`     | Splits the amount based on given ratios into multiple Cowry objects. |

---

## 🔁 **Comparison**
Used to compare Cowry values.

| Function              | Description |
|-----------------------|-------------|
| `equals_to()`         | Checks if two Cowry objects have the same amount and currency. |
| `has_same_currency()` | Checks if two Cowry objects have the same currency. |
| `has_same_amount()`   | Checks if two Cowry objects have the same amount. |

---

## 📉 **Minimum & Maximum**
Used to compare multiple Cowry instances.

| Function              | Description |
|-----------------------|-------------|
| `minimum(&[Cowry])`   | Returns the Cowry with the smallest amount. |
| `maximum(&[Cowry])`   | Returns the Cowry with the largest amount. |

---

## 🔍 **Checks & Flags**
Boolean checks for inspecting value state.

| Function              | Description |
|-----------------------|-------------|
| `is_zero()`           | Checks if the amount is zero. |
| `is_positive()`       | Checks if the amount is positive. |
| `is_negative()`       | Checks if the amount is negative. |
| `has_sub_units()`     | Checks if the amount has sub-units (like cents). |

---

## 💱 **Currency Conversion**
Support for converting between currencies using global rates.

| Function              | Description |
|-----------------------|-------------|
| `convert(to: &str)`   | Converts the Cowry to another currency using global API. |
| `Cowry::set_global_exchange_api()` | Sets global API configuration for currency conversion. |

---

## ⚙️ **Global Configuration**
Handles global behavior through configuration (e.g., in TOML).

| Field / Method                        | Description |
|---------------------------------------|-------------|
| `default_amount`                      | Default amount if none is specified. |
| `default_currency`                    | Default currency code (e.g. USD). |
| `default_precision`                   | Number of decimal places. |
| `global_locale`                       | Global locale setting for formatting. |
| `global_format`                       | Global string format pattern. |
| `global_rounding_mode`                | Mode used for rounding (e.g., round half up). |
| `global_format_rounding_mode`         | Rounding mode for formatted output. |
| `global_exchange_rates_api`           | Global exchange rate API config (endpoint, headers, property path). |

---

## 🧾 **Formatting & Representation**
For outputting monetary values in various formats.

| Function              | Description |
|-----------------------|-------------|
| `to_unit()`           | Converts amount to base unit (e.g., dollars). |
| `to_rounded_unit()`   | Converts amount to base unit and rounds it. |
| `to_format()`         | Formats the amount based on a format string and locale. |

---

## 🔧 **Additional Utility Functions**
These functions provide extra functionality, such as error handling, logging, or custom calculations.

| Function              | Description |
|-----------------------|-------------|
| `toObject()`          | Serializes the Cowry instance into an object for easy manipulation. |
| `fromObject()`        | Creates a Cowry instance from an object (useful for parsing JSON-like data). |
| `parse()`             | Parses a string value into a Cowry object. |
```
