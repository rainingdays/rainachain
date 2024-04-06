use std::cmp::PartialEq;
use std::collections::HashMap;
use std::time::SystemTime;
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

#[derive(Debug, Clone)]
pub struct Transaction {
    nonce: u128,
    from: String,
    created_at: SystemTime,
    pub(crate) record: TransactionData,
    signature: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionData {
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

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        return Self {
            store: HashMap::new(),
            acc_type: account_type,
            tokens: 0,
        };
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), String> {
        let is_genesis = self.blocks.len() == 0;

        if !block.verify_own_hash() {
            return Err("The block hash is mismatching! (Code: 1234)".into());
        }
        if !(block.prev_hash == self.get_last_block_hash()) {
            return Err("The new block has to point to the previous block (Code: 1235)".into());
        }
        if block.get_transaction_count() == 0 {
            return Err(
                "There has to be at least one transaction inside the block! (Code: 12346)".into(),
            );
        }
        // Since we don't want to implement a complex roll-back algorithm, just clone the old state in case we need roll back
        let old_state = self.accounts.clone();

        for (i, transaction) in block.transactions.iter().enumerate() {
            if let Err(err) = transaction.execute(self, &is_genesis) {
                // Recover state on failure
                self.accounts = old_state;

                return Err(format!(
                    "Could not execute transaction {} due to `{}`. Rolling back (Code: 1237)",
                    i + 1,
                    err
                ));
            }
        }
        self.blocks.push(block);

        Ok(())
    }
}

impl Block {
    pub fn verify_own_hash(&self) -> bool {
        if self.trans_hash.is_some()
            && self
                .trans_hash
                .as_ref()
                .unwrap()
                .eq(&byte_vector_to_string(&self.calculate_hash()))
        {
            return true;
        }
        false
    }
}
