use observe::{observer::*, Subject};

pub enum PaymentEvent {
    Purchase,
    Pay,
}

pub type PaymentEmmiter = Subject<PaymentEvent>;
