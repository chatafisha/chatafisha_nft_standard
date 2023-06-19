
 use near_contract_standards::non_fungible_token::metadata::{
     NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
 };
 use near_contract_standards::non_fungible_token::{Token, TokenId};
 use near_contract_standards::non_fungible_token::NonFungibleToken;
 use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
 use near_sdk::collections::{LazyOption};
 use near_sdk::{
     env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
 };
 use std::convert::TryInto;
 
 #[near_bindgen]
 #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
 pub struct Contract {
     tokens: NonFungibleToken,
     metadata: LazyOption<NFTContractMetadata>,
 }
 
 const DATA_IMAGE_SVG_NEAR_ICON: &str = "https://media.licdn.com/dms/image/D4D07AQEzxLyfF8lY4w/group-logo_image-shrink_92x92/0/1680622225500?e=1682355600&v=beta&t=h22hblAO3BzLkFsaca7mdAqJT8mW58GhA2hT5GHeT3A";
 
 #[derive(BorshSerialize, BorshStorageKey)]
 enum StorageKey {
     NonFungibleToken,
     Metadata,
     TokenMetadata,
     Enumeration,
     Approval,
 }
 
 #[near_bindgen]
 impl Contract {
     /// Initializes the contract owned by `owner_id` with
     /// default metadata (for example purposes only).
     #[init]
     pub fn new_default_meta() -> Self {
         Self::new(
             NFTContractMetadata {
                 spec: NFT_METADATA_SPEC.to_string(),
                 name: "Chatafisha marketplace".to_string(),
                 symbol: "CHAT".to_string(),
                 icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                 base_uri: None,
                 reference: None,
                 reference_hash: None,
             },
         )
     }
 
     #[init]
     pub fn new(metadata: NFTContractMetadata) -> Self {
         assert!(!env::state_exists(), "Already initialized");
         metadata.assert_valid();
         Self {
             tokens: NonFungibleToken::new(
                 StorageKey::NonFungibleToken,
                 "chatafisha_nft.testnet".to_string().try_into().unwrap(),
                 Some(StorageKey::TokenMetadata),
                 Some(StorageKey::Enumeration),
                 Some(StorageKey::Approval),
             ),
             metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
         }
     }
 
     
 
     /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
     ///
     /// Since this example implements metadata, it also requires per-token metadata to be provided
     /// in this call. `self.tokens.mint` will also require it to be Some, since
     /// `StorageKey::TokenMetadata` was provided at initialization.
     ///
     /// `self.tokens.internal_mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
     /// initialization call to `new`.
     #[payable]
     pub fn nft_mint(
         &mut self,
         token_id: TokenId,
         receiver_id: AccountId,
         token_metadata: TokenMetadata,
     ) -> Token {
         self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
     }
 }
 
 near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
 near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
 near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);
 
 #[near_bindgen]
 impl NonFungibleTokenMetadataProvider for Contract {
     fn nft_metadata(&self) -> NFTContractMetadata {
         self.metadata.get().unwrap()
     }
 }