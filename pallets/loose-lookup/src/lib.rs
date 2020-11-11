#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use primitivesv1::multiaddress::MultiAddress;
use sp_runtime::traits::{LookupError, Saturating, 
	//StaticLookup
};
use frame_support::{decl_module, decl_error, decl_event, decl_storage, ensure, RuntimeDebug, debug};
use frame_support::dispatch::DispatchResult;
use frame_support::traits::{
	Currency, ReservableCurrency, Get, EnsureOrigin, OnUnbalanced,
	WithdrawReason, ExistenceRequirement::KeepAlive, Imbalance,
};
use frame_system::ensure_signed;
use codec::{Encode, Decode};
use name_service::StaticLookup;

pub trait Trait: frame_system::Trait {
	// type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type Lookie: StaticLookup <Target = Self::AccountId> + StaticLookup <Source = MultiAddress<Self::AccountId, <Self as frame_system::Trait>::AccountIndex>> ;  
}

// decl_event!(
//     pub enum Event<T>
//     where
//         Source = <T as system::Trait>::
// 

decl_storage! {
	trait Store for Module<T: Trait> as SimpleMap {
		SimpleMap get(fn simple_map): map hasher(blake2_128_concat) T::AccountId => u32;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		LookupErrorororor,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// fn deposit_event() = default;

		// Checks whether the caller is a member of the set of account IDs provided by the
		// MembershipSource type. Emits an event if they are, and errors if not.
		#[weight = 10_000]
		fn check_name(origin, xxx: 
			//MultiAddress::Address32([u8; 32])
			<T::Lookie as StaticLookup>::Source,
			account: <T::Lookie as StaticLookup>::Source
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			// Get find the AccountId of the 
			let target_from_name = T::Lookie::lookup(xxx)?;
			let target_from_account = T::Lookie::lookup(account)?;

			match target_from_name {
				target_from_account => <SimpleMap<T>>::insert(&target_from_account,55)
			}

            // debug::info!("this account's name is: {:?}", name);
			//ensure!(caller == name, Error::<T>::LookupErrorororor);

			Ok(())
		}
	}
}