use crate::*;
use near_sdk::{CryptoHash};
use std::mem::size_of;

pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) {
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

pub(crate) fn internal_transfer(
    &mut self,
    sender_id: &AccountId,
    receiver_id: &AccountId,
    token_id: &TokenId
    approval_id: Option<u64>,
    memo: Option<String>
) -> Token {
    let token = self.tokens_by_id.get(token_id).expect("No token");
    
    if sender_id != &token.owner_id {
        if !token.approved_account_ids.contains_key(sender_id) {
            env::panic_str("Unauthorized");
        }

        if let Some(enforced_approval_id) = approval_id {
            let actual_approval_id = token
                .approved_account_ids
                .get(sender_id)
                .expect("Sender is not approved account");
        }
    }

}

pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
} 