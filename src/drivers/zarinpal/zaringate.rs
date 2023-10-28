use super::ZarinPalStrategy;
use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice};

static ZARINGATE_API_PURCHASE_URL: &str = "https://ir.zarinpal.com/pg/services/WebGate/wsdl";
static ZARINGATE_API_PAYMENT_URL: &str =
    "https://www.zarinpal.com/pg/StartPay/:authority/ZarinGate";
static ZARINGATE_API_VERIFICATION_URL: &str = "https://ir.zarinpal.com/pg/services/WebGate/wsdl";

pub(super) struct Zaringate {
    invoice: Invoice,
}

impl Driver for Zaringate {
    #[inline]
    fn new(invoice: Invoice) -> Self {
        Zaringate { invoice }
    }

    #[inline]
    fn purchase(&self) -> Result<String, MultiPayErr> {
        unimplemented!()
    }

    #[inline]
    fn pay(&self) {}

    #[inline]
    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}

impl ZarinPalStrategy for Zaringate {}
