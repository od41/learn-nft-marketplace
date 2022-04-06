use crate::*;
pub type TokenId = String;

//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
} 

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate="near_sdk::serde")]
pub struct NFTContractMetadata {
    // do stuff
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate="near_sdk::serde")]
pub struct TokenMetadata {
    // do stuff
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate="near_sdk::serde")]
pub struct Token {
    // do stuff
}

#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct JsonToken {
    // do stuff
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self);
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) {
        // do stuff
    }
}