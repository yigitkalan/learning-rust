use std::{collections::HashMap, io::stdin};

fn main() {
    let remove_appendix: &str = "REMOVE";
    let add_appendix: &str = "TO";
    let mut departmants: HashMap<String, Vec<String>> = HashMap::new();
    println!("Welcome to employee management System \n");

    loop {
        println!("Enter Your Command:");
        let mut command = String::new();

        stdin()
            .read_line(&mut command)
            .expect("Failed to process command");
        let words: Vec<&str> = command.split_whitespace().collect();

        let verb = match words.get(0) {
            Some(word) => word.to_uppercase(),
            None => {
                println!("Invalid Command, to see possible options type 'Help'");
                continue;
            }
        };

        let verb = str_to_cmd(&verb);
        let mut name = String::new();
        let mut appendix = String::new();
        let mut department = String::new();

        if words.len() > 1 {
            name = match words.get(1) {
                Some(val) => val.to_uppercase(),
                None => {
                    println!("Invalid Command, to see possible options type 'Help'\n");
                    continue;
                }
            };

            if words.len() > 2 {
                appendix = match words.get(2) {
                    Some(val) => val.to_uppercase(),
                    None => {
                        println!("Invalid Command, to see possible options type 'Help'\n");
                        continue;
                    }
                };
                if words.len() == 4 {
                    department = match words.get(3) {
                        Some(val) => val.to_uppercase(),
                        None => {
                            println!("Invalid Command, to see possible options type 'Help'\n");
                            continue;
                        }
                    };
                } else {
                    println!("Invalid Command, to see possible options type 'Help'\n");
                    continue;
                }
            }
        }

        match &verb {
            Command::Add => {
                if appendix == add_appendix {
                    add_to_department(&mut departmants, name, department)
                } else {
                    println!("Invalid Command, to see possible options type 'Help'\n");
                    continue;
                }
            }
            Command::Remove => {
                if appendix == remove_appendix {
                    remove_from_department(&mut departmants, name, department)
                } else {
                    println!("Invalid Command, to see possible options type 'Help'\n");
                    continue;
                }
            }
            Command::Get => get_departmant(&departmants, name),
            Command::Help => show_help(),
            Command::Exit => break,
            Command::Err => {
                println!("Invalid Command, to see possible options type 'Help'\n");
                continue;
            }
        }
    }
}

fn show_help() {
    println!("Use the available formats:");
    println!(" ---- 'Add employe__name to departmant_name' to add an employee to a department");
    println!(
        " ---- 'Remove employee_name from departmant_name' to remove an employee from department"
    );
    println!(" ---- 'Get departmant_name' to list a specific department ");
    println!(" ---- 'Exit' to quit the program \n\n");
}

fn str_to_cmd(word: &str) -> Command {
    match word {
        "ADD" => Command::Add,
        "REMOVE" => Command::Remove,
        "GET" => Command::Get,
        "HELP" => Command::Help,
        "EXIT" => Command::Exit,
        _ => Command::Err,
    }
}

fn get_departmant(company: &HashMap<String, Vec<String>>, name: String) {
    let dep = match company.get(&name) {
        Some(data) => data,
        None => {
            println!("No such department\n");
            return;
        }
    };

    println!("{:?}", dep);
}

fn add_to_department(company: &mut HashMap<String, Vec<String>>, emp: String, dep: String) {
    let employees = company.entry(dep).or_insert(Vec::new());
    if !employees.contains(&emp) {
        employees.push(emp);
    }
}

fn remove_from_department(company: &mut HashMap<String, Vec<String>>, emp: String, dep: String) {
    let employees = company.entry(dep).or_insert(Vec::new());
    if !employees.contains(&emp) {
        employees.retain(|name| name != &emp)
    }
}

enum Command {
    Add,
    Remove,
    Get,
    Help,
    Exit,
    Err,
}
