use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::random_seed;
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

pub mod external;
pub use crate::external::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const NO_ARGS: Vec<u8> = vec![];

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    // Public - query external greeting
    pub fn evaluate_hello_near(&self, contract_name: AccountId) -> Promise {
        // // First let's get a random string from random seed
        let get_array: Vec<u8> = random_seed();
        let random_string: String = String::from_utf8_lossy(&get_array).to_string();
        println!("the random string is {:?}", random_string);

        let args = json!({ "message": random_string })
            .to_string()
            .into_bytes();

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
                    .evaluate_hello_near_callback(random_string),
            )
    }

    #[private]
    pub fn evaluate_hello_near_callback(
        &self,
        #[callback_result] last_result: Result<String, PromiseError>,
        random_string: String,
    ) -> bool {
        // The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {result}"));
            result == random_string
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }
}
