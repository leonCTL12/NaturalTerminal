use keyring::Entry;

const APP_NAME: &str = "natural_terminal";
const OPEN_ROUTER_API_KEY: &str = "OPEN_ROUTER_API_KEY";
pub fn set_open_router_api_key (api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entry = match Entry::new(APP_NAME, OPEN_ROUTER_API_KEY) {
        Ok(entry) => entry,
        Err(_) => {
            return Err("Error creating entry".into());
        }
    };
    match entry.set_password(api_key) {
        Ok(_) => {
            println!("password set successfully");
            Ok(())
        },
        Err(_) => Err("Error setting password".into())
    }
}

pub fn get_open_router_api_key() -> Result<String, Box<dyn std::error::Error>> {
    let entry = match Entry::new(APP_NAME, OPEN_ROUTER_API_KEY) {
        Ok(entry) => entry,
        Err(_) => {
            return Err("Error creating entry".into());
        }
    };
    match entry.get_password() {
        Ok(password) => Ok(password),
        Err(_) => Err("Error getting password".into()),
    }
}