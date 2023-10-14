use super::invoice::Invoice;

#[derive(Default)]
pub struct Payment {
    config: Vec<String>,
    setting: Vec<String>,
    callback_url: String,
    driver: String,
    driver_instance: String,
    invoice: Invoice,
}

impl Payment {
    pub fn new() -> Self {
        //    Self::via(String::new());
        Payment {
            ..Default::default()
        }
    }

    pub fn set_config(&mut self, config: Vec<String>) {
        self.config = config;
    }

    pub fn get_default_config() -> String {
        String::new()
    }

    pub fn config() -> String {
        unimplemented!()
    }

    pub fn callback_url(url: String) {}

    pub fn reset_callback_url() {}

    pub fn amount(&mut self, amount: f64) {
        self.invoice.amount(amount);
    }

    pub fn detail(&mut self, key: String, value: String) {
        self.invoice.detail(key, value);
    }

    pub fn transaction_id(&mut self, id: String) {
        self.invoice.transaction_id(id);
    }

    pub fn via(&mut self, driver: String) {
        self.driver = driver;
        self.invoice.via(driver);
        unimplemented!()
    }

    pub fn purchase() {
        unimplemented!()
    }

    pub fn pay() {
        unimplemented!()
    }
}
