use std::{collections::HashMap, rc::Rc};
use crate::{
    drivers::{zarinpal::ZarinPal, Driver},
    events,
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
            setting: HashMap::new(),
            driver: String::new(),
            invoice: Invoice::new(),
            callback_url: String::new(),
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
        finalize_callback: fn(Rc<dyn Driver>, String),
    ) {
        if let Some(inv) = invoice {
            self.invoice(inv)
        }

        self.driver_instance = self.get_fresh_driver_instance();

        let transaction_id = self.driver_instance.purchase();

        //        finalize_callback(self.driver_instance, transaction_id);

        self.emit(events::PaymentEvent::Purchase);
    }

    pub fn pay(&mut self, initialize_callback: Option<fn(deriver_instance: Rc<dyn Driver>)>) {
        // self.driver_instance = self.get_driver_instance();

        if let Some(ini_fn) = initialize_callback {
            ini_fn(Rc::clone(&self.driver_instance));
        }

        self.emit(events::PaymentEvent::Pay);

        self.driver_instance.pay();
    }

    pub fn verify(&self, initialize_callback: Option<fn(deriver_instance: Rc<dyn Driver>)>) {
        let receip = self.driver_instance.verify();

        if let Some(ini_fn) = initialize_callback {
            ini_fn(Rc::clone(&self.driver_instance));
        }

        self.emit(events::PaymentEvent::Verify);

        receip
    }

    pub fn get_fresh_driver_instance(&mut self) -> Rc<dyn Driver> {
        unimplemented!();
    }

    fn emit(&self, event: events::PaymentEvent) {
        let mut emitter = events::PaymentEmmiter::new();

        // attach payment observe
        emitter.add_observer(Payment::new());

        // fire event
        emitter.notify(&event);
    }
}
