//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl pallet_kitties::WeightInfo for WeightInfo {
	fn create() -> Weight {
		(877_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn breed() -> Weight {
		(1_069_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn transfer() -> Weight {
		(683_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn set_price() -> Weight {
		(579_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn buy() -> Weight {
		(1_950_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
}
