use std::collections::HashMap;


pub trait HasDetail {

    fn detail(key: String, value: String) {
        
    }

    fn get_detail() {
        
    }

    fn get_details(&self) -> HashMap<String, String> {
       unimplemented!() 
    }
}
