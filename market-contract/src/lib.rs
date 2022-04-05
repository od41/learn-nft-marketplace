#[init]
pub fn new (owner_id: AccountId) -> Self {
    let this = Self {
        owner_id,
        sales: UnorderedMap::new(StorageKey::Sales),
        by_owner_id: LookupMap::new(Storage::ByOwnerId),
        by_nft_contract_id: LookupMap::new(StorageKey::ByNFTContractId),
        storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
    };

    this
}

#[payable]
pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
    let storage_account_id = account_id
        .map(|a| a.into())
        .unwrap_or_else(env::predecessor_account_id);

    let deposit = env::attached_deposit();

    assert!(
        deposit >= STORAGE_PER_SALE,
        "Requires minimum deposit of {}",
        STORAGE_PER_SALE
    );

    let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
    balance += deposit;
    self.storage_deposits.insert(&storage_account_id, &balance);
}

#[payable]
pub fn storage_withdraw(&mut self) {
    assert_one_yocto();

    let owner_id = env::predecessor_account_id();
    let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);

    let sales = self.by_owner_id.get(&owner_id);
    let len = sales.map(|s| s.len()).unwrap_or_default();

    let diff = u128::from(len) * STORAGE_PER_SALE;

    amount -= diff;

    if amount > 0 {
        Promise::new(owner_id.clone()).transfer(amount);
    }

    if diff > 0 {
        self.storage_deposits.insert(&owner_id, &diff);
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    pub sale_conditions: SalePriceInYoctoNear,
}