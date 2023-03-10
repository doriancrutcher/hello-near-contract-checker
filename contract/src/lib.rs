use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::random_seed;
use near_sdk::{
    env, log, near_bindgen, serde_json::json, AccountId, Gas, PanicOnDefault, Promise, PromiseError,
};

pub mod external;
pub use crate::external::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;
const XCC_GAS: Gas = Gas(30u64.pow(13));

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

impl Default for Contract {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl Contract {
    // Public - query external greeting
    pub fn batch_actions(&self, contract_name: AccountId) -> Promise {
        // // First let's get a random string from random seed
        let get_array: Vec<u8> = random_seed();
        let random_string: String = String::from_utf8_lossy(&get_array).to_string();
        println!("the random string is {:?}", random_string);

        let promise1 = hello_near::ext(contract_name.clone())
            .with_static_gas(Gas(5 * TGAS))
            .set_greeting(random_string.clone());

        let promise2 = hello_near::ext(contract_name)
            .with_static_gas(Gas(5 * TGAS))
            .get_greeting();

        promise1
            .then(promise2)
            .then(Self::ext(env::current_account_id()).batch_actions_callback(random_string))
    }

    #[private]
    pub fn batch_actions_callback(
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
