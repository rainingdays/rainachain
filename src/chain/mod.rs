use std::cmp::PartialEq;
use std::collections::HashMap;
/// Container of blockchain
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// All accepted blocks
    pub blocks: Vec<Block>,
    /// Accounts that had transactions invovled
    pub accounts: HashMap<String, Account>,
    /// transactions doesn't include into blocks aka UTXO
    pending_transactions: Vec<Transaction>,
}

/// Represents an active account
#[derive(Debug, Clone)]
pub struct Account {
    /// Account public key
    store: HashMap<String, String>,

    acc_type: AccountType,
    /// balance of the account
    tokens: u128,
}

/// Single block
#[derive(Debug, Clone)]
pub struct Block {
    pub(crate) transactions: Vec<Transaction>,
    prev_hash: Option<String>,
    trans_hash: Option<String>,
    nonce: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Transaction {
    CreateUserAccount(String),
    ChangeStoreValue { key: String, value: String },
    TransferTokens { to: String, amount: u128 },
    CreateTokens { receiver: String, amount: u128 },
}

#[derive(Clone, Debug)]
pub enum AccountType {
    User,
    Contract,
    Validator {
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
    },
}

trait CurrentWorldState {
    fn get_user_ids(&self) -> Vec<String>;
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    fn create_account(&mut self, id: String, account_type: AccountType)
        -> Result<(), &'static str>;
}
