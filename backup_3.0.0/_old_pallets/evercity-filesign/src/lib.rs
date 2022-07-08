#![allow(clippy::unused_unit)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]    
mod tests;
pub mod file;
use file::{FileStruct, H256, FileId};
use frame_support::traits::Randomness;
use codec::Encode;
use frame_support::traits::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
        use frame_support::{pallet_prelude::*, dispatch::{DispatchResultWithPostInfo, DispatchResult}};
        use frame_system::pallet_prelude::*;
        use super::*;

        #[pallet::config]
        pub trait Config: frame_system::Config {
		    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
            type Randomness: frame_support::traits::Randomness<Self::Hash>;
        }

        #[pallet::pallet]
        #[pallet::generate_store(pub(super) trait Store)]
        pub struct Pallet<T>(PhantomData<T>);

        #[pallet::hooks]
        impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { }

        #[pallet::call]
        impl<T: Config> Pallet<T>
        {
            #[pallet::weight (T::DbWeight::get().reads_writes(2, 1) + 10_000)]
            pub fn create_new_file(origin: OriginFor<T>, tag: Vec<u8>, filehash: H256, file_id_option: Option<FileId>) 
                -> DispatchResultWithPostInfo {
                ensure!(!tag.is_empty(), Error::<T>::EmptyTag);
                let caller = ensure_signed(origin)?;
                
                // Update last created file ID
                let file_id = match file_id_option {
                    Some(id) => id,
                    None => Self::get_random_id()
                };
                ensure!(<FileByID<T>>::get(file_id).is_none(), Error::<T>::IdAlreadyExists);
                let new_file = FileStruct::<<T as frame_system::Config>::AccountId>::new(caller.clone(), file_id, tag, &filehash);
                <FileByID<T>>::insert(file_id, new_file);
                Self::deposit_event(Event::<T>::FileCreated(caller, file_id));
                Ok(().into())
            }

            #[pallet::weight(T::DbWeight::get().reads_writes(1, 1) + 10_000)]
            pub fn sign_latest_version(origin: OriginFor<T>, id: FileId) 
                -> DispatchResultWithPostInfo {
                let caller = ensure_signed(origin)?;
                FileByID::<T>::try_mutate(
                    id, |file_option| -> DispatchResult {
                        match file_option {
                            None => return Err(Error::<T>::FileNotFound.into()),
                            Some(file) => {
                                ensure!(file.signers.iter().any(|x| *x == caller), Error::<T>::AddressNotSigner);
                                file.sign_latest_version(caller.clone());
                            }
                        }
                        Ok(())
                    })?;

                Self::deposit_event(Event::<T>::FileSigned(caller, id));
                Ok(().into())
            }

            #[pallet::weight(T::DbWeight::get().reads_writes(1, 1) + 10_000)]
            pub fn delete_signer(origin: OriginFor<T>, id: FileId, signer: T::AccountId)
                -> DispatchResultWithPostInfo  {
                let caller = ensure_signed(origin)?;

                FileByID::<T>::try_mutate(
                    id, |file_option| -> DispatchResult {
                        match file_option {
                            None => return Err(Error::<T>::FileNotFound.into()),
                            Some(file) => {
                                ensure!(file.owner == caller, Error::<T>::AddressNotOwner);
                                ensure!(file.signers.iter().any(|x| *x == signer), Error::<T>::AddressNotSigner);
                                ensure!(file.delete_signer_from_file(signer.clone()).is_ok(), 
                                    Error::<T>::AddressNotSigner);
                            }
                        }
                        Ok(())
                    }
                )?;

                Self::deposit_event(Event::<T>::SignerDeleted(caller, id, signer));
                Ok(().into())
            }

            #[pallet::weight(T::DbWeight::get().reads_writes(1, 1) + 10_000)]
            pub fn assign_signer(origin: OriginFor<T>, id: FileId, signer: T::AccountId)
                -> DispatchResultWithPostInfo {
                let caller = ensure_signed(origin)?;
    
                FileByID::<T>::try_mutate(
                    id, |file_option| -> DispatchResult {
                        match file_option {
                            None => return Err(Error::<T>::FileNotFound.into()),
                            Some(file) => {
                                ensure!(file.owner == caller, Error::<T>::AddressNotOwner);
                                file.assign_signer_to_file(signer.clone());
                            }
                        }
                        Ok(())
                    }
                )?;
    
                Self::deposit_event(Event::<T>::SignerAssigned(caller, id, signer));
                Ok(().into())
            }
        }

        #[pallet::event]
        #[pallet::generate_deposit(pub(super) fn deposit_event)]
	    #[pallet::metadata(T::AccountId = "AccountId")]
        pub enum Event<T: Config> {
            /// \[account, fileid, signer\]
            SignerAssigned(T::AccountId, FileId, T::AccountId),
            /// \[account, fileid\]
            FileCreated(T::AccountId, FileId),
            /// \[account, fileid, signer\]
            SignerDeleted(T::AccountId, FileId, T::AccountId),
            /// \[account, fileid\]
            FileSigned(T::AccountId, FileId),
        }

        /// Old name generated by `decl_event`.
        #[deprecated(note="use `Event` instead")]
        pub type RawEvent<T> = Event<T>;

        #[pallet::error]
        pub enum Error<T> {
            /// Address is not a signer 
            AddressNotSigner,
            /// Address is not an owner of a file
            AddressNotOwner,
            /// No such file in storage
            FileNotFound,
            /// Validation error - no tag
            EmptyTag,
            /// Validation error - no tag
            FileHasNoSigners,
            /// File id is busy
            IdAlreadyExists,
        }


        /// Storage map for file IDs
        #[pallet::storage]
        #[pallet::getter(fn file_by_id)]
        pub(super) type FileByID<T: Config> = StorageMap<_, Blake2_128Concat, FileId, FileStruct<T::AccountId>>;

        /// Nonce for random file id generating
        #[pallet::storage]
        pub(super) type NonceId<T: Config> = StorageValue<_, u64, ValueQuery>;

}


impl<T: Config> Pallet<T> {
    /// <pre>
    /// Method: address_is_auditor_for_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: FileId, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address is an auditor for the given file
    /// </pre>
    pub fn address_is_signer_for_file(id: FileId, address: &T::AccountId) -> bool {
        match FileByID::<T>::get(id) {
            None => false,
            Some(file) => file.signers.iter().any(|x| x == address)
        }
    }

    /// <pre>
    /// Method: address_has_signed_the_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: FileId, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address has signed last version of the given file
    /// </pre>
    pub fn address_has_signed_the_file(id: FileId, address: &T::AccountId) -> bool {
        match FileByID::<T>::get(id) {
            None => false,
            Some(file) => {
                if let Some(vers_strunct) = file.versions.last() {
                    return vers_strunct.signatures.iter().any(|sign| sign.address == *address && sign.signed);
                }
                false
            }
        }
    }

    /// <pre>
    /// Method: address_is_owner_for_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: FileId, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address is the owner for the given file
    /// </pre>
    pub fn address_is_owner_for_file(id: FileId, address: &T::AccountId) -> bool {
        match FileByID::<T>::get(id) {
            None => false,
            Some(file) => file.owner == *address
        }
    }

    /// <pre>
    /// Method: get_file_by_id(id: FileId) -> Option<FileStruct<<T as frame_system::Config>::AccountId>> 
    /// Arguments: id: FileId - file ID
    ///
    /// Returns the file option
    /// </pre>
    #[inline]
    pub fn get_file_by_id(id: FileId) -> Option<FileStruct<<T as frame_system::Config>::AccountId>> {
        FileByID::<T>::get(id)
    }

    fn get_random_id() -> FileId {
        let nonce = Self::get_and_increment_nonce();
        let rand = T::Randomness::random(&nonce);
        codec::Encode::using_encoded(&rand, sp_io::hashing::blake2_128)
    }

    fn get_and_increment_nonce() -> Vec<u8> {
        let nonce = NonceId::<T>::get();
        NonceId::<T>::put(nonce.wrapping_add(1));
        nonce.encode()
    }
}