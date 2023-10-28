use super::ZarinPalStrategy;
use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice};

static API_PURCHASE_URL: &str = "https://api.zarinpal.com/pg/v4/payment/request.json";
static API_PAYMENT_URL: &str = "https://www.zarinpal.com/pg/StartPay/";
static API_VERIFICATION_URL: &str = "https://api.zarinpal.com/pg/v4/payment/verify.json";

pub(super) struct Normal {
    invoice: Invoice,
}

impl Driver for Normal {
    fn new(invoice: Invoice) -> Self {
        Normal { invoice }
    }

    fn purchase(&self) -> Result<String, MultiPayErr> {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}

impl ZarinPalStrategy for Normal {}
