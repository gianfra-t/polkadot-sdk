// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_885 nanoseconds.
		Weight::from_ref_time(11_162_365)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 2_439
			.saturating_add(Weight::from_ref_time(4_637_012).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_203 nanoseconds.
		Weight::from_ref_time(5_441_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_067 nanoseconds.
		Weight::from_ref_time(11_552_327)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 2_783
			.saturating_add(Weight::from_ref_time(4_935_647).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_999 nanoseconds.
		Weight::from_ref_time(9_297_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_876 nanoseconds.
		Weight::from_ref_time(10_175_160)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 3_709
			.saturating_add(Weight::from_ref_time(4_685_640).saturating_mul(c.into()))
	}
}
