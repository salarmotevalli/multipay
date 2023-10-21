use crate::payment::Payment;
use event_observer::{Observer, Subject};

pub enum PaymentEvent {
    Purchase,
    Pay,
    Verify,
}

pub type PaymentEmmiter = Subject<PaymentEvent>;

impl<'a> Observer<PaymentEvent> for Payment<'a> {
    fn on_notify(&self, event: &PaymentEvent) {
        use PaymentEvent::*;

        match event {
            Purchase => println!("Hello Purchase"),
            Pay => println!("hello Pay"),
            Verify => println!("hello Verify"),
        }
    }
}
