use crate::*;

pub trait NonFungibleTokenCore {
  	fn nft_payout(
        &self, 
        token_id: TokenId, 
        balance: U128, 
        max_len_payout: u32
    );
    
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    );
}