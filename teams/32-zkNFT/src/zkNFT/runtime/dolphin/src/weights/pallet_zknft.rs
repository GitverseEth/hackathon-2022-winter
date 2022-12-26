// Copyright 2020-2022 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_zknft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dolphin-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=dolphin-dev
// --steps=50
// --repeat=20
// --pallet=pallet_zknft
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_zknft.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_zknft.
pub trait WeightInfo {
    fn to_private() -> Weight;
    fn to_public() -> Weight;
    fn private_transfer() -> Weight;
    fn public_transfer() -> Weight;
}

/// Weights for pallet_zknft using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_zknft::WeightInfo for SubstrateWeight<T> {
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Zknft UtxoSet (r:1 w:1)
    // Storage: Zknft NullifierSetSize (r:1 w:0)
    // Storage: Zknft ShardTrees (r:1 w:1)
    // Storage: Zknft UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: Zknft Shards (r:0 w:1)
    fn to_private() -> Weight {
        (40_529_218_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Zknft UtxoAccumulatorOutputs (r:2 w:1)
    // Storage: Zknft NullifierCommitmentSet (r:2 w:2)
    // Storage: Zknft UtxoSet (r:1 w:1)
    // Storage: Zknft NullifierSetSize (r:1 w:1)
    // Storage: Zknft ShardTrees (r:1 w:1)
    // Storage: Zknft NullifierSetInsertionOrder (r:0 w:2)
    // Storage: Zknft Shards (r:0 w:1)
    fn to_public() -> Weight {
        (53_595_142_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(12 as Weight))
    }
    // Storage: Zknft UtxoAccumulatorOutputs (r:2 w:2)
    // Storage: Zknft NullifierCommitmentSet (r:2 w:2)
    // Storage: Zknft UtxoSet (r:2 w:2)
    // Storage: Zknft NullifierSetSize (r:1 w:1)
    // Storage: Zknft ShardTrees (r:2 w:2)
    // Storage: Zknft NullifierSetInsertionOrder (r:0 w:2)
    // Storage: Zknft Shards (r:0 w:2)
    fn private_transfer() -> Weight {
        (71_453_360_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(13 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    fn public_transfer() -> Weight {
        (44_712_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Zknft UtxoSet (r:1 w:1)
    // Storage: Zknft NullifierSetSize (r:1 w:0)
    // Storage: Zknft ShardTrees (r:1 w:1)
    // Storage: Zknft UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: Zknft Shards (r:0 w:1)
    fn to_private() -> Weight {
        (40_529_218_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(7 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Zknft UtxoAccumulatorOutputs (r:2 w:1)
    // Storage: Zknft NullifierCommitmentSet (r:2 w:2)
    // Storage: Zknft UtxoSet (r:1 w:1)
    // Storage: Zknft NullifierSetSize (r:1 w:1)
    // Storage: Zknft ShardTrees (r:1 w:1)
    // Storage: Zknft NullifierSetInsertionOrder (r:0 w:2)
    // Storage: Zknft Shards (r:0 w:1)
    fn to_public() -> Weight {
        (53_595_142_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(10 as Weight))
            .saturating_add(RocksDbWeight::get().writes(12 as Weight))
    }
    // Storage: Zknft UtxoAccumulatorOutputs (r:2 w:2)
    // Storage: Zknft NullifierCommitmentSet (r:2 w:2)
    // Storage: Zknft UtxoSet (r:2 w:2)
    // Storage: Zknft NullifierSetSize (r:1 w:1)
    // Storage: Zknft ShardTrees (r:2 w:2)
    // Storage: Zknft NullifierSetInsertionOrder (r:0 w:2)
    // Storage: Zknft Shards (r:0 w:2)
    fn private_transfer() -> Weight {
        (71_453_360_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(13 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    fn public_transfer() -> Weight {
        (44_712_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
}
