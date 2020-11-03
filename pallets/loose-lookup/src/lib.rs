#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use primitivesv1::multiaddress::MultiAddress;
use sp_runtime::traits::{LookupError, Saturating, StaticLookup};
use frame_support::{decl_module, decl_error, decl_event, decl_storage, ensure, RuntimeDebug, debug};
use frame_support::dispatch::DispatchResult;
use frame_support::traits::{
	Currency, ReservableCurrency, Get, EnsureOrigin, OnUnbalanced,
	WithdrawReason, ExistenceRequirement::KeepAlive, Imbalance,
};
use frame_system::ensure_signed;
use codec::{Encode, Decode};
// use name_service::StaticLookup;

pub trait Trait: frame_system::Trait {
	// type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	// type Lookie: StaticLookup<Source = Self::AccountId>;
	type Lookie: StaticLookup;     
}

// decl_event!(
//     pub enum Event<T>
//     where
//         Source = <T as system::Trait>::
// )

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// fn deposit_event() = default;

		/// Checks whether the caller is a member of the set of account IDs provided by the
		/// MembershipSource type. Emits an event if they are, and errors if not.
		#[weight = 10_000]
		fn check_name(origin) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			// Get the members from the `vec-set` pallet
            let name = T::Lookie::lookup(caller);
            //debug::info!("this account's name is: {:?}", name);

			// Check whether the caller is a member
			//ensure!(members.contains(&caller), Error::<T>::NotAMember);

			// If the previous call didn't error, then the caller is a member, so emit the event
			//Self::deposit_event(RawEvent::IsAMember(caller));
			Ok(())
		}
	}
}