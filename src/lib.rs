#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    #[pallet::getter(fn something)]

    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Addition { value: u32, who: T::AccountId },
        Substraction { value: u32, who: T::AccountId },
        Multiply { value: u32, who: T::AccountId },
        Divide { value: u32, who: T::AccountId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        CannotSub,
        CannotDivide,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight({10000})]
        pub fn do_add(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let something = Something::<T>::get().unwrap_or_default();
            let new_value = something + value;
            // Update storage.
            <Something<T>>::put(new_value);

            // Emit an event.
            Self::deposit_event(Event::Addition { value, who });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }
        #[pallet::call_index(1)]
        #[pallet::weight({10000})]
        pub fn do_sub(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let something = Something::<T>::get().unwrap_or_default();
            let new_value = something.checked_sub(value);
            if let Some(value) = new_value {
                <Something<T>>::put(value);
                // Emit an event.
                Self::deposit_event(Event::Substraction { value, who });
            } else {
                return Err(Error::<T>::CannotSub.into());
            }
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight({10000})]
        pub fn do_multiply(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let something = Something::<T>::get().unwrap_or_default();
            let new_value = something * value;
            <Something<T>>::put(new_value);

            Self::deposit_event(Event::Multiply { value, who });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight({10000})]
        pub fn do_divide(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let something = Something::<T>::get().unwrap_or_default();
            if value ==0 {
                return Err(Error::<T>::CannotDivide.into());
            }
            let new_value = something / value;
            // Update storage.
            <Something<T>>::put(new_value);

            // Emit an event.
            Self::deposit_event(Event::Divide { value, who });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }
    }
}
