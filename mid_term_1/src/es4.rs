use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    id: u32,
}

impl Student {
    fn new(n: &str, id: u32) -> Self {
        Student {
            name: n.to_owned(),
            id,
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Student {{ Name: {:?}, ID: {:?} }}", self.name, self.id)
    }
}
#[allow(dead_code)]
struct University {
    name: String,
    students: Vec<Student>,
}

impl University {
    fn new(uni: &str, students: &[Student]) -> Self {
        University {
            name: uni.to_owned(),
            students: Vec::from(students),
        }
    }

    fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
        let index = self.students.iter().position(|s| s.id == id);

        match index {
            Some(i) => Ok(self.students.swap_remove(i)),
            None => Err("ID not found."),
        }
    }
}

pub fn es4() {
    let student1 = Student::new("BrumBrum", 13590);
    println!("{}", student1);

    let student2 = Student::new("John Doe", 24680);
    let student3 = Student::new("Jane Smith", 35791);

    let students = vec![student1.clone(), student2.clone(), student3.clone()];
    let mut university = University::new("Università del Nulla", &students);

    match university.remove_student(student2.id) {
        Ok(removed_student) => println!("Removed student: {}", removed_student),
        Err(err) => println!("Error: {}", err),
    }
    match university.remove_student(12345) {
        Ok(removed_student) => println!("Removed student: {}", removed_student),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_display() {
        let student = Student::new("BrumBrum", 13590);
        assert_eq!(
            format!("{}", student),
            "Student { Name: \"BrumBrum\", ID: 13590 }"
        );
    }

    #[test]
    fn test_university_remove_student() {
        let student1 = Student::new("BrumBrum", 13590);
        let student2 = Student::new("John Doe", 24680);
        let student3 = Student::new("Jane Smith", 35791);

        let students = vec![student1.clone(), student2.clone(), student3.clone()];
        let mut university = University::new("Università del Nulla", &students);

        // Test case where the student is in the struct
        assert_eq!(university.remove_student(student2.id), Ok(student2.clone()));
        assert_eq!(university.students.len(), 2);

        // Test case where the student is not in the struct
        assert_eq!(university.remove_student(99999), Err("ID not found."));
        assert_eq!(university.students.len(), 2);
    }
}
