use std::collections::HashMap;

#[derive(Clone)]
pub struct Invoice {
    uuid: &'static str,
    amount: f64,
    transaction_id: &'static str,
    detail: HashMap<&'static str, &'static str>,
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

    #[inline]
    pub fn uuid(&mut self, uuid: &'static str) {
        self.uuid = uuid;
    }
    #[inline]
    pub fn get_uuid(&self) -> &'static str {
        self.uuid
    }

    #[inline]
    pub fn amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    #[inline]
    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    #[inline]
    pub fn transaction_id(&mut self, id: &'static str) {
        self.transaction_id = id;
    }

    #[inline]
    pub fn get_transaction_id(&self) -> &'static str {
        self.transaction_id
    }

    #[inline]
    pub fn detail(&mut self, key: &'static str, value: &'static str) {
        self.detail.insert(key, value);
    }

    #[inline]
    pub fn get_detail(&self, key: &'static str) -> Option<&&'static str> {
        self.detail.get(key)
    }
}
