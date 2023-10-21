use super::Driver;
use crate::{invoice::Invoice, receipt::Receipt};

enum ZarinpalStrategy {
    Normal,
    SandBox,
    Zaringate,
}

pub struct ZarinPal {
    strategy: ZarinpalStrategy,
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
    pub fn new(strategy: &'static str, invoice: Invoice) -> Self {
        let stg = match strategy {
            "normal" => ZarinpalStrategy::Normal,
            "sandbox" => ZarinpalStrategy::SandBox,
            "zariingate" => ZarinpalStrategy::Zaringate,
            &_ => ZarinpalStrategy::Normal
        };

        ZarinPal {
            strategy: stg,
            invoice,
        }
    }
}
