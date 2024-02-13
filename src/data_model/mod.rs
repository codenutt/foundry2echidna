use serde::{Deserialize, Serialize};

/*//////////////////////////////////////////////////////////////
                        DATA MODEL STRUCTS
////////////////////////////////////////////////////////////// */
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Broadcast {
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(rename(serialize = "event"))]
    pub transaction_type: String,
    pub contract_address: String,
    pub transaction: TransactionDetails,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub gas_used: String,
    #[serde(rename(serialize = "gas_price"))]
    pub effective_gas_price: String,
}

fn default_tx_value() -> String {
    "0x0".to_string()
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct TransactionDetails {
    pub from: String,
    pub to: Option<String>,
    #[serde(default = "default_tx_value")]
    pub value: String,
    pub data: String,
}
