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

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct BookTransaction {
    pub from: u8,
    pub to: u8,
    pub book: u8,
    pub transporter: u8,
}

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
        LastTransactionId get(fn last_transaction_id): u8 = 1;
		Libraries get(fn library): map hasher(blake2_128_concat) u8 => Option<Library>;
		Books get(fn book): map hasher(blake2_128_concat) u8 => Option<Book>;
		BookTransactions get(fn transaction): map hasher(blake2_128_concat) u8 => Option<BookTransaction>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		LibraryAdded(AccountId, u8),
		BookAdded(AccountId, u8),
		TransactionAdded(AccountId, u8),
	}
);

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
		pub fn add_book_transaction(origin, from: u8, to: u8, book_id: u8, transporter_id: u8) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            let transaction = BookTransaction {
                from,
                to,
                book: book_id,
                transporter: transporter_id
            };

            let last_id = <LastTransactionId>::get();

            <BookTransactions>::insert(last_id, transaction);

            Self::deposit_event(RawEvent::TransactionAdded(sender, last_id));

            <LastTransactionId>::put(last_id + 1);

			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn claim_book(origin, library_id: u8, book_id: u8) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;

            ensure!(<Books>::contains_key(&book_id), "Deze boek bestaat niet");
            ensure!(<Libraries>::contains_key(&library_id), "Deze bibliotheek bestaat niet");
            ensure!(<Books>::get(book_id).unwrap().owner == None, "Deze boek heeft al een eigenaar");

            // <Books>::mutate(book_id.clone(), |editable_book| {
            //     let library = <Libraries>::get(library_id).unwrap();
            //
            //     editable_book.unwrap().owner = Some(library.clone());
            // });

            // <Libraries>::mutate(library_id.clone(), |editable_library| {
            //     let book = <Books>::get(book_id).unwrap();
            //
            //     editable_library.unwrap().books.push(book.clone());
            // });

            // Self::deposit_event(RawEvent::BookAdded(sender, id));

			Ok(())
		}
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,
	}
}
