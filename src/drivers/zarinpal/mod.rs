mod sandbox;
mod normal;
mod zaringate;

use super::Driver;
use crate::{invoice::Invoice, receipt::Receipt};

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

    pub fn mode(&mut self, mode: &'static str) {
        self.mode = mode;
    }

    pub fn merchent_id(&mut self, m_id: &'static str) {
        self.merchant_id = m_id;
    }

    pub fn callback_url(&mut self, callback_url: &'static str) {
        self.callback_url = callback_url;
    }

    pub fn description(&mut self, description: &'static str) {
        self.description = description;
    }

    pub fn currency(&mut self, currency: &'static str) {
        self.currency = currency;
    }
}

pub struct ZarinPal {
    config: ZarinPalConfig,
    invoice: Invoice,
}

impl Driver for ZarinPal {
    fn pay(&self) {
        unimplemented!()
    }

    fn verify(&self) -> Receipt {
        Receipt::new("test_id")
    }

    fn purchase(&self) -> String {
        unimplemented!()
    }
}

impl ZarinPal {
    pub fn new(cnfig: ZarinPalConfig, invoice: Invoice) -> Self {
        let stg = match cnfig.mode {
            "normal" => ZarinpalStrategy::Normal,
            "sandbox" => ZarinpalStrategy::SandBox,
            "zariingate" => ZarinpalStrategy::Zaringate,
            &_ => ZarinpalStrategy::Normal,
        };

        ZarinPal {
            strategy: cnfig,
            invoice,
        }
    }
}
