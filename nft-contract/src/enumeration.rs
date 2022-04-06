use crate::*;

#[near_bindgen]
impl Contract {
    pub fn nft_total_supply(&self) {
        // do stuff
    }

    pub fn nft_tokens(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>
    ) {
        // do stuff
    }

    pub fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) {
        // do stuff
    }

    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) {
        // do stuff
    }
}