use std::collections::HashMap;

#[derive(Default)]
pub struct Invoice<'a> {
    uuid: String,
    amount: f64,
    transaction_id: &'a str,
    detail: HashMap<String, String>,
    deriver: String,
}

impl<'a> Invoice<'a> {
    pub fn new() -> Self {
        Invoice {
            ..Default::default()
        }
    }

    pub fn uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn transaction_id(&mut self, id: &'a str) {
        self.transaction_id = id;
    }

    pub fn get_transaction_id(&self) -> &'a str {
        self.transaction_id
    }

    pub fn detail(&mut self, key: String, value: String) {
        self.detail.insert(key, value);
    }

    pub fn get_detail(&self, key: String) -> Option<&String> {
        self.detail.get(&key)
    }

    pub fn get_driver(&self) -> String {
        self.deriver.clone()
    }

    pub fn via(&mut self, deriver: String) {
        self.deriver = deriver;
    }
}
