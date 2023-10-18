use event_observer::{Subject, Observer};
use crate::payment::Payment;

pub enum PaymentEvent {
    Purchase,
    Pay,
    Verify,
}

pub type PaymentEmmiter = Subject<PaymentEvent>;

impl Observer<PaymentEvent> for Payment {
    fn on_notify(&self, event: &PaymentEvent) {
        use PaymentEvent::*;
        
        match event {
            Purchase => println!("Hello Purchase"),
            Pay => println!("hello Pay"),
            Verify => println!("hello Verify"),
        }
    }
}
