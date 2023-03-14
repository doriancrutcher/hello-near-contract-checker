use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env::{predecessor_account_id, random_seed, signer_account_id};
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

pub mod external;
pub use crate::external::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const NO_ARGS: Vec<u8> = vec![];

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    records: LookupMap<AccountId, bool>,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"The contract is not initialized.")
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        // Useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here
        Self {
            records: LookupMap::new(b"r".to_vec()),
        }
    }

    // Public - query external greeting
    pub fn evaluate_hello_near(&mut self, contract_name: AccountId) -> Promise {
        // // First let's get a random string from random seed
        let get_array: Vec<u8> = random_seed();
        let random_string: String = String::from_utf8_lossy(&get_array).to_string();
        println!("the random string is {:?}", random_string);

        let args = json!({ "message": random_string }).to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call("set_greeting".to_string(), args, NO_DEPOSIT, Gas(5 * TGAS))
            .function_call(
                "get_greeting".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_hello_near_callback(random_string, contract_name.clone()),
            )
    }

    #[private]
    pub fn evaluate_hello_near_callback(
        &mut self,
        #[callback_result] last_result: Result<String, PromiseError>,
        random_string: String,
        contract_name: AccountId,
    ) -> bool {
        // The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {result}"));
            let output = result == random_string;
            self.records.insert(&contract_name, &output);
            output
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    pub fn account_participation(&self, account_name: AccountId) -> bool {
        self.records.get(&account_name).unwrap_or(false)
    }
}
