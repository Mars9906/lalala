
struct Student {
    name: String,
    age: u32,
}

struct Teacher {
    name: String,
    sub: String,
    age: u32,
}

trait SchoolName {
    fn get_school_name(&self) ->String {            
        String::from("csust")
    }
}

impl SchoolName for Teacher{

}

impl SchoolName for Student{
    
}

fn main(){
    let stu1 = Student {
        name : "zl".to_string(),
        age: 21,
    };
    let tea1 = Teacher {
        name : "tjf".to_string(),
        sub: "math".to_string(),
        age: 50,
    };
    println!("hhh");
    let stu1_school_name=stu1.get_school_name();
    let tea1_school_name=tea1.get_school_name();
    let sum_name = format!("student's school :{}  and teacher's school : {}",stu1_school_name,tea1_school_name);
    println!("{}",sum_name.len());
    println!("{}",sum_name);
    println!("{}   {}",stu1_school_name,tea1_school_name);
 }