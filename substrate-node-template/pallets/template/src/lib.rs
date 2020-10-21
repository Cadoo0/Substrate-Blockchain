#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, ensure};
use frame_system::ensure_signed;
use sp_std::vec::*;
use frame_support::codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Library {
    pub id: u8,
    pub name: Vec<u8>,
    pub books: Vec<Book>
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Book {
    pub id: u8,
    pub isbn: Vec<u8>,
    pub name: Vec<u8>,
    pub owner: Option<Library>
}

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Libraries get(fn library): map hasher(blake2_128_concat) u8 => Option<Library>;
		Books get(fn book): map hasher(blake2_128_concat) u8 => Option<Book>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		LibraryAdded(AccountId, u8),
		BookAdded(AccountId, u8),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_library(origin, id: u8, name: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!<Libraries>::contains_key(&id), "Deze bibliotheek zit al in het systeem");

            let library = Library {
                id,
                name,
                books: Vec::new()
            };

            <Libraries>::insert(id.clone(), library);

            Self::deposit_event(RawEvent::LibraryAdded(sender, id));

			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_book(origin, id: u8, isbn: Vec<u8>, name: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!<Books>::contains_key(&id), "Deze boek zit al in het systeem");

            let book = Book {
                id,
                isbn,
                name,
                owner: None
            };

            <Books>::insert(id.clone(), book);

            Self::deposit_event(RawEvent::BookAdded(sender, id));

			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn claim_book(origin, library_id: u8, book_id: u8) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(<Libraries>::contains_key(&library_id), "Deze bibliotheek bestaat niet");
            ensure!(<Books>::contains_key(&book_id), "Deze boek bestaat niet");

            let book = <Books>::get(book_id).unwrap();

            ensure!(book.owner == None, "Deze boek heeft al een eigenaar");

            // book.owner = Some(library.clone());
            // library.books.push(book.clone());

            <Libraries>::mutate(library_id.clone(), |library| {
                library.clone().unwrap().books.push(book.clone());
            });

            <Books>::mutate(book_id.clone(), |book| {
                let library = <Libraries>::get(library_id).unwrap();
                book.clone().unwrap().owner = Some(library.clone());
            });

            // <Libraries>::remove(library_id.clone());
            // <Books>::remove(book_id.clone());
            //
            // <Libraries>::insert(library_id, library);
            // <Books>::insert(book_id, book);

            // <Libraries>::mutate(library_id, library);
            // <Books>::mutate(book_id, book);

            // Self::deposit_event(RawEvent::BookAdded(sender, id));

			Ok(())
		}
	}
}