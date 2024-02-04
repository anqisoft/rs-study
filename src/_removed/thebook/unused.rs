
#[derive[Debug]]
enum Lang {
  En,
  ZhCn,
  ZhTw,
}    
#[derive[Debug]]
enum Commands {
  Add(String, String),
  Query(String),
  List,
  Quit,
  ChangeLang(Lang),
}

#[derive(Debug)]
struct Command {
  command: String,
  employee_name: String,
  department_name: String,
}