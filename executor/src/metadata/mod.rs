use move_core_types::account_address::AccountAddress;

#[derive(Debug)]
pub struct TestMetadata {
    script_name: String,
    signers: Vec<AccountAddress>,
    failure: Option<u32>,

}

