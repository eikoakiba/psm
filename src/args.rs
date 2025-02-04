#[derive(Default, Debug)]
pub struct CfgField {
    pub name: String,
    pub value: String,
    pub rank: usize,
}

impl CfgField {
    pub fn IsExists(&self) -> bool {
        return self.name != "";
    }
}

#[derive(Default, Debug)]
pub struct Config {
    init_origin: CfgField,
    new_password: CfgField,
    key: CfgField,
    show_password: CfgField,
    generate_password: CfgField,
    pub new_name: CfgField,
    new_description: CfgField,
    new_password_tui: CfgField,
    list_passwords: CfgField,
    resset_origin: CfgField,
    list_password_verbose: CfgField,
    program_version: CfgField,
    modify_password: CfgField,
}

// const ARGS: &'static [&'static str] = &["-a"];

impl Config {
    fn new() -> Self {
        Config::default()
    }
    pub fn IsNewPassword(&self) -> Option<&CfgField> {
        match self.new_password.IsExists() {
            true => Some(&self.new_password),
            false => None,
        }
    }

    pub fn IsInitOrigin(&self) -> Option<&CfgField> {
        match self.init_origin.IsExists() {
            true => Some(&self.init_origin),
            false => None,
        }
    }

    pub fn IsNewName(&self) -> Option<&CfgField> {
        match self.new_name.IsExists() {
            true => Some(&self.new_name),
            false => None,
        }
    }

    pub fn IsListPassword(&self) -> Option<&CfgField> {
        match self.list_passwords.IsExists() {
            true => Some(&self.list_passwords),
            false => None,
        }
    }

    pub fn IsRessetOrigin(&self) -> Option<&CfgField> {
        match self.resset_origin.IsExists() {
            true => Some(&self.resset_origin),
            false => None,
        }
    }

    pub fn IsShowPassword(&self) -> Option<&CfgField> {
        match self.show_password.IsExists() {
            true => Some(&self.show_password),
            false => None,
        }
    }

    pub fn IsGeneratePassword(&self) -> Option<&CfgField> {
        match self.generate_password.IsExists() {
            true => Some(&self.generate_password),
            false => None,
        }
    }

    pub fn IsKeyExists(&self) -> Option<&CfgField> {
        match self.key.IsExists() {
            true => Some(&self.key),
            false => None,
        }
    }

    pub fn IsNewDescription(&self) -> &CfgField {
        return &self.new_description;
    }

    pub fn IsNewPasswordTui(&self) -> &CfgField {
        return &self.new_password_tui;
    }
}

pub fn parse_arguments(args: &Vec<String>) -> Option<Config> {
    let mut cfg: Config = Config::new();
    let mut two: bool = false;
    let mut args_iter_obj = args[1..].iter().enumerate();
    if args.len() <= 1 {
        // println!("There is no any item");
        return None;
    }
    loop {
        if args_iter_obj.len() <= 0 {
            // println!("Ran out of index");
            break;
        }
        if (args_iter_obj.len() >= 2) {
            two = true;
        } else {
            two = false;
        }
        // let mut i: usize;
        let mut v: &String = &String::default();
        let (index, mut value) = args_iter_obj.next().expect("The Point 1");

        if two {
            loop {
                // println!("RIGHT");
                (_, v) = args_iter_obj.next().expect("The point 2");
                // TODO: Make chars of v's unwarap better
                if v.chars().nth(0).unwrap() == '-' {
                    value = v;
                    continue;
                } else {
                    break;
                }
            }
        }
        //println!("{} and {}", value, v);
        match value.as_str() {
            "-i" | "--init" => {
                println!("You provide the -a option");
                cfg.init_origin = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-a" | "--append" => {
                println!("You provide the -a option");
                cfg.new_password = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-n" | "--name" => {
                println!("You provide the -n option");
                cfg.new_name = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-d" | "--description" => {
                println!("You provide the -d option");
                cfg.new_description = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-l" | "--list" => {
                println!("You provide the -l option");
                cfg.list_passwords = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-k" | "--key" => {
                println!("You provide the -k option");
                cfg.key = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }

            "-g" | "--generate" => {
                println!("You provide the -g option");
                cfg.generate_password = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-r" | "--resset" => {
                println!("You provide the -r option");
                cfg.resset_origin = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            &_ => {
                println!("{}", value);
            }
        }
    }
    Some(cfg)
}
