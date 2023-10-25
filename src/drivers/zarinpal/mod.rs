mod normal;
mod sandbox;
mod zaringate;

use self::{normal::Normal, sandbox::Sandbox, zaringate::Zaringate};

use super::Driver;
use crate::{invoice::Invoice, receipt::Receipt, error::MultiPayErr};

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

impl ZarinPalConfig {
    pub fn load() -> Self {
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
    fn pay(&self) {
        self.strategy.pay();
    }

    fn verify(&self) -> Receipt {
        self.strategy.verify()
    }

    fn purchase(&self) -> Result<String, MultiPayErr> {
        self.strategy.purchase()
    }
}

impl ZarinPal {
    pub fn new(config: ZarinPalConfig, invoice: Invoice) -> Self {
        let stg: Box<dyn Driver> = match config.mode {
            "normal" => Box::new(Normal::new(config, invoice)),
            "sandbox" => Box::new(Sandbox::new(config, invoice)),
            "zariingate" => Box::new(Zaringate::new(config, invoice)),
            &_ => Box::new(Normal::new(config, invoice)),
        };

        ZarinPal { strategy: stg }
    }
}
