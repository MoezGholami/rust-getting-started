use std::fmt;
struct Student {
    name: &'static str,
    gpa: f32,
}

struct Color(u8, u8, u8);

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Student {} with GPA {} .", self.name, self.gpa);
    }
}

trait Human { // they are like java interface
    fn get_name(&self) -> String;
}

impl Human for Student {
    fn get_name(&self) -> String {
        return self.name.to_string();
    }
}

impl Student {
    fn increase_gpa(&mut self) {
        self.gpa += 0.1;
    }
    fn name_length(&self) -> i32 {
        return self.name.len() as i32;
    }
    fn max_possible_gpa() -> f32 {
        return 4.33;
    }
    fn get_void_student() -> Student {
        return Student{name: "void", gpa: 0.0};
    }
}

fn print_student_name_copy_req(s : Student) {
    println!("{}", s.name);
}

fn print_student_name(s : &Student) { // no copy, no change
    // s.gpa = 1.0; // illegal
    println!("{}", s.name);
}

pub fn main() {
    let mut s : Student = Student{name: "ali", gpa: 3.5};
    println!("maximum possible gpa is {}", Student::max_possible_gpa());
    println!("void student: {}", Student::get_void_student());
    println!("student is {}", s);
    s.increase_gpa();
    println!("student now is {} with name length of \"{}\": {}.", s, s.get_name(), s.name_length());
    s.name = "mamad";
    println!("student's name is \"{}\"", s.name);
    print_student_name(&s); print_student_name(&s); // we are lending s and ask to not to change it
    print_student_name_copy_req(s);
    // print_student_name_copy_req(s); // we are giving s away, we cannot use it anymore

    let mut green = Color(0,255,0);
    println!("my favorite color is ({},{},{})", green.0, green.1, green.2);
    green.1 = 180;
    println!("a bit darker actually ({},{},{})", green.0, green.1, green.2);
}
