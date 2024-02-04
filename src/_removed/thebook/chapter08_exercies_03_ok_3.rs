
/*
 * <en>
 * Using hash maps and vectors, create a text interface that allows users to add employee names to
 * company departments. For example, "Add Sally to Engineering" or "Add Amir to Sales."
 * Then let the user get a list of all employees in a department, or a list of all employees in
 * each department of the company in lexicographic order.
 * </en>
 *
 * <cn>
 * 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
 * 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
 * 或者公司每个部门的所有员工按照字典序排列的列表。
 * </cn>
 *
 * <tw>
 * 使用哈希 map 和 vector，建立一個文字介面來允許使用者在公司的部門中增加員工的名字。
 * 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。 接著讓使用者取得一個部門的所有員工的列表，
 * 或是公司每個部門的所有員工依照字典序排列的列表。
 * </tw>
*/
fn exercise_03() {
    println!("\nIn exercise_03()");

    use std::{collections::HashMap, io};

    /*
     * <en>
     * Welcome to this People List gadget!
     * You can enter in the following three different formats to use the corresponding function:
     * 1. Add employee list: Add employee_name to department_name
     * 2. Query the employee list of a certain department: : Query department_name
     * 3. Query the list of employees in all departments (arranged in dictionary order): List
     * You can also type 'quit' to exit this gadget, thank you for using it!
     * </en>
     *
     * <cn>
     * 欢迎使用这个人员名单小工具！
     * 您可以输入按照以下三种不同的格式使用相应功能：
     * 1. 增加员工名单：Add 员工名 to 部门名
     * 2. 查询某部门的员工名单：Query 部门名
     * 3. 查询所有部门的员工名单（按照字典顺序排列）：List
     * 您也可以输入quit退出本小工具，谢谢您的使用！
     * 现在，请您输入所需内容：
     * </cn>
     *
     * <tw>
     * 歡迎使用這個人員名單小工具！
     * 您可以輸入依照以下三種不同的格式使用對應功能：
     * 1. 增加員工名單：Add 員工名稱 to 部門名
     * 2. 查詢某部門的員工名單：Query 部門名
     * 3. 查詢所有部門的員工名單（依照字典順序排列）：List
     * 您也可以輸入quit退出本小工具，謝謝您的使用！
     * 現在，請您輸入所需內容：
     * </tw>
     */
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

/*
https://www.yces.chc.edu.tw/english/enggirlname-all.htm

error_name
add
add ThirdParameterIsWrong too test error
add WithoutDepartmentName to
query

add Alice to test
add Zoe to dev
add Nancy to test
add Sue to dev
add Alex to test
add Ada to dev
add Molly to test
add Anne to dev
add Sal to test
add Olive to market
query dev
query test
query market
list

quit

*/