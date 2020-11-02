#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod auction {
    use ink_storage::collections::Vec;

    #[ink(event)]
    pub struct NewHighestBid {
        #[ink(topic)]
        amount: u8,
        #[ink(topic)]
        transporter_id: AccountId,
    }

    #[ink(storage)]
    pub struct Auction {
        pub book_id: u8,
        pub bids: Vec<(AccountId, u8)>,
    }

    impl Auction {
        #[ink(constructor)]
        pub fn new(book_id: u8) -> Self {
            Self { book_id, bids: Vec::new() }
        }

        #[ink(message)]
        pub fn get_book_id(&self) -> u8 {
            self.book_id
        }

        // #[ink(message)]
        // pub fn get_bid_history(&self) -> Vec<(AccountId, u8)> {
        //     self.bids
        // }

        #[ink(message)]
        pub fn get_latest_bid(&self) -> Option<(AccountId, u8)> {
            if self.bids.len() > 0 {
                return Some(self.bids[self.bids.len() - 1])
            }

            None
        }

        #[ink(message)]
        pub fn new_bid(&mut self, bid: u8) {
            if self.bids.len() == 0 {
                self.bids.push((Self::env().caller(), bid));

                Self::env().emit_event(NewHighestBid {
                    amount: bid.clone(),
                    transporter_id: Self::env().caller()
                });

                return;
            }

            let latest_bid = self.bids[self.bids.len() - 1];

            if latest_bid.1 > bid {
                self.bids.push((Self::env().caller(), bid));

                Self::env().emit_event(NewHighestBid {
                    amount: bid.clone(),
                    transporter_id: Self::env().caller()
                });
            }
        }
    }
}
