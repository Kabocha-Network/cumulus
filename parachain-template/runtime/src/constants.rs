// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use frame_support::weights::{constants::WEIGHT_PER_SECOND, Weight};
use sp_runtime::{Perbill, Perquintill};

use crate::{Balance, BlockNumber};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
// Julian year as Substrate handles it
pub const BLOCKS_PER_YEAR: BlockNumber = DAYS * 36525 / 100;

pub const MIN_VESTED_TRANSFER_AMOUNT: Balance = 100 * MILLIUNIT;
pub const MAX_COLLATOR_STAKE: Balance = 200_000 * UNIT;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

// 1 in 4 blocks (on average, not counting collisions) will be primary babe
// blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

/// Inflation configuration which is used at genesis
pub const INFLATION_CONFIG: (Perquintill, Perquintill, Perquintill, Perquintill) = (
	// max collator staking rate
	Perquintill::from_percent(40),
	// collator reward rate
	Perquintill::from_percent(10),
	// max delegator staking rate
	Perquintill::from_percent(10),
	// delegator reward rate
	Perquintill::from_percent(8),
);

pub mod staking {
	use sp_runtime::Perquintill;

	use super::KILT;
	#[cfg(not(feature = "fast-gov"))]
	use super::{DAYS, HOURS};
	use crate::{Balance, BlockNumber};

	/// Minimum round length is 1 hour (300 * 12 second block times)
	#[cfg(feature = "fast-gov")]
	pub const MIN_BLOCKS_PER_ROUND: BlockNumber = 10;
	#[cfg(not(feature = "fast-gov"))]
	pub const MIN_BLOCKS_PER_ROUND: BlockNumber = HOURS;

	#[cfg(feature = "fast-gov")]
	pub const DEFAULT_BLOCKS_PER_ROUND: BlockNumber = 20;
	#[cfg(not(feature = "fast-gov"))]
	pub const DEFAULT_BLOCKS_PER_ROUND: BlockNumber = 2 * HOURS;

	#[cfg(feature = "fast-gov")]
	pub const STAKE_DURATION: BlockNumber = 30;
	#[cfg(not(feature = "fast-gov"))]
	pub const STAKE_DURATION: BlockNumber = 7 * DAYS;

	#[cfg(feature = "fast-gov")]
	pub const MIN_COLLATORS: u32 = 4;
	#[cfg(not(feature = "fast-gov"))]
	pub const MIN_COLLATORS: u32 = 16;

	#[cfg(feature = "fast-gov")]
	pub const MAX_CANDIDATES: u32 = 16;
	#[cfg(not(feature = "fast-gov"))]
	pub const MAX_CANDIDATES: u32 = 75;

	pub const MIN_DELEGATOR_STAKE: Balance = 20 * UNIT;

	pub const NETWORK_REWARD_RATE: Perquintill = Perquintill::from_percent(10);
}
