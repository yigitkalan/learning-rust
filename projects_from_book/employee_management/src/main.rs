use std::{collections::HashMap, io::{stdin, self, Write}};

fn main() {
    let remove_appendix: &str = "FROM";
    let add_appendix: &str = "TO";
    let mut departmants: HashMap<String, Vec<String>> = HashMap::new();
    println!("Welcome to employee management System \n");

    loop {
        print!("-> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut command = String::new();

        stdin()
            .read_line(&mut command)
            .expect("Failed to process command");
        let words: Vec<&str> = command.split_whitespace().collect();

        let verb = match words.get(0) {
            Some(word) => word.to_uppercase(),
            None => {
                error_msg();
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
                    error_msg();
                    continue;
                }
            };

            if words.len() > 2 {
                appendix = match words.get(2) {
                    Some(val) => val.to_uppercase(),
                    None => {
                        error_msg();
                        continue;
                    }
                };
                if words.len() == 4 {
                    department = match words.get(3) {
                        Some(val) => val.to_uppercase(),
                        None => {
                            error_msg();
                            continue;
                        }
                    };
                } else {
                    error_msg();
                    continue;
                }
            }
        }

        println!();
        match &verb {
            Command::Add => {
                if appendix == add_appendix {
                    add_to_department(&mut departmants, name, department)
                } else {
                    error_msg();
                    continue;
                }
            }
            Command::Remove => {
                if appendix == remove_appendix {
                    remove_from_department(&mut departmants, name, department)
                } else {
                    error_msg();
                    continue;
                }
            }
            Command::Get => get_departmant(&departmants, name),
            Command::Help => show_help(),
            Command::Exit => break,
            Command::Err => {
                error_msg();
                continue;
            }
        }
    }
}

fn error_msg() {
    println!("Invalid Command, to see possible options type 'Help'\n");
}

fn show_help() {
    println!("Use the available formats:\n");
    println!(" 'Add employe__name to departmant_name' to add an employee to a department\n");
    println!(
        " 'Remove employee_name from departmant_name' to remove an employee from department"
    );
    println!();
    println!(" 'Get departmant_name' to list a specific department \n");
    println!(" 'Exit' to quit the program \n\n");
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

    println!("People in {}:", make_lower_case(&name));
    dep.iter()
        .for_each(|person| println!("{}", make_lower_case(person)));
}

fn add_to_department(company: &mut HashMap<String, Vec<String>>, emp: String, dep: String) {
    let employees = company.entry(dep).or_insert(Vec::new());
    if !employees.contains(&emp) {
        employees.push(emp);
    }
}

fn remove_from_department(company: &mut HashMap<String, Vec<String>>, emp: String, dep: String) {
    let employees = company.entry(dep).or_insert(Vec::new());
    if employees.contains(&emp) {
        employees.retain(|name| name != &emp)
    }
}

fn make_lower_case(stri: &str) -> String {
    //make the first character of each word upper case and the rest lowercase
    let mut new_string = String::new();
    for (i, c) in stri.chars().enumerate() {
        if i == 0 {
            new_string.push(c.to_uppercase().next().unwrap());
        } else {
            new_string.push(c.to_lowercase().next().unwrap());
        }
    }
    new_string
}

#[derive(Debug)]
enum Command {
    Add,
    Remove,
    Get,
    Help,
    Exit,
    Err,
}
