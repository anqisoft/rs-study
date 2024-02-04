fn exercise_03() {
    println!("\nIn exercise_03()");

    use std::{collections::HashMap, io};

    let prompt = "\
Welcome to this People List gadget!
You can enter in the following three different formats to use the corresponding function:
1. Add employee list: Add employee_name to department_name
2. Query the employee list of a certain department: Query department_name
3. Query the list of employees in all departments (arranged in dictionary order): List
You can also type quit to exit this gadget, thank you for using it!"
        .to_string();

    #[derive(Debug)]
    enum Commands {
        Add(String, String),
        Query(String),
        List,
        Quit,
        Unknown,
    }

    fn parse_command(message: &str) -> Commands {
        let mut command = String::new();
        let mut employee_name = String::new();
        let mut department_name = String::new();

        for (index, item) in message.trim().split_whitespace().into_iter().enumerate() {
            match index {
                0 => {
                    command.push_str(item);
                }
                1 => {
                    if command == "add" {
                        employee_name.push_str(item);
                    } else if command == "query" {
                        department_name.push_str(item);
                    }
                }
                2 => {
                    if command == "add" {
                        if item != "to" {
                            println!(
                                "Please input by this format: Add employee_name to department_name"
                            );
                            return Commands::Unknown;
                        }
                    }
                }
                3 => {
                    if command == "add" {
                        department_name.push_str(item);
                    }
                }
                _ => {
                    return Commands::Unknown;
                }
            }
        }

        match &command as &str {
            "add" => Commands::Add(employee_name, department_name),
            "query" => Commands::Query(department_name),
            "list" => Commands::List,
            "quit" => Commands::Quit,
            _ => Commands::Unknown,
        }
    }

    let mut departments: Vec<String> = Vec::with_capacity(20);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    fn add(
        map: &mut HashMap<String, Vec<String>>,
        departments: &mut Vec<String>,
        employee_name: &String,
        department_name: &String,
    ) {
        println!("exec: add {employee_name} to {department_name}");

        if employee_name.len() == 0 {
            print!("Error: When you want to add an employee to a department, the employee name cannot be empty.");
        } else if department_name.len() == 0 {
            print!("Error: When you want to add employees to a department, the department name cannot be empty.");
        } else {
            let people_names = map.entry(department_name.to_string()).or_insert(vec![]);
            people_names.push(employee_name.to_string());

            if !departments.contains(&department_name) {
                departments.push(department_name.to_string());

                // order the items of departments
                departments.sort();
            }
        }
    }

    fn query(map: &HashMap<String, Vec<String>>, department_name: &String) {
        println!("exec: query {department_name}");
        if department_name.len() == 0 {
            print!("Error: When querying the employee list of a certain department, the department name cannot be empty.");
        } else {
            print!("The employee list of {}:\t", &department_name);
            let mut people_names: Vec<String> =
                map.get(department_name).unwrap_or(&Vec::new()).to_vec();
            people_names.sort();
            for name in people_names {
                print!("{}\t", &name);
            }
            println!();
        }
    }

    fn list(map: &HashMap<String, Vec<String>>, departments: &Vec<String>) {
        println!("exec: list all");
        for department_name in departments {
            print!("{}:\t", &department_name);
            let mut people_names: Vec<String> = map
                .get(&(department_name.to_string()))
                .unwrap_or(&Vec::new())
                .to_vec();
            people_names.sort();
            for name in people_names {
                print!("{}\t", &name);
            }
            println!();
        }
    }

    fn quit() {
        println!("Now, quit it.");
    }

    loop {
        println!("{prompt}");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match parse_command(&input) {
                    Commands::Add(employee_name, department_name) => {
                        add(&mut map, &mut departments, &employee_name, &department_name);
                    }
                    Commands::Query(department_name) => {
                        query(&map, &department_name);
                    }
                    Commands::List => {
                        list(&map, &departments);
                    }
                    Commands::Quit => {
                        quit();
                        break;
                    }
                    Commands::Unknown => {
                        println!("Error: unknown command name.");
                        continue;
                    }
                }

                println!();
            }
            Err(error) => {
                println!("The error is '{error}.");
                continue;
            }
        }
    }
}