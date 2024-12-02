struct Student {
    id: u32,
    name: String,
    graduated: bool,
}

impl Student {
    // constructor to create new instance of student
    fn new(id: u32, name: String) -> Self {
        Self{id,name, graduated: false}
    }

    // display struct instance
    fn display(&self) -> String {
        format!("id: {} name: {} graduate status: {}", self.id, self.name, self.graduated)
    }
}

struct Professor {
    id: u32,
    name: String,
    graduated: bool,
}

impl Professor {

    fn new(id: u32, name: String) -> Self {
        Self{id, name, graduated: false}
    }

    fn display(&self) -> String {
        format!("id: {} name: {} graduate status: {}", self.id, self.name, self.graduated)
    }
}

// Define the Graduate trait
trait Graduate {
    fn graduate(&mut self);
}

impl Graduate for Professor {
    fn graduate(&mut self) {
        println!("A professor cannot graduate");
    }
}

impl Graduate for Student {
    fn graduate(&mut self) {
        self.graduated = true;
    }
    
}

fn main() {
    println!("Hello, world!");

    let mut student = Student::new(1, String::from("Cloud"));

    println!("{}", student.display());

    student.graduate();

    println!("{}", student.display());

    let mut professor = Professor::new(2, String::from("Sephiroth"));
    println!("{}", professor.display());
    // println!("{}",  student.graduate());
    professor.graduate();
    



    let this_string: Option<String> = Some(String::from("initial_string"));

    let new_string = if let Some(ref current_str) = this_string {
        String::from("new_string based on ") + current_str
    } else {
        String::from("default string") // Provide a default value if None
    };

    println!("{}", new_string);
}
