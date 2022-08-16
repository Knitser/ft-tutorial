use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};

pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const DATA_IMAGE_SVG_GT_ICON: &str = "data:image/jpeg;base64,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";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Keep track of each account's balances
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// The bytes for the largest possible account ID that can be registered on the contract 
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in 
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Team Token FT Tutorial".to_string(),
                symbol: "gtNEAR".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized. 
        let mut this = Self {
            // Set the total supply and bytes for the longest account ID to 0 initially. These will be populated after.
            total_supply: 0,
            bytes_for_longest_account_id: 0,
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
        .emit();

        // Return the Contract object
        this
    }
}