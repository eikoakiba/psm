use std::fs;

use crate::args::{CfgField, Config};
use crate::password::{self};
use crate::util::{self, is_origin_exists, origin_add};
use aes_gcm::aead::Nonce;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use base64::prelude::*;

const NONCE_SIZE_SAMPLE: usize = 12;

fn add_new_password(
    new_pass_field: &CfgField,
    new_desc_field: &CfgField,
    new_name_field: &CfgField,
    key_field: Vec<u8>,
    is_new_key: bool,
) -> Result<(), String> {
    // println!("{} - {}", config.new_name.name, config.new_name.value);
    let date: String = format!("2024"); // TODO!()
                                        // Check if there is required things active
    let actual_key = key_field.as_slice();
    // Use unwrap since we check them before
    let base = new_password(
        &new_name_field.value,
        &new_pass_field.value,
        &date,
        &new_desc_field.value,
        actual_key,
    );

    if !origin_add(&base) {
        return Err(String::from("Fail to make new password"));
    }

    println!("Successfully Encrypted your password: {:?}", base.value);

    if is_new_key {
        println!(
            "[!] This is your new key: {:?} Keep this for your own",
            actual_key
        );
    }

    Ok(())
}

pub fn generate_random_key() -> Vec<u8> {
    // let mut rng: &[u8; 32] = &[32; 32];
    let asd = Aes256Gcm::generate_key(OsRng);
    let bts = asd.as_slice();
    // println!("{:?}", BASE64_STANDARD.encode(asd));
    // let n_key: String = String::from_utf8(asd).unwrap();
    Vec::from(bts)
}

pub fn new_password(
    name: &String,
    password: &String,
    date: &String,
    description: &String,
    key: &[u8],
) -> password::Password {
    // TODO: Find a better way that cloning the value
    let mut base: password::Password =
        password::Password::new(name.clone(), description.clone(), date.clone());
    let gna = Key::<Aes256Gcm>::from_slice(key);
    let g_key = Aes256Gcm::new(gna);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = g_key.encrypt(&nonce, password.as_bytes().as_ref());
    // TODO: Check the ciphertext Result enum
    base.set_value(ciphertext.unwrap(), nonce);
    base.is_enc = true;

    println!(
        "This is the generated password bytes: {:?}",
        password.as_bytes(),
    );
    // println!("This is the Text {:?}", ciphertext);
    // println!("{:?}", ciphertext);
    base
}

// Show the password to the user
pub fn show_password(name: &String, key: &String) -> Result<(), String> {
    // Get the password by its name
    let upr: Result<password::Password, String> = util::origin_show(&name.to_string());
    let key = BASE64_STANDARD.decode(key);
    if let Err(err) = key {
        return Err(format!(
            "[!] Can't parse the key cause {}. Please provide a valid key",
            err,
        ));
    }
    let key = key.unwrap();
    println!("{}", key.len());

    // Show/print the actual metadata and data
    if let Err(err) = upr {
        return Err(format!("File: {}", err));
    }
    let mut upr: password::Password = upr.unwrap();

    // TODO: Check the unwrap
    let f_pass = BASE64_STANDARD.decode(&upr.value).unwrap();
    let f_pass_res = f_pass.as_slice();

    println!("This is the read key from args: {:?}", key.as_slice());
    println!("This is the read password base64 from file: {}", &upr.value);
    println!("This is the read password from file: {:?}", f_pass_res);

    let nonce_res = &f_pass_res[..NONCE_SIZE_SAMPLE];
    let f_pass_res = &f_pass_res[NONCE_SIZE_SAMPLE..];

    // Descript the data base on key
    let gna = Key::<Aes256Gcm>::from_slice(key.as_slice());
    let g_key = Aes256Gcm::new(gna);
    let nonce = Nonce::<Aes256Gcm>::clone_from_slice(nonce_res);
    // let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = g_key.decrypt(&nonce, f_pass_res.as_ref());
    // Show the result data nad metadata togethera

    if let Err(err) = ciphertext {
        return Err(format!("Encryption: {}", err));
    }
    let result_pass = ciphertext.unwrap();
    upr.is_enc = false;

    println!(
        "name: {}\ndescription: {}\ndate: {}\npassword: {}",
        upr.name,
        upr.description,
        upr.date,
        String::from_utf8(result_pass).unwrap()
    );

    Ok(())
}

pub fn print_list_passwords(field: &CfgField) -> Result<(), String> {
    todo!("Print the list of user passwords's metadata");
}

pub fn process_args(config: &Config) -> Result<String, String> {
    if let Some(_) = config.IsInitOrigin() {
        if let Err(err) = fs::create_dir("./pass") {
            return Err(format!("Can't make the origin folder"));
        }
        // TODO: Make a single function to generate key
        let n_key = generate_random_key();
        let m_key = BASE64_STANDARD.encode(n_key);
        println!();
        return Ok(format!(
            "Sucsessfully generated the origin :D\nThis is your new key: {}\n",
            String::from(m_key),
        ));
    }
    // TODO: Make a better process if statement
    else if let Some(_) = config.IsNewPassword() {
        // Check for existsing of origin

        let new_pass = config.IsNewPassword();
        let new_desc = config.IsNewDescription(); // Make a better output (return)
        let l_new_name: Option<&CfgField> = config.IsNewName();
        let l_generate_pass: Option<&CfgField> = config.IsGeneratePassword();
        let l_key: Option<&CfgField> = config.IsKeyExists();

        if let None = l_new_name {
            return Err(String::from("Please Provide a name for your password"));
        }

        let mut n_key: Vec<u8> = Vec::default();
        let mut is_new_key: bool = false;
        let actual_key: &[u8] = &[0; 30];
        let m_key: Vec<u8>;

        if let None = l_key {
            if let None = l_generate_pass {
                return Err(String::from(
                    "Please Provide your main key for encryption the password",
                ));
            } else {
                if (util::is_origin_exists()) {
                    return Err(String::from("You already made the base key.\nPlease use the same key or make another with `psm -g`"));
                }
                n_key = generate_random_key();
                is_new_key = true;
                m_key = BASE64_STANDARD.decode(&n_key).unwrap();
            }
        } else {
            m_key = BASE64_STANDARD.decode(&l_key.unwrap().value).unwrap();
            // println!(
            //     "This is the ready to use key for make new password {:?}",
            //     actual_key
            // );
        }

        if let Err(err) = add_new_password(
            new_pass.unwrap(),
            new_desc,
            l_new_name.unwrap(),
            m_key,
            is_new_key,
        ) {
            // Ok(_) => {
            //     println!("+ Your new password added to the system");
            // }
            return Err(err);
        }
        return Ok(format!("Succsessfully update your password list"));
    } else if let Some(p_name) = config.IsNewName() {
        // Check for existsing of origin
        if !is_origin_exists() {
            return Err(String::from(
                "Can't find the origin path. Please make one with `psm --init or psm -i`",
            ));
        }
        let key = config.IsKeyExists();
        if let None = key {
            return Err(String::from("Please provide the key"));
        }
        let key = key.unwrap();
        if let Err(err) = show_password(&p_name.value, &key.value) {
            return Err(format!("Print: {}", err));
        }
        return Ok(String::default());
    } else if let Some(list_pass) = config.IsListPassword() {
        // Check for existsing of origin
        if !is_origin_exists() {
            return Err(String::from(
                "Can't find the origin path. Please make one with `psm --init or psm -i`",
            ));
        }
        if let Err(err) = print_list_passwords(list_pass) {
            return Err(format!("Can't show the list of passwords cause: {}", err));
        }
        return Ok(String::default());
    } else if let Some(_) = config.IsRessetOrigin() {
        // Check for existsing of origin
        if !is_origin_exists() {
            return Err(String::from(
                "Can't find the origin path. Please make one with `psm --init or psm -i`",
            ));
        }
        // TODO: remove origin recursive
        return Ok(String::from("[+] Succsessfully resset the origin"));
    } else {
        return Err(String::from("Please provide a option"));
    }
}
