use super::invoice::Invoice;

pub struct Payment {
   config: Vec<String>,
   setting: Vec<String>,
   callback_url: String,
   driver: String,
   driver_instance: String,
   invoice: Invoice,
}

impl Payment {
    
}
