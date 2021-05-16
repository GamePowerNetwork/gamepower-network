// This pallet use The Open Runtime Module Library (ORML) which is a community maintained collection of Substrate runtime modules.
// Thanks to all contributors of orml.
// https://github.com/open-web3-stack/open-runtime-module-library

// This pallet also uses code from the Bit-Country-Blockchain
// https://github.com/bit-country/Bit-Country-Blockchain

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]


use codec::{Decode, Encode};
use frame_support::{
	traits::{
		Currency,
		ReservableCurrency,
	},
  transactional,
};
//use orml_traits::NFT;
use primitives::{AssetId, SeriesId};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
  traits::{One},
	RuntimeDebug, ModuleId,
};
use sp_std::vec::Vec;
pub use pallet::*;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SeriesData<AccountId> {
    pub name: Vec<u8>,
    pub owner: AccountId,
    pub metadata: Vec<u8>,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData<Balance> {
    pub deposit: Balance,
    pub metadata: Vec<u8>,
    pub asset_type: AssetType,
    pub collection_type: CollectionType,
    pub total_supply: u64,
    pub initial_supply: u64,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetData<Balance> {
    pub deposit: Balance,
    pub name: Vec<u8>,
    pub metadata: Vec<u8>,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AssetType {
    Transferrable,
    BoundToAddress,
}

impl AssetType {
    pub fn is_transferrable(&self) -> bool {
        match *self {
          AssetType::Transferrable => true,
            _ => false,
        }
    }
}

impl Default for AssetType {
    fn default() -> Self {
      AssetType::Transferrable
    }
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CollectionType {
    Collectable,
    Consumable,
    Wearable,
    Usable,
}

//Collection extension for fast retrieval
impl CollectionType {
    pub fn is_collectable(&self) -> bool {
        match *self {
            CollectionType::Collectable => true,
            _ => false,
        }
    }

    pub fn is_consumable(&self) -> bool {
        match *self {
            CollectionType::Consumable => true,
            _ => false,
        }
    }

    pub fn is_wearable(&self) -> bool {
        match *self {
            CollectionType::Wearable => true,
            _ => false,
        }
    }

    pub fn is_usable(&self) -> bool {
      match *self {
          CollectionType::Usable => true,
          _ => false,
      }
  }
}

impl Default for CollectionType {
    fn default() -> Self {
        CollectionType::Collectable
    }
}

type SeriesDataOf<T> = <T as frame_system::Config>::AccountId;
type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[frame_support::pallet]
pub mod pallet {
  use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;



	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: 
    frame_system::Config
    + orml_nft::Config<ClassData = ClassData<BalanceOf<Self>>, TokenData = AssetData<BalanceOf<Self>>>
  {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// The minimum balance to create class
		#[pallet::constant]
		type CreateClassDeposit: Get<BalanceOf<Self>>;

		/// The minimum balance to create token
		#[pallet::constant]
		type CreateAssetDeposit: Get<BalanceOf<Self>>;

    // Currency type for reserve/unreserve balance
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    /// The NFT's module id
		#[pallet::constant]
		type ModuleId: Get<ModuleId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


  
	#[pallet::storage]
	#[pallet::getter(fn get_asset)]
	pub(super) type Assets<T: Config> = StorageMap<_, Blake2_128Concat, AssetId, Option<(ClassIdOf<T>, TokenIdOf<T>)>, ValueQuery>;

  #[pallet::storage]
	#[pallet::getter(fn get_assets_by_owner)]
  pub(super) type AssetsByOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<AssetId>, ValueQuery>;

  #[pallet::storage]
	#[pallet::getter(fn get_series)]
  pub(super) type Series<T: Config> = StorageMap<_, Blake2_128Concat, SeriesId, Option<SeriesDataOf<T>>>;

  #[pallet::storage]
	#[pallet::getter(fn get_class_series)]
  pub(super) type ClassDataSeries<T: Config> = StorageMap<_, Blake2_128Concat, ClassIdOf<T>, SeriesId, ValueQuery>;

  #[pallet::storage]
	#[pallet::getter(fn next_series_id)]
	pub(super) type NextSeriesId<T: Config> = StorageValue<_, u64, ValueQuery>;

  #[pallet::storage]
	#[pallet::getter(fn all_series_count)]
	pub(super) type AllSeries<T: Config> = StorageValue<_, u64, ValueQuery>;

  #[pallet::storage]
	#[pallet::getter(fn get_class_type)]
  pub(super) type ClassDataType<T: Config> = StorageMap<_, Blake2_128Concat, ClassIdOf<T>, AssetType, ValueQuery>;

  #[pallet::storage]
	#[pallet::getter(fn next_asset_id)]
	pub(super) type NextAssetId<T: Config> = StorageValue<_, AssetId, ValueQuery>;
  


	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// parameters. [owner, series]
		NewSeriesCreated(T::AccountId, SeriesId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		AssetIdNotFound,
		/// Errors should have helpful documentation associated with them.
		NoPermission,
    //No available series id
    NoAvailableSeriesId,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_series(origin: OriginFor<T>, name: Vec<u8>, metadata: Vec<u8>) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

      let next_series_id = Self::do_create_series(&sender, name.clone(), metadata.clone())?;

      let all_series_count = Self::all_series_count();
      let new_all_series_count = all_series_count.checked_add(One::one())
        .ok_or("Overflow adding new collection to total collection")?;

      AllSeries::<T>::put(new_all_series_count);

      Self::deposit_event(Event::NewSeriesCreated(sender, next_series_id));

			Ok(().into())
		}
	}

  impl<T: Config> Pallet<T> {
    #[transactional]
    fn do_create_series(
      sender: &T::AccountId, 
      name: Vec<u8>, 
      metadata: Vec<u8>
    ) -> Result<SeriesId, DispatchError> {
      let next_series_id = NextSeriesId::<T>::try_mutate(
        |series_id| -> Result<SeriesId, DispatchError> {
          let current_id = *series_id;

          *series_id = series_id
            .checked_add(One::one())
            .ok_or(Error::<T>::NoAvailableSeriesId)?;

          Ok(current_id)
        }
      )?;

      let series_data = SeriesData::<T::AccountId> {
        name,
        owner: sender.clone(),
        metadata,
      };

      <Series<T>>::insert(next_series_id, series_data);

      Ok(next_series_id)
    }
  }
}