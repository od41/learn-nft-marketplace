use crate::*;
use near_sdk::{ext_contract, Gas};

const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenCore {
    fn nft_approve(
        &mut self, 
        token_id: TokenId, 
        account_id: AccountId, 
        msg: Option<String>
    );

    fn nft_is_approved(
        &mut self, 
        token_id: TokenId, 
        approved_account_id: AccountId, 
        approval_id: Option<u64>,
    );

    fn nft_revoke(
        &mut self, 
        token_id: TokenId, 
        account_id: AccountId, 
    );

    fn nft_revoke_all(
        &mut self, 
        token_id: TokenId,
    );
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}