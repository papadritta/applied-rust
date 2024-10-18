//! Colorized output utilities for the terminal using ANSI escape codes.
//!
//! This module provides functions and structs for colorizing text output in the terminal using ANSI escape codes.
//! It includes functions to generate ANSI escape codes for different colors and text styles, as well as a struct
//! to represent a colorized string.
//!
//! # Examples
//!
//! ```
//! use cli_utils::colors::*;
//!
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```
//!
//! The above example demonstrates how to use the color functions to generate colorized strings and print them to the terminal.
//!
//! # Code
//!
//! ```rust
//! pub fn red(s: &str) -> String {
//!     format!("\x1b[31m{}\x1b[0m", s)
//! }
//!
//! pub fn green(s: &str) -> String {
//!     format!("\x1b[32m{}\x1b[0m", s)
//! }
//!
//! pub fn blue(s: &str) -> String {
//!     format!("\x1b[34m{}\x1b[0m", s)
//! }
//!
//! pub fn bold(s: &str) -> String {
//!     format!("\x1b[1m{}\x1b[0m", s)
//! }
//!
//! pub fn reset(s: &str) -> String {
//!     format!("\x1b[0m{}\x1b[0m", s)
//! }
//!
//! pub enum Color {
//!     Red,
//!     Green,
//!     Blue,
//!     Bold,
//! }
//!
//! pub struct ColorString {
//!     pub colors: Color,
//!     pub string: String,
//!     pub colorized: String,
//! }
//!
//! impl ColorString {
//!     /// Paints the colorized string based on the color field.
//!     ///
//!     /// This method takes the value of the `color` field and applies the corresponding color to the `string` field,
//!     /// generating a colorized string and assigning it to the `colorized` field.
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```
//!     /// use cli_utils::colors::*;
//!     ///
//!     /// let mut color_string = ColorString {
//!     ///     color: Color::Red,
//!     ///     string: String::from("Hello, world!"),
//!     ///     colorized: String::new(),
//!     /// };
//!     ///
//!     /// color_string.paint();
//!     ///
//!     /// assert_eq!(color_string.colorized, red("Hello, world!"));
//!     /// ```
//!     pub fn paint(&mut self) {
//!         match self.color {
//!             Color::Red => self.colorized = red(&self.string),
//!             Color::Green => self.colorized = green(&self.string),
//!             Color::Blue => self.colorized = blue(&self.string),
//!             Color::Bold => self.colorized = bold(&self.string),
//!         };
//!     }
//!
//!     /// Resets the colorized string to its original state.
//!     ///
//!     /// This method resets the `colorized` field to the original `string` value, removing any applied color or style.
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```
//!     /// use cli_utils::colors::*;
//!     ///
//!     /// let mut color_string = ColorString {
//!     ///     color: Color::Red,
//!     ///     string: String::from("Hello, world!"),
//!     ///     colorized: String::new(),
//!     /// };
//!     ///
//!     /// color_string.paint();
//!     /// color_string.reset();
//!     ///
//!     /// assert_eq!(color_string.colorized, color_string.string);
//!     /// ```
//!     pub fn reset(&mut self) {
//!         self.colorized = reset(&self.string);
//!     }
//! }
//! ```
