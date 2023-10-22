use crate::invoice::Invoice;
use crate::receipt::Receipt;
use crate::{
    drivers::{zarinpal::ZarinPal, Driver},
    events,
};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct Payment {
    setting: HashMap<String, String>,
    callback_url: &'static str,
    driver_instance: Rc<dyn Driver>,
    invoice: Invoice,
}

impl Payment {
    pub fn new(invoice: Invoice) -> Self {
        Payment {
            setting: HashMap::new(),
            callback_url: "",
            driver_instance: Rc::new(ZarinPal::new("sandbox", invoice.clone())),
            invoice,
        }
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

    pub fn transaction_id(&mut self, id: &'static str) {
        self.invoice.transaction_id(id);
    }

    pub fn via(&mut self) {
        // self.invoice.via(driver);
        unimplemented!()
    }

    pub fn purchase(&mut self, finalize_callback: Option<fn(Rc<dyn Driver>, String)>) {
        self.driver_instance = self.get_fresh_driver_instance();

        let transaction_id = self.driver_instance.purchase();

        if let Some(fnl_fn) = finalize_callback {
            fnl_fn(Rc::clone(&self.driver_instance), transaction_id);
        }

        self.emit(events::PaymentEvent::Purchase);
    }

    pub fn pay(&mut self, initialize_callback: Option<fn(deriver_instance: Rc<dyn Driver>)>) {
        if let Some(ini_fn) = initialize_callback {
            ini_fn(Rc::clone(&self.driver_instance));
        }

        self.emit(events::PaymentEvent::Pay);

        self.driver_instance.pay();
    }

    pub fn verify(
        &self,
        initialize_callback: Option<fn(deriver_instance: Rc<dyn Driver>)>,
    ) -> Receipt {
        let receipt = self.driver_instance.verify();

        if let Some(ini_fn) = initialize_callback {
            ini_fn(Rc::clone(&self.driver_instance));
        }

        self.emit(events::PaymentEvent::Verify);

        receipt
    }

    pub fn get_fresh_driver_instance(&mut self) -> Rc<dyn Driver> {
        unimplemented!();
    }

    fn emit(&self, event: events::PaymentEvent) {
        let mut emitter = events::PaymentEmmiter::new();

        // attach payment observe
        emitter.add_observer(self.clone());

        // fire event
        emitter.notify(&event);
    }
}
