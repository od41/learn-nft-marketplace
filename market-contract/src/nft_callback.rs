use crate::*;

fn nft_on_approve(
    &mut self,
    token_id: TokenId,
    owner_id: AccountId,
    approval_id: u64,
    msg: String,
) {
    let nft_contract_id = env::predecessor_account_id();
    let signer_id = env::signer_account_id();

    assert_ne!(
        nft_contract_id,
        signer_id,
        "nft_on_approve should only be called via cross-contract call"
    );

    assert_eq!(
        owner_id,
        signer_id,
        "owner_id should be signer_id"
    );

    let storage_amount = self.storage_minimum_balance().0;
    let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
    let signer_storage_required = (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * sotrage_amount;

    assert!(
        owner_paid)storage >= signer_storage_required,
        "Insufficient storage paid: {}, for {} sales at {} rate of per sale",
        owner_paid_storage, signer_storage_required / STORAGE_PER_SALE, STORAGE_PER_SALE
    );

    let SaleArgs { sale_conditions} = 
        near_sdk::serde_json::from_str(&msg).expect("Not valid SaleArgs");
    
    let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
}