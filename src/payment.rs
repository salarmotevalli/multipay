use std::{collections::HashMap, rc::Rc};

use observe::Observer;

use crate::{
    drivers::{zarinpal::ZarinPal, Driver},
    events::{PaymentEmmiter, PaymentEvent},
};

use super::invoice::Invoice;

pub struct Payment {
    setting: HashMap<String, String>,
    callback_url: String,
    driver: String,
    driver_instance: Rc<dyn Driver>,
    invoice: Invoice,
}

impl Default for Payment {
    fn default() -> Self {
        Payment {
            driver_instance: Rc::new(ZarinPal::new()),
            ..Default::default()
        }
    }
}

impl Observer<PaymentEvent> for Payment {
    fn on_notify(&self, event: &PaymentEvent) {
        match event {
            PaymentEvent::Purchase => println!("Hello"),
        }
    }
}

impl Payment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_setting(&mut self, key: String, value: String) {
        self.setting.insert(key, value);
    }

    pub fn callback_url(&mut self, url: String) {
        self.set_setting("callbackUrl".to_string(), url)
    }

    pub fn reset_callback_url(&mut self) {
        self.set_setting("callbackUrl".to_string(), "".to_string())
    }

    pub fn amount(&mut self, amount: f64) {
        self.invoice.amount(amount);
    }

    pub fn detail(&mut self, key: String, value: String) {
        self.invoice.detail(key, value);
    }

    pub fn transaction_id(&mut self, id: String) {
        self.invoice.transaction_id(id);
    }

    pub fn invoice(&mut self, invoice: Invoice) {
        self.invoice = invoice;
    }

    pub fn via(&mut self, driver: String) {
        self.driver = driver.clone();
        self.invoice.via(driver);
        unimplemented!()
    }

    pub fn purchase(
        &mut self,
        invoice: Option<Invoice>,
        finalize_callback: fn(Box<dyn Driver>, String),
    ) {
        if let Some(inv) = invoice {
            self.invoice(inv)
        }

        self.driver_instance = self.get_fresh_driver_instance();

        let transaction_id = self.driver_instance.purchase();

        //        finalize_callback(self.driver_instance, transaction_id);

        self.emit(PaymentEvent::Purchase);
    }

    pub fn pay(&mut self, initialize_callback: Option<fn(deriver_instance: Rc<dyn Driver>)>) {
        // self.driver_instance = self.get_driver_instance();
        
        if let Some(ini_fn) = initialize_callback {
            ini_fn(Rc::clone(&self.driver_instance));
        } 

        self.emit(PaymentEvent::Pay);

        self.driver_instance.pay();
    }

    pub fn get_fresh_driver_instance(&mut self) -> Rc<dyn Driver> {
        unimplemented!()
    }

    fn emit(&self, event: PaymentEvent) {
        let mut emitter = PaymentEmmiter::new();
        emitter.add_observer(Payment::new());
    }
}
