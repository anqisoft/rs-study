/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter08_exercies\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch08-03-hash-maps.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch08-03-hash-maps.html</tw>
 *
 * <en>
 * Created on Sun Nov 26 2023 13:57:30
 * Feature:
 *   Do the Chapter 8 exercises:
 *   1. Given a list of numbers, use a vector and return the median (the value in the middle of the array
 *      after arranging it) and the mode (the value that occurs most often; a hash map is helpful here) of the list.
 *   2. Convert the string to Pig Latin, that is, the first consonant of each word is moved to the end of
 *      the word and "ay" is added, so "first" becomes "irst-fay". Words that begin with a vowel have "hay"
 *      added to the end ("apple" becomes "apple-hay"). Remember UTF-8 encoding!
 *   3. Using hash maps and vectors, create a text interface that allows users to add employee names to
 *      company departments. For example, "Add Sally to Engineering" or "Add Amir to Sales."
 *      Then let the user get a list of all employees in a department, or a list of all employees in
 *      each department of the company in lexicographic order.
 * </en>
 *
 * <cn>
 * 创建：2023年11月26日 13:57:30
 * 功能：
 *   做第8章练习：
 *   1. 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值. 和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
 *   2. 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
 *      所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
 *      牢记 UTF-8 编码！
 *   3. 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
 *      例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
 *      或者公司每个部门的所有员工按照字典序排列的列表。
 * </cn>
 *

 * <tw>
 * 創建：2023年11月26日 13:57:30
 * 功能：
 *   做第8章練習：
 *   1. 給定一系列數字，使用 vector 並傳回這個列表的中位數（排列數組後位於中間的值. 和眾數（mode，出現次數最多的值；這裡哈希 map 會很有幫助）。
 *   2. 將字串轉換為 Pig Latin，也就是每一個單字的第一個輔音字母被移動到單字的結尾並增加 “ay”，
 *      所以 “first” 會變成 “irst-fay”。 母音字母開頭的單字則在結尾增加 “hay”（“apple” 會變成 “apple-hay”）。
 *      牢記 UTF-8 編碼！
 *   3. 使用哈希 map 和 vector，建立一個文字介面來允許使用者在公司的部門中增加員工的名字。
 *      例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。 接著讓使用者取得一個部門的所有員工的列表，
 *      或是公司每個部門的所有員工依照字典序排列的列表。
 * </tw>
*/

/*
 * <en>
 * Given a list of numbers, use a vector and return the median (the value in the middle of the array
 * after arranging it) and the mode (the value that occurs most often; a hash map is helpful here) of the list.
 * </en>
 *
 * <cn>
 * 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值. 和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
 * </cn>
 *
 * <tw>
 * 給定一系列數字，使用 vector 並傳回這個列表的中位數（排列數組後位於中間的值. 和眾數（mode，出現次數最多的值；這裡哈希 map 會很有幫助）。
 * </tw>
*/
fn exercise_01() {
    println!("\nIn exercise_01()");

    use std::{collections::HashMap, io};

    let mut numbers: Vec<i32> = Vec::with_capacity(100);
    println!("Please input some numbers, use the space to seperate them:");
    let mut words = String::new();
    loop {
        match io::stdin().read_line(&mut words) {
            Ok(_byte_count) => {
                for word in words[..].split_whitespace() {
                    if let Ok(value) = word.trim().parse::<i32>() {
                        numbers.push(value);
                    }
                }
            }
            Err(info) => println!("The error is '{info}', please reinput:"),
        }

        if numbers.len() > 0 {
            break;
        }

        // Clear the buffer
        words.clear();
    }

    numbers.sort();

    let median = numbers[numbers.len() / 2];

    let mut map = HashMap::new();
    for number in &numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_count_item = 0;
    for (number, count) in &map {
        if *count > max_count {
            max_count = *count;
            max_count_item = **number;
        }
    }
    println!("The median is {median}, and the mode is {max_count_item}, its count is {max_count}.");
}
/*
In exercise_01()
Please input some numbers, use the space to seperate them:
2 5 7 4 8 9 6 3 4 5 1 2 2 6 7 8 9
The median is 5, and the mode is 2, its count is 3.
*/

/*
 * <en>
 * Convert the string to Pig Latin, that is, the first consonant of each word is moved to the end of
 * the word and "ay" is added, so "first" becomes "irst-fay". Words that begin with a vowel have "hay"
 * added to the end ("apple" becomes "apple-hay"). Remember UTF-8 encoding!
 * </en>
 *
 * <cn>
 * 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
 * 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
 * 牢记 UTF-8 编码！
 * </cn>
 *
 * <tw>
 * 將字串轉換為 Pig Latin，也就是每一個單字的第一個輔音字母被移動到單字的結尾並增加 “ay”，
 * 所以 “first” 會變成 “irst-fay”。 母音字母開頭的單字則在結尾增加 “hay”（“apple” 會變成 “apple-hay”）。
 * 牢記 UTF-8 編碼！
 * </tw>
*/
fn exercise_02() {
    println!("\nIn exercise_02()");

    fn convert_to_pig_latin(from: &str) -> String {
        let from_chars = from.chars();
        // The kind of from_chars is core::str::iter::Chars
        // println!("The kind of from_chars is {}", type_of(&from_chars));

        let mut first_char = '\0';
        let mut result = String::new();
        for (index, char) in from_chars.into_iter().enumerate() {
            if index == 0 {
                first_char = char;

                if "aeiou".contains(first_char) {
                    return from.to_string() + "-hay";
                }
            } else {
                result.push(char);
            }
        }

        format!("{}-{first_char}ay", result)
    }

    // for word in ["first", "apple"] {
    for word in [
        "alley",
        "careless",
        "kissing",
        "stage",
        "immunity",
        "tooth",
        "god",
        "tight",
        "drugstore",
        "caramel",
        "phonetic",
        "warmth",
        "locust",
        "hide",
        "rain",
        "beautiful",
        "carbon",
        "warrior",
        "halting",
        "critical",
        "glass",
        "architect",
        "chieftain",
        "estate",
        "exception",
        "arm",
        "dual",
        "kill",
        "ability",
        "fluid",
        "divinity",
        "chapter",
        "vast",
        "ether",
        "heritage",
        "fight",
        "morsel",
        "affair",
        "hunger",
        "founder",
        "graceful",
        "cobra",
        "blame",
        "lime",
        "rare",
        "grinding",
        "whale",
        "bunny",
        "blinking",
        "fizz",
        "drone",
        "hostility",
        "grounds",
        "unit",
        "breath",
        "cruelty",
        "cosmic",
        "drum",
        "heartsick",
        "firecracker",
        "crew",
        "authority",
        "carbon",
        "moment",
        "twin",
        "executioner",
        "paste",
        "hickory",
        "crucifixion",
        "insect",
        "chef",
        "brown",
        "boarder",
        "cadaver",
        "conspiracy",
        "circus",
        "orthodox",
        "fork",
        "general",
        "pilgrim",
        "sparkler",
        "drowsy",
        "amputation",
        "sponge",
        "federal",
        "axis",
        "code",
        "kingdom",
        "luxury",
        "alibi",
        "magnetic",
        "pandemic",
        "bloodstream",
        "parasitic",
        "brick",
        "crooked",
        "elevator",
        "confrontational",
        "charisma",
        "fix",
    ] {
        println!("Converted '{}' to '{}'", word, convert_to_pig_latin(word))
    }
}

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
#[allow(dead_code)]
fn exercise_03_v1() {
    println!("\nIn exercise_03_v1()");

    /*
     * <en>
     * Welcome to this People List gadget!
     * You can enter in the following three different formats to use the corresponding function:
     * 1. Add employee list: add employee_name to department_name
     * 2. Query the employee list of a certain department: : query department_name
     * 3. Query the list of employees in all departments (arranged in dictionary order): list
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
    prompt.push_str("1. Add employee list: add employee_name to department_name\n");
    prompt.push_str("2. Query the employee list of a certain department: query department_name\n");
    prompt.push_str(
        "3. Query the list of employees in all departments (arranged in dictionary order): list\n",
    );
    prompt.push_str("You can also type quit to exit this gadget, thank you for using it!");
    let prompt = prompt;

    use std::{collections::HashMap, io};
    let mut departments: Vec<String> = Vec::with_capacity(20);
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
                                    error.push_str("Please input by this format: add employee_name to department_name");
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
                            println!("When you want to add an employee to a department, the employee name cannot be empty.");
                            continue;
                        } else if department_name.len() == 0 {
                            println!("When you want to add employees to a department, the department name cannot be empty.");
                            continue;
                        } else {
                            let people_names = map.entry(department_name.clone()).or_insert(vec![]);
                            people_names.push(employee_name);

                            if !departments.contains(&department_name) {
                                departments.push(department_name);
                            }
                        }
                    }
                    "query" => {
                        println!("exec: query {department_name}");
                        if department_name.len() == 0 {
                            println!("When querying the employee list of a certain department, the department name cannot be empty.");
                            continue;
                        } else {
                            print!("The employee list of {}:\t", &department_name);
                            let mut people_names: Vec<String> =
                                map.get(&department_name).unwrap_or(&Vec::new()).to_vec();
                            people_names.sort();
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
                    "quit" => {
                        println!("Now, quit it.");
                        break;
                    }
                    _ => {
                        println!("Error: unknown command name.");
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
#[allow(dead_code)]
fn exercise_03_v2() {
    println!("\nIn exercise_03_v2()");

    use std::{collections::HashMap, io};

    /*
     * <en>
     * Welcome to this People List gadget!
     * You can enter in the following three different formats to use the corresponding function:
     * 1. Add employee list: add employee_name to department_name
     * 2. Query the employee list of a certain department: : query department_name
     * 3. Query the list of employees in all departments (arranged in dictionary order): list
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
1. Add employee list: add employee_name to department_name
2. Query the employee list of a certain department: query department_name
3. Query the list of employees in all departments (arranged in dictionary order): list
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
                                "Please input by this format: add employee_name to department_name"
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

    loop {
        println!("{prompt}");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match parse_command(&input) {
                    Commands::Add(employee_name, department_name) => {
                        println!("exec: add {employee_name} to {department_name}");

                        if employee_name.len() == 0 {
                            println!("When you want to add an employee to a department, the employee name cannot be empty.");
                            continue;
                        } else if department_name.len() == 0 {
                            println!("When you want to add employees to a department, the department name cannot be empty.");
                            continue;
                        } else {
                            let people_names = map.entry(department_name.clone()).or_insert(vec![]);
                            people_names.push(employee_name);

                            if !departments.contains(&department_name) {
                                departments.push(department_name);
                            }
                        }
                    }
                    Commands::Query(department_name) => {
                        println!("exec: query {department_name}");
                        if department_name.len() == 0 {
                            println!("When querying the employee list of a certain department, the department name cannot be empty.");
                            continue;
                        } else {
                            print!("The employee list of {}:\t", &department_name);
                            let mut people_names: Vec<String> =
                                map.get(&department_name).unwrap_or(&Vec::new()).to_vec();
                            people_names.sort();
                            for name in people_names {
                                print!("{}\t", &name);
                            }
                            println!();
                        }
                    }
                    Commands::List => {
                        // order the items of departments
                        departments.sort();

                        println!("exec: list all");
                        for department_name in &departments {
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
                    Commands::Quit => {
                        println!("Now, quit it.");
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
     * 1. Add employee list: add employee_name to department_name
     * 2. Query the employee list of a certain department: : query department_name
     * 3. Query the list of employees in all departments (arranged in dictionary order): list
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
1. Add employee list: add employee_name to department_name
2. Query the employee list of a certain department: query department_name
3. Query the list of employees in all departments (arranged in dictionary order): list
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
                                "Please input by this format: add employee_name to department_name"
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

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter08_main";
    let start_chapter_line = "Chapter 08";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            exercise_01();
            exercise_02();
            exercise_03();
        };

        // let functions = vec![
        //     ("exercise_01", exercise_01 as fn()),
        //     ("exercise_02", exercise_02 as fn()),
        //     ("exercise_03", exercise_03 as fn()),
        // ];

        done_and_show_used_milliseconds(main_function_name, action);
        // done_and_show_used_seconds(main_function_name, action);

        // done_and_show_used_milliseconds_for_vec(functions.clone());
        // done_and_show_used_seconds_for_vec(functions.clone());

        println!();
        println!("  end: {:?}", Instant::now());
    });
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!(
        "Calling {name} tooks {:?} milliseconds.",
        now.elapsed().as_millis()
    );
}

#[allow(dead_code)]
fn done_and_show_used_seconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!(
        "Calling {name} tooks {:?} seconds.",
        now.elapsed().as_secs()
    );
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!(
            "Calling {name} tooks {:?} milliseconds.",
            now.elapsed().as_millis()
        );
    }
    println!("Total used {:?} milliseconds.", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn done_and_show_used_seconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!(
            "Calling {name} tooks {:?} seconds.",
            now.elapsed().as_secs()
        );
    }
    println!("Total used {:?} seconds.", start.elapsed().as_secs());
}

#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
