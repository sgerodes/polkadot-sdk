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

#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config(with_default)]
	pub trait Config: frame_system::Config {
		type WeightInfo: WeightInfo;
	}

	pub trait WeightInfo {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call(weight = T::WeightInfo)]
	impl<T: Config> Pallet<T> {
		#[pallet::authorize(Ok(Default::default()))]
		#[pallet::weight_of_authorize(Weight::zero())]
		#[pallet::weight(Weight::zero())]
		#[pallet::call_index(0)]
		pub fn call1(origin: OriginFor<T>, a: u32) -> DispatchResult {
			let _ = origin;
			let _ = a;
			Ok(())
		}
	}
}

fn main() {}
