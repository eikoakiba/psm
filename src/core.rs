use std::result;

use crate::args::Config;

fn add_new_password(config: &Config) -> Result<(), String> {
    // Check if there is required things active
    if let Some(value) = config.IsNewName() {
        // OPtional: Description is optional for that
    }
    Ok(())
}

pub fn process_args(config: &Config) -> Result<(), ()> {
    // Check if there is new request
    if let Some(value) = config.IsNewPassword() {
        match add_new_password(config) {
            Ok(value) => {
                println!("+ Your new password added to the system");
            }
            Err(err) => {
                println!("- Can't add your password cause {}", err);
            }
        }
    }
    //if (config.IsNewPassword() != "") {
    //    if (config.IsNewName().value != "" && config.IsNewPassword) {
    //        println!("+ Start processing new password with name {}", config.)
    //    }
    //}
    Ok(())
}
