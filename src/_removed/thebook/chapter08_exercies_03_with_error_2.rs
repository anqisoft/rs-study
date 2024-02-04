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
    let mut prompt = String::new();
    prompt.push_str("Welcome to this People List gadget!\n");
    prompt.push_str("You can enter in the following three different formats to use the corresponding function:\n");
    prompt.push_str("1. Add employee list: Add employee_name to department_name\n");
    prompt.push_str("2. Query the employee list of a certain department: Query department_name\n");
    prompt.push_str(
        "3. Query the list of employees in all departments (arranged in dictionary order): List\n",
    );
    prompt.push_str("You can also type quit to exit this gadget, thank you for using it!");
    let prompt = prompt;

    use std::{collections::HashMap, io};
    let mut departments: Vec<&str> = Vec::with_capacity(20);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("{prompt}");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut command = String::new();
                let mut employee_name = String::new();
                let mut department_name = String::new();
                let mut error = String::new();

                for (index, item) in input.trim().split_whitespace().into_iter().enumerate() {
                    match index {
                        0 => {
                            // let item_lowercase = item.to_lowercase();
                            // command = &(item_lowercase.clone()[..]);
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
                                    error.push_str("Please input by this format: Add employee_name to department_name");
                                    break;
                                }
                            }
                        }
                        3 => {
                            if command == "add" {
                                department_name.push_str(item);
                            }
                        }
                        _ => (),
                    }
                }

                if error.len() > 0 {
                    println!("{error}");
                    continue;
                }

                let command = &command as &str;
                match command {
                    "add" => {
                        println!("exec: add {employee_name} to {department_name}");

                        if employee_name.len() == 0 {
                            println!("When you want to add an employee to a department, the employee name could not empty.");
                            continue;
                        } else if department_name.len() == 0 {
                            println!("When you want to add an employee to a department, the department name could not empty.");
                            continue;
                        } else {
                            let people_names = map
                                .entry(department_name.clone())
                                .or_insert(vec![])
                                // .to_vec()
                                ;

                            // *people_names.push(employee_name);
                            // type `()` cannot be dereferenced

                            // The type of people_names is &mut alloc::vec::Vec<alloc::string::String>
                            // println!("The type of people_names is {}", type_of(&people_names));
                            println!("The people_names is {:?}", people_names);
                            people_names.push(employee_name);
                            println!("The people_names is {:?}", people_names);
                            /*
                            add wang to dev
                            exec: add wang to dev
                            The people_names is []
                            The people_names is ["wang"]
                            */

                            if !departments.contains(&(&department_name as &str)) {
                                // non-primitive cast: `String` as `&str`
                                // departments.push(department_name.clone() as &str);

                                // non-primitive cast: `String` as `&str`
                                // departments.push(department_name as &str);

                                // `department_name` does not live long enough
                                // borrowed value does not live long enough

                                let department_name = department_name[..];
                                departments.push(department_name as &str);
                            }
                        }
                    }
                    "query" => {
                        println!("exec: query {department_name}");
                        if department_name.len() == 0 {
                            println!("When you want to query the employee list of a department, the department name could not empty.");
                            continue;
                        } else {
                            print!("The employee list of {}:\t", &department_name);
                            let people_names: Vec<String> = map
                                .get(&(department_name.to_string()))
                                .unwrap_or(&Vec::new())
                                .to_vec();
                            for name in people_names {
                                print!("{}\t", &name);
                            }
                            println!();
                        }
                    }
                    "list" => {
                        // order the items of departments
                        departments.sort();

                        println!("exec: list all");
                        for department_name in &departments {
                            print!("{}:\t", &department_name);
                            let people_names: Vec<String> = map
                                .get(&(department_name.to_string()))
                                .unwrap_or(&Vec::new())
                                .to_vec();
                            for name in people_names {
                                print!("{}\t", &name);
                            }
                        }
                    }
                    "quit" => {
                        println!("Now, quit it.");
                        break;
                    }
                    _ => (),
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
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src\main.rs:425:37
    |
425 | ...                   let department_name = department_name[..];
    |                           ^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature
help: consider borrowing here
    |
425 |                                 let department_name = &department_name[..];
    |                                                       +

error[E0605]: non-primitive cast: `str` as `&str`
   --> src\main.rs:426:50
    |
426 | ...                   departments.push(department_name as &str);
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
    |
help: consider borrowing the value
    |
426 |                                 departments.push(&department_name as &str);
    |                                                  +

Some errors have detailed explanations: E0277, E0605.
*/