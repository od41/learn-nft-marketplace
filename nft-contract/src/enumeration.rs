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
    ) -> Vec<JsonToken> {
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        tokens.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}