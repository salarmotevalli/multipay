use observe::{observer::*, Subject};

pub enum PaymentEvent {
    Purchase,
}

pub type PaymentEmmiter = Subject<PaymentEvent>;
