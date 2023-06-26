// What would be the proper way to return a value from a possible undefined function?

use web_sys::Storage;

pub fn get_from_local_storage() -> Option<String> {
    let username = String::from("John Doe");
    Some(username)
}
