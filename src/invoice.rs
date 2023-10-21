use std::collections::HashMap;

#[derive(Clone)]
pub struct Invoice {
    uuid: &'static str,
    amount: f64,
    transaction_id: &'static str,
    detail: HashMap<String, String>,
    // deriver: String,
}

impl Invoice {
    pub fn new(amount: f64) -> Self {
        Invoice {
            amount,
            uuid: "",
            transaction_id: "",
            detail: HashMap::new(),
        }
    }

    pub fn uuid(&mut self, uuid: &'static str) {
        self.uuid = uuid;
    }

    pub fn get_uuid(&self) -> &'static str {
        self.uuid.clone()
    }

    pub fn amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn transaction_id(&mut self, id: &'static str) {
        self.transaction_id = id;
    }

    pub fn get_transaction_id(&self) -> &'static str {
        self.transaction_id
    }

    pub fn detail(&mut self, key: String, value: String) {
        self.detail.insert(key, value);
    }

    pub fn get_detail(&self, key: String) -> Option<&String> {
        self.detail.get(&key)
    }
}
