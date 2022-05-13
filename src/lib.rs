use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::{near_bindgen, AccountId, env, Balance, log};
use near_sdk::collections::{LazyOption};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::storage_management::{StorageManagement, StorageBalance, StorageBalanceBounds};
use near_contract_standards::fungible_token::core::{FungibleTokenCore};
use near_contract_standards::fungible_token::metadata::{ FungibleTokenMetadata, FT_METADATA_SPEC, FungibleTokenMetadataProvider};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    token : FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}



#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "D fungible token".to_string(),
                symbol: "D".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 24,
            }
        )
    }
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Contract already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        // near_contract_standards::fungible_token
        // near_contract_standards::fungible_token::events::FtMint{
        //     owner_id: &owner_id,
        //     amount: &total_supply,
        //     memo: Some("Initial tokens supply is minted"),
        // }.emit();
        this
    }
    fn on_account_closed(&mut self, account_id: AccountId, balance:Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }
    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}


#[near_bindgen]
impl FungibleTokenCore for Contract {
    fn ft_transfer(&mut self, receiver_id: near_sdk::json_types::ValidAccountId, amount: U128, memo: Option<String>) {
        self.token.ft_transfer(receiver_id, amount, memo)
    }

    #[payable]
    fn ft_transfer_call(&mut self, receiver_id: near_sdk::json_types::ValidAccountId, amount: U128, memo: Option<String>, msg: String) -> near_sdk::PromiseOrValue<U128> {
        self.token.ft_transfer_call(receiver_id, amount, memo, msg)
    }

    fn ft_total_supply(&self) -> U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: near_sdk::json_types::ValidAccountId) -> U128 {
        self.token.ft_balance_of(account_id)
    }
}


#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<ValidAccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.token.storage_deposit(account_id, registration_only)
    }

    #[payable]
    fn storage_withdraw(
        &mut self,
        amount: Option<U128>
    ) -> StorageBalance {
        self.token.storage_withdraw(amount)
    }

    #[payable]
    fn storage_unregister(
        &mut self,
        force: Option<bool>
    ) -> bool {
        #[allow(unused_variables)]
        if let Some((account_id, balance)) = self.token.internal_storage_unregister(force){
            self.on_account_closed(account_id, balance);
            true
        } else {
            false
        }
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.token.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: ValidAccountId) -> Option<StorageBalance> {
        self.token.storage_balance_of(account_id)
    }

}


#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};
    use super::*;
    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();

        builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);

        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)), TOTAL_SUPPLY);
    }


}