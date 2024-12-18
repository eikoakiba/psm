#[derive(Default, Debug)]
pub struct CfgField {
    pub name: String,
    pub value: String,
    pub rank: usize,
}

impl CfgField {
    pub fn IsExists(&self) -> bool {
        return self.name != "" && self.value != "";
    }
}

#[derive(Default, Debug)]
pub struct Config {
    new_password: CfgField,
    new_name: CfgField,
    new_description: CfgField,
    new_password_tui: CfgField,
    list_passwords: CfgField,
    list_password_verbose: CfgField,
    program_version: CfgField,
    modify_password: CfgField,
}

const ARGS: &'static [&'static str] = &["-a"];

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

    pub fn IsNewName(&self) -> Option<&CfgField> {
        match self.new_name.IsExists() {
            true => Some(&self.new_name),
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
    let mut args_iter_obj = args[2..].iter().enumerate();
    if args.len() <= 1 {
        println!("There is no any item");
        return None;
    }
    loop {
        let (index, value) = args_iter_obj.next()?;
        if index > args.len() {
            println!("Ran out of index");
            break;
        }
        let (i, v) = args_iter_obj.next()?;
        //println!("{} and {}", value, v);
        match value.as_str() {
            "-a" => {
                println!("You provide the -a option");
                cfg.new_password = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-n" => {
                println!("You provide the -n option");
                cfg.new_name = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-d" => {
                println!("You provide the -d option");
                cfg.new_description = CfgField {
                    name: String::from(value),
                    value: String::from(v),
                    rank: index,
                }
            }
            "-l" => {
                println!("You provide the -d option");
                cfg.list_passwords = CfgField {
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
