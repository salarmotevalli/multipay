use std::collections::HashMap;

use super::ZarinPalStrategy;
use crate::{drivers::Driver, error::MultiPayErr, invoice::Invoice, DriverConfig};

static API_PURCHASE_URL: &str = "https://api.zarinpal.com/pg/v4/payment/request.json";
static API_PAYMENT_URL: &str = "https://www.zarinpal.com/pg/StartPay/";
static API_VERIFICATION_URL: &str = "https://api.zarinpal.com/pg/v4/payment/verify.json";

pub(super) struct Normal {
    invoice: Invoice,
    config: HashMap<&'static str, &'static str>
}

impl Driver for Normal {
    fn new(drv_cnf: HashMap<&str, &str>, invoice: Invoice) -> Self {
        Normal { invoice , config: drv_cnf}
    }

    fn purchase(&self) -> Result<String, MultiPayErr> {
        let data: HashMap<&str, &str> = HashMap::new();
        unimplemented!()
    }

    fn pay(&self) {}

    fn verify(&self) -> crate::receipt::Receipt {
        unimplemented!()
    }
}

impl ZarinPalStrategy for Normal {}
