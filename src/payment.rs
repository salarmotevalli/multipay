use std::collections::HashMap;

use crate::events::{PaymentEvent, PaymentObserve};
use crate::invoice::Invoice;
use crate::receipt::Receipt;
use crate::{drivers::Driver, events};
use event_observer::Subject;

#[derive(Clone, Copy)]
pub struct Payment<D: Driver + 'static> {
    callback_url: &'static str,
    driver_instance: D,
}

impl<D: Driver + 'static> Payment<D> {
    pub fn new(config: HashMap<&'static str, &'static str>, invoice: Invoice) -> Self {
        Payment {
            callback_url: "",
            driver_instance: D::new(config, invoice),
        }
    }

    pub fn purchase(&mut self, finalize_callback: Option<fn(&D, String)>) {
        let transaction_id = &self.driver_instance.purchase().expect("test");

        if let Some(fnl_fn) = finalize_callback {
            fnl_fn(&self.driver_instance, transaction_id.clone());
        }

        self.emit(events::PaymentEvent::Purchase);
    }

    pub fn pay(&mut self, initialize_callback: Option<fn(deriver_instance: &D)>) {
        if let Some(ini_fn) = initialize_callback {
            ini_fn(&self.driver_instance);
        }

        self.emit(events::PaymentEvent::Pay);

        self.driver_instance.pay();
    }

    pub fn verify(&self, initialize_callback: Option<fn(deriver_instance: &D)>) -> Receipt {
        let receipt = self.driver_instance.verify();

        if let Some(ini_fn) = initialize_callback {
            ini_fn(&self.driver_instance);
        }

        self.emit(events::PaymentEvent::Verify);

        receipt
    }

    fn emit(&self, event: PaymentEvent) {
        let mut emitter = Subject::<PaymentEvent>::new();

        emitter.add_observer(PaymentObserve);

        // fire event
        emitter.notify(&event);
    }
}
