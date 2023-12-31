use std::collections::HashMap;

use super::ZarinPalStrategy;
use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice, DriverConfig};

static ZARINGATE_API_PURCHASE_URL: &str = "https://ir.zarinpal.com/pg/services/WebGate/wsdl";
static ZARINGATE_API_PAYMENT_URL: &str =
    "https://www.zarinpal.com/pg/StartPay/:authority/ZarinGate";
static ZARINGATE_API_VERIFICATION_URL: &str = "https://ir.zarinpal.com/pg/services/WebGate/wsdl";

pub(super) struct Zaringate {
    invoice: Invoice,
        config: HashMap<&'static str, &'static str>

}

impl Driver for Zaringate {
    #[inline]
    fn new(drv_cnf: HashMap<&'static str, &'static str>, invoice: Invoice) -> Self {
        Zaringate { invoice, config: drv_cnf }
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

    #[inline]
        fn set_config(&mut self, config: HashMap<&'static str, &'static str>) {
                self.config = config;
        }

}

impl ZarinPalStrategy for Zaringate {}
