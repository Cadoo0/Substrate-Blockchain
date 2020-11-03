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
        pub from: u8,
        pub to: u8,
        pub book_id: u8,
        pub end_timestamp: u32,
        pub bids: Vec<(AccountId, u8)>,
    }

    impl Auction {
        #[ink(constructor)]
        pub fn new(from: u8, to: u8, book_id: u8, end_timestamp: u32) -> Self {
            Self { from, to, book_id, end_timestamp, bids: Vec::new() }
        }

        #[ink(message)]
        pub fn get_from(&self) -> u8 {
            self.from
        }

        #[ink(message)]
        pub fn get_to(&self) -> u8 {
            self.to
        }

        #[ink(message)]
        pub fn get_book_id(&self) -> u8 {
            self.book_id
        }

        #[ink(message)]
        pub fn get_end_timestamp(&self) -> u32 {
            self.end_timestamp
        }

        // #[ink(message)]
        // pub fn get_bid_history(&self) -> Vec<(AccountId, u8)> {
        //     self.bids
        // }

        #[ink(message)]
        pub fn get_nth_bid(&self, index: u32) -> Option<(AccountId, u8)> {
            if self.bids.len() <= (index + 1) {
                return Some(self.bids[index])
            }

            None
        }

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
