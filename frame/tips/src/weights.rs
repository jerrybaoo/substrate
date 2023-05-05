// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_tips
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-w5zkt3rr-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_tips
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/tips/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_tips.
pub trait WeightInfo {
	fn report_awesome(r: u32, ) -> Weight;
	fn retract_tip() -> Weight;
	fn tip_new(r: u32, t: u32, ) -> Weight;
	fn tip(t: u32, ) -> Weight;
	fn close_tip(t: u32, ) -> Weight;
	fn slash_tip(t: u32, ) -> Weight;
}

/// Weights for pallet_tips using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 300]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3469`
		// Minimum execution time: 31_203_000 picoseconds.
		Weight::from_parts(32_556_841, 3469)
			// Standard Error: 170
			.saturating_add(Weight::from_parts(2_815, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	fn retract_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3686`
		// Minimum execution time: 30_530_000 picoseconds.
		Weight::from_parts(31_384_000, 3686)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:0 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 300]`.
	/// The range of component `t` is `[1, 13]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `526 + t * (64 ±0)`
		//  Estimated: `3991 + t * (64 ±0)`
		// Minimum execution time: 20_590_000 picoseconds.
		Weight::from_parts(21_295_759, 3991)
			// Standard Error: 126
			.saturating_add(Weight::from_parts(1_652, 0).saturating_mul(r.into()))
			// Standard Error: 3_017
			.saturating_add(Weight::from_parts(67_547, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(t.into()))
	}
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 13]`.
	fn tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `747 + t * (112 ±0)`
		//  Estimated: `4212 + t * (112 ±0)`
		// Minimum execution time: 16_502_000 picoseconds.
		Weight::from_parts(16_856_669, 4212)
			// Standard Error: 5_253
			.saturating_add(Weight::from_parts(234_492, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 112).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 13]`.
	fn close_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `786 + t * (112 ±0)`
		//  Estimated: `4242 + t * (112 ±0)`
		// Minimum execution time: 64_341_000 picoseconds.
		Weight::from_parts(66_566_361, 4242)
			// Standard Error: 12_633
			.saturating_add(Weight::from_parts(229_718, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 112).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 13]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 14_915_000 picoseconds.
		Weight::from_parts(15_713_480, 3734)
			// Standard Error: 2_761
			.saturating_add(Weight::from_parts(18_808, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 300]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3469`
		// Minimum execution time: 31_203_000 picoseconds.
		Weight::from_parts(32_556_841, 3469)
			// Standard Error: 170
			.saturating_add(Weight::from_parts(2_815, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	fn retract_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3686`
		// Minimum execution time: 30_530_000 picoseconds.
		Weight::from_parts(31_384_000, 3686)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:0 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `r` is `[0, 300]`.
	/// The range of component `t` is `[1, 13]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `526 + t * (64 ±0)`
		//  Estimated: `3991 + t * (64 ±0)`
		// Minimum execution time: 20_590_000 picoseconds.
		Weight::from_parts(21_295_759, 3991)
			// Standard Error: 126
			.saturating_add(Weight::from_parts(1_652, 0).saturating_mul(r.into()))
			// Standard Error: 3_017
			.saturating_add(Weight::from_parts(67_547, 0).saturating_mul(t.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(t.into()))
	}
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 13]`.
	fn tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `747 + t * (112 ±0)`
		//  Estimated: `4212 + t * (112 ±0)`
		// Minimum execution time: 16_502_000 picoseconds.
		Weight::from_parts(16_856_669, 4212)
			// Standard Error: 5_253
			.saturating_add(Weight::from_parts(234_492, 0).saturating_mul(t.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 112).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 13]`.
	fn close_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `786 + t * (112 ±0)`
		//  Estimated: `4242 + t * (112 ±0)`
		// Minimum execution time: 64_341_000 picoseconds.
		Weight::from_parts(66_566_361, 4242)
			// Standard Error: 12_633
			.saturating_add(Weight::from_parts(229_718, 0).saturating_mul(t.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 112).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None, mode: Measured)
	/// The range of component `t` is `[1, 13]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 14_915_000 picoseconds.
		Weight::from_parts(15_713_480, 3734)
			// Standard Error: 2_761
			.saturating_add(Weight::from_parts(18_808, 0).saturating_mul(t.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
