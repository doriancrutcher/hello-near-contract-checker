use near_sdk::ext_contract;

// Validator interface, for cross-contract calls
// why does this value say hello_near and the trait says HelloNear?
#[ext_contract(hello_near)]
trait HelloNear {
    fn get_greeting(&self) -> String;
    fn set_greeting(&self, message: String);
}
