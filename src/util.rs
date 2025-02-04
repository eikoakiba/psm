use crate::password;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn banner(version: &f64) {
    println!(
        "MyPass password manager version {} \n\
        A simple and powerfull password manager \n\
        \nUsage: mypass [OPTIONS...] \n\
            \
        \t-a Add a new password \n\
        \t-n Set name for your new password \n\
        \t-d Set description for your new password \n\
        \t-A Add a new password with TUI menu \n\
        \t-l List ALl of your password by their name \n\
        \t-L List all of your password by their name and description and date \n\
        \t-v get the version of this program \n\
        \t-m modify password information",
        version,
    )
}

// PSM Passwords Folder Struction:

//  .pass folder to store all the passwords that each password has
//  its own folder with the data and meta files.
//  For example:

//  ===================================
//  .pass/
//  |- instagram/
//  |  |- data - data file that stores things like password and etc.
//  |  |- meta - metadata file that stores the ascii base meta data.
//  |- TikTok/
//  |  |- data - ~
//  |  |- meta - ~
//  |- Spacehey/
//  |  |- data - ~
//  |  |- meta - ~
// ====================================

// The data file is the main file that stores the encrypted password
// And if there is any metadata file, the needed data will be store
// on the same file. The data file will store password and metadata
// by order.

// The metadata file is the information file that use to store the
// extra information about the password, like description and date.
// Notice that the name of the password will be the same as the name
// of the parent folder. with that said we don't need to store the name
// of the password in the meta file anymore.

// TODO: Replace Result<(), String> with bool
// Add password to the origin
pub fn origin_add(password: &password::Password) -> bool {
    if !is_data_exists(&password.name) {
        if let Err(_) = fs::create_dir(format!("./pass/{}", &password.name)) {
            println!("[!] Can't find the password folder");
            return false;
        }
    } else {
        // TODO: Ask the user to agree the existence of password
        println!("[!] notice you are updating the exists password");
    }

    let data: String = format!("{}", password.value.clone());
    let meta: String = format!("{}\n{}", password.description, password.date);

    let file_data = fs::File::create(format!("./pass/{}/data", &password.name));
    if let Err(_) = file_data {
        println!("Can't make data file_data");
        return false;
    }

    let res = file_data.unwrap().write_all(data.as_bytes());
    if let Err(_) = res {
        println!("Can't write to the file_data");
        return false;
    }

    let file_meta = fs::File::create(format!("./pass/{}/meta", &password.name));
    if let Err(_) = file_meta {
        println!("Can't make meta file");
        return false;
    }

    let res = file_meta.unwrap().write_all(meta.as_bytes());
    if let Err(_) = res {
        println!("Can't write to the file meta");
        return false;
    }

    true
}

pub fn origin_show(name: &String) -> Result<password::Password, String> {
    let mut u_pass = password::Password::default();

    // Check for existing of password
    if !is_data_exists(name) {
        // TODO: Ask the user if wants to make a new password with the same name
        return Err(format!("Can't find any password with that name."));
    }

    // Read the data and meta files and Check for data/meta readabliti
    let data = fs::read(format!("./pass/{}/data", name));
    if let Err(err) = data {
        return Err(err.to_string());
    }

    // Read the meta file
    let meta = fs::read(format!("./pass/{}/meta", name));
    if let Err(err) = meta {
        return Err(err.to_string());
    }

    // Shadowing the data and meta
    let data = String::from_utf8(data.unwrap());
    let meta = String::from_utf8(meta.unwrap());

    // TOOD: Better warning handeling
    if let Err(err) = data {
        return Err(err.to_string());
    } else if let Err(err1) = meta {
        return Err(err1.to_string());
    }

    let data = data.unwrap();
    let meta = meta.unwrap();

    // Parse them into u_pass

    let l_data = data.split("\n").collect::<Vec<&str>>();
    let d_pass = l_data[0]; // First line name

    let l_meta: Vec<&str> = meta.split("\n").collect();
    let description = l_meta[0];
    let date = l_meta[1];

    u_pass.name = String::from(name);
    u_pass.description = String::from(description);
    u_pass.value = String::from(d_pass);
    u_pass.date = String::from(date);

    // return the pass obj
    Ok(u_pass)
}

pub fn is_data_exists(name: &str) -> bool {
    if Path::new(format!("./pass/{}", name).as_str()).exists() {
        return true;
    }
    return false;
}

// Check is there is any global .pass folder
pub fn is_origin_exists() -> bool {
    if Path::new("./pass").exists() {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample: String = String::from("This is the text\nThis is the new line");
        let l_data = sample.split("\n").collect::<Vec<&str>>();
        println!("This is the first line {}", l_data[0]);
    }
}
