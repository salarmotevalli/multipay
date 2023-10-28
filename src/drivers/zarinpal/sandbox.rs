use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice};

use super::ZarinPalStrategy;

static SANDBOX_API_PURCHASE_URL: &str = "https://sandbox.zarinpal.com/pg/services/WebGate/wsdl";
static SANDBOX_API_PAYMENT_URL: &str = "https://sandbox.zarinpal.com/pg/StartPay";
static SANDBOX_API_VERIFICATION_URL: &str = "https://sandbox.zarinpal.com/pg/services/WebGate/wsdl";

pub(super) struct Sandbox {
    invoice: Invoice,
}

impl Driver for Sandbox {
    fn new(invoice: Invoice) -> Self {
        Sandbox { invoice }
    }

    fn purchase(&self) -> Result<String, MultiPayErr> {
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}

impl ZarinPalStrategy for Sandbox {}
