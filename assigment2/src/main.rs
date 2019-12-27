#[derive(Debug)]
struct Student {
    name:String,
}

impl Student{
     
    fn student_data(name: String) -> Student{
        Student {
            name
        }        
    }
}

fn main() {
    
    let student_via_af = Student::student_data(String::from("Faizan Ali"));
    println!("Student instance via Associated Function {:#?}",student_via_af);

    let student_direct = Student {
            name: String::from("Faizan Ali")
    };

    println!("Student Instance Direct {:#?}",student_direct);
    //In a single program Direct instance can be used but if struct defined in some other module
    //we need to public each and every field of that struct, also we want to hide the definition
    //of struct for other applications so we use associated function for making an instance.
    

}
