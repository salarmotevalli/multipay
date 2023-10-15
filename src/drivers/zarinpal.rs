use super::Driver;

pub struct ZarinPal {}

impl Driver for ZarinPal {
    fn pay(&self) {
        unimplemented!()
    }

    fn verify(&self) {
        unimplemented!()
    }

    fn purchase(&self) -> String {
        unimplemented!()
    }
}

impl ZarinPal {
    pub fn new() -> Self {
        ZarinPal {}
    }
}
