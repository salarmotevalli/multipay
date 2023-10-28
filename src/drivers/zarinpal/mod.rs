mod normal;
mod sandbox;
mod zaringate;

use self::normal::Normal;

use super::{Driver, DriverConfig};
use crate::{error::MultiPayErr, invoice::Invoice, receipt::Receipt};

pub struct ZarinPalConfig {
    /* normal api */
    api_purchase_url: &'static str,
    api_payment_url: &'static str,
    api_verification_url: &'static str,

    /* sandbox api */
    sandbox_api_purchase_url: &'static str,
    sandbox_api_payment_url: &'static str,
    sandbox_api_verification_url: &'static str,

    /* zarinGate api */
    zaringate_api_purchase_url: &'static str,
    zaringate_api_payment_url: &'static str,
    zaringate_api_verification_url: &'static str,

    mode: &'static str, // can be normal, sandbox, zaringate
    merchant_id: &'static str,
    callback_url: &'static str,
    description: &'static str,
    currency: &'static str, //Can be R, T (Rial, Toman)
}

impl DriverConfig for ZarinPalConfig {
    fn load() -> Self {
        ZarinPalConfig {
            api_purchase_url: "https://api.zarinpal.com/pg/v4/payment/request.json",
            api_payment_url: "https://www.zarinpal.com/pg/StartPay/",
            api_verification_url: "https://api.zarinpal.com/pg/v4/payment/verify.json",

            sandbox_api_purchase_url: "https://sandbox.zarinpal.com/pg/services/WebGate/wsdl",
            sandbox_api_payment_url: "https://sandbox.zarinpal.com/pg/StartPay",
            sandbox_api_verification_url: "https://sandbox.zarinpal.com/pg/services/WebGate/wsdl",

            zaringate_api_purchase_url: "https://ir.zarinpal.com/pg/services/WebGate/wsdl",
            zaringate_api_payment_url: "https://www.zarinpal.com/pg/StartPay/:authority/ZarinGate",
            zaringate_api_verification_url: "https://ir.zarinpal.com/pg/services/WebGate/wsdl",

            mode: "normal",
            merchant_id: "",
            callback_url: "",
            description: "",
            currency: "R",
        }
    }
}

impl ZarinPalConfig {
    #[inline]
    pub fn mode(&mut self, mode: &'static str) {
        self.mode = mode;
    }

    #[inline]
    pub fn merchent_id(&mut self, m_id: &'static str) {
        self.merchant_id = m_id;
    }

    #[inline]
    pub fn callback_url(&mut self, callback_url: &'static str) {
        self.callback_url = callback_url;
    }

    #[inline]
    pub fn description(&mut self, description: &'static str) {
        self.description = description;
    }

    #[inline]
    pub fn currency(&mut self, currency: &'static str) {
        self.currency = currency;
    }
}

pub struct ZarinPal {
    strategy: Box<dyn Driver>,
}

impl Driver for ZarinPal {
    #[inline]
    fn new(invoice: Invoice) -> Self {
        ZarinPal {
            strategy: Box::new(Normal::new(invoice)),
        }
    }

    #[inline]
    fn amount(&mut self, amount: f64) {
        self.strategy.amount(amount);
    }

    #[inline]
    fn transaction_id(&mut self, id: &'static str) {
        self.strategy.transaction_id(id);
    }

    #[inline]
    fn detail(&mut self, key: String, value: String) {
        self.strategy.detail(key, value);
    }

    #[inline]
    fn pay(&self) {
        self.strategy.as_ref().pay();
    }

    #[inline]
    fn verify(&self) -> Receipt {
        self.strategy.as_ref().verify()
    }

    #[inline]
    fn purchase(&self) -> Result<String, MultiPayErr> {
        self.strategy.as_ref().purchase()
    }
}

pub trait ZarinPalStrategy {}
