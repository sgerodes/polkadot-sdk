// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg(test)]


use crate::*;
use sp_core::crypto::Ss58AddressFormat;
use std::collections::HashMap;



/// Tests parsing SS58 address formats from known network names.
/// This ensures that valid SS58 network names correctly map to their expected format.
#[test]
fn parse_ss58_address_format_from_registry() {
	let expected_formats: HashMap<&str, u16> = [
		("polkadot", 0),
		("kusama", 2),
		("substrate", 42),
		("zeitgeist", 73),
	]
		.iter()
		.cloned()
		.collect();

	for (name, expected) in &expected_formats {
		assert_eq!(
			parse_ss58_address_format(name),
			Ok(Ss58AddressFormat::from(*expected)),
			"Failed for network: {}",
			name
		);
	}
}

/// Tests parsing SS58 address formats from numeric string inputs.
/// This verifies that numeric representations of SS58 formats are correctly recognized.
#[test]
fn parse_ss58_address_custom_format_from_u16() {
	let expected_formats: HashMap<&str, u16> = [
		("0", 0),
		("42", 42),
		("355", 355),
	]
		.iter()
		.cloned()
		.collect();

	for (name, expected) in &expected_formats {
		assert_eq!(
			parse_ss58_address_format(name),
			Ok(Ss58AddressFormat::from(*expected)),
			"Failed for network: {}",
			name
		);
	}
}

/// Tests parsing SS58 address formats with invalid inputs.
/// This ensures that incorrect or malformed inputs return errors as expected.
#[test]
fn parse_ss58_address_invalid_inputs() {
	let invalid_inputs = [
		"-42",          // Negative numbers are not valid SS58 formats
		"abc123",       // Random strings should fail
		"polka_dot",    // Misspelled network names should fail
		"99999999",     // Out-of-range number (SS58 formats are u16)
		"",             // Empty string should fail
		"!@#$%^&*()",   // Special characters should fail
	];

	for &name in &invalid_inputs {
		assert!(
			parse_ss58_address_format(name).is_err(),
			"Expected error for invalid input: {}",
			name
		);
	}
}