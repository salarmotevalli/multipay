#[derive(Default)]
pub struct Invoice {
    uuid: String,
    amount: f64,
    transaction_id: String,
    deriver: String,
}

impl Invoice {
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

    pub fn transaction_id(&mut self, id: String) {
        self.transaction_id = id;
    }

    pub fn get_transaction_id(&self) -> String {
        self.transaction_id.clone()
    }

    pub fn driver(&mut self, driver: String) {
        self.deriver = driver;
    }

    pub fn get_driver(&self) -> String {
        self.deriver.clone()
    }
}
