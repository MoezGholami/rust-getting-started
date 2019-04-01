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

fn print_student_name_copy_req(s : Student) {
    println!("{}", s.name);
}

fn print_student_name(s : &Student) { // no copy, no change
    // s.gpa = 1.0; // illegal
    println!("{}", s.name);
}

pub fn main() {
    let mut s : Student = Student{name: "ali", gpa: 3.5};
    println!("student is {}", s);
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
