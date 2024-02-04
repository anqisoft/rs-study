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
    let mut department_and_people_map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("{prompt}");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // let words = input.trim().split_whitespace();
                // // The type of words: core::str::iter::SplitWhitespace
                // println!("The type of words: {}", type_of(&words));

                // let words = input.trim().split_whitespace().into_iter();
                // // The type of words: core::str::iter::SplitWhitespace
                // println!("The type of words: {}", type_of(&words));

                let mut words = input.trim().split_whitespace().into_iter().enumerate();
                // The type of words: core::iter::adapters::enumerate::Enumerate<core::str::iter::SplitWhitespace>
                println!("The type of words: {}", type_of(&words));

                let (_, command) = words.next();
                // Option<(usize, &str)>
                println!(
                    "The type of command: {}, and the value is {:?}.",
                    type_of(&command),
                    &command
                );

                // let command: &str = match words.next() {
                //     // Some(value) => &(value.to_lowercase()[..]),
                //     Some(value) => &(value.to_lowercase()[..]),
                //     None => {
                //         continue;
                //     }
                // };

                let command = "quit";
                match command {
                    "add" => {
                        println!("add");
                    }
                    "query" => {
                        println!("query");
                    }
                    "list" => {
                        // order the items of departments
                        departments.sort();

                        println!("List all:");
                        for department_name in &departments {
                            print!("{}:\t", &department_name);
                            let people_names: Vec<String> = department_and_people_map
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
            }
            Err(error) => {
                println!("The error is '{error}.");
                continue;
            }
        }
 
   }
}