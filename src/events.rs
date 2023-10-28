use event_observer::Observer;

pub enum PaymentEvent {
    Purchase,
    Pay,
    Verify,
}

pub struct PaymentObserve;

impl Observer<PaymentEvent> for PaymentObserve {
    fn on_notify(&self, event: &PaymentEvent) {
        use PaymentEvent::*;

        match event {
            Purchase => println!("Hello Purchase"),
            Pay => println!("hello Pay"),
            Verify => println!("hello Verify"),
        }
    }
}
