// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Determine if a `char` is a valid identifier for a parser and/or lexer
//! according to [Unicode Standard Annex #31](http://www.unicode.org/reports/tr31/) rules.
//!
//! ```rust
//! use unicode_id::UnicodeID;
//!
//! fn main() {
//! 	let ch = 'a';
//! 	println!("Is {} a valid start of an identifier? {}", ch, UnicodeID::is_id_start(ch));
//! }
//! ```
//!
//! # features
//!
//! unicode-id supports a `no_std` feature. This eliminates dependence
//! on std, and instead uses equivalent functions from core.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc(
	html_logo_url = "https://unicode-rs.github.io/unicode-rs_sm.png",
	html_favicon_url = "https://unicode-rs.github.io/unicode-rs_sm.png"
)]
#![no_std]
#![cfg_attr(feature = "bench", feature(test, unicode_internals))]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(feature = "bench")]
extern crate test;

pub use tables::UNICODE_VERSION;
use tables::derived_property;

mod tables;

#[cfg(test)]
mod tests;

/// Methods for determining if a character is a valid identifier character.
pub trait UnicodeID {
	/// Returns whether the specified character satisfies the 'ID_Start'
	/// Unicode property.
	///
	/// 'ID_Start' is a Unicode Derived Property specified in
	/// [UAX #31](http://unicode.org/reports/tr31/#NFKC_Modifications),
	/// mostly similar to ID_Start but modified for closure under NFKx.
	fn is_id_start(self) -> bool;

	/// Returns whether the specified `char` satisfies the 'ID_Continue'
	/// Unicode property.
	///
	/// 'ID_Continue' is a Unicode Derived Property specified in
	/// [UAX #31](http://unicode.org/reports/tr31/#NFKC_Modifications),
	/// mostly similar to 'ID_Continue' but modified for closure under NFKx.
	fn is_id_continue(self) -> bool;
}

impl UnicodeID for char {
	#[inline]
	fn is_id_start(self) -> bool {
		// Fast-path for ascii idents
		('a' <= self && self <= 'z')
			|| ('A' <= self && self <= 'Z')
			|| (self > '\x7f' && derived_property::ID_Start(self))
	}

	#[inline]
	fn is_id_continue(self) -> bool {
		// Fast-path for ascii idents
		('a' <= self && self <= 'z')
			|| ('A' <= self && self <= 'Z')
			|| ('0' <= self && self <= '9')
			|| self == '_'
			|| (self > '\x7f' && derived_property::ID_Continue(self))
	}
}
