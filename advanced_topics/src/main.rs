/*
    Handling circular references in Rust can be tricky due to its strict ownership and borrowing rules.
    A circular reference occurs when two or more objects reference each other, creating a cycle that can
    inhibit proper memory deallocation (potentially leading to memory leaks). The commented-out code below
    is a failure example using explicit lifetimes. It demonstrates how trying to maintain bidirectional
    relationships with standard references results in conflicts between mutable and immutable borrows.

    The failure example (kept for illustration) shows that directly referencing one object from another,
    without using smart pointers, leads to borrow-checker errors:

    // struct Student<'a> {
    //     name: String,
    //     courses: Vec<&'a Course<'a>>,
    // }
    //
    // impl<'a> Student<'a> {
    //     fn new(name: &str) -> Student<'a> {
    //         Student {
    //             name: name.into(),
    //             courses: Vec::new(),
    //         }
    //     }
    // }
    //
    // struct Course<'a> {
    //     name: String,
    //     students: Vec<&'a Student<'a>>,
    // }
    //
    // impl<'a> Course<'a> {
    //     fn new(name: &str) -> Course<'a> {
    //         Course {
    //             name: name.into(),
    //             students: Vec::new(),
    //         }
    //     }
    //
    //     fn add_student(&'a mut self, student: &'a mut Student<'a>) {
    //         student.courses.push(self); // Mutable borrow begins here
    //         self.students.push(student); // Immutable borrow conflict occurs here
    //         // This approach fails because the borrow checker cannot reconcile the mutable and immutable borrows.
    //     }
    // }
    //
    // fn main() {
    //     let kenry = Student::new("Kenry");
    //     let course = Course::new("Rust Course");
    //     course.add_student(kenry);
    // }

    To overcome these issues, the working example below uses Rc (Reference Counted pointers) along with
    RefCell (for interior mutability). This combination relaxes the borrowing rules at runtime, allowing
    bidirectional associations. However, be cautious: while Rc and RefCell provide flexibility, they also
    bypass some of the compiler’s safety checks—misuse can lead to runtime borrow errors.

    Best practices:
      - Redesign data structures to avoid circular dependencies when possible.
      - If circular references are required, consider using Weak pointers to break cycles and prevent
        potential memory leaks.
*/
#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

// We use Rc (Reference Counted pointers) to allow multiple owners of the same data.
// RefCell provides interior mutability, letting us borrow and modify data even when
// it’s wrapped in an Rc. Together, they enable many-to-many relationships, such as a
// Student enrolling in multiple Courses and a Course containing multiple Students.
// Note that this approach defers certain checks to runtime, so it's important to manage
// borrows carefully to avoid runtime errors.

struct Student {
    name: String,
    // Each student holds a list of courses they are enrolled in.
    courses: Vec<Rc<RefCell<Course>>>,
}

struct Course {
    name: String,
    // Each course holds a list of students enrolled in it.
    students: Vec<Rc<RefCell<Student>>>,
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new(),
        }
    }
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new(),
        }
    }

    // Adds a student to the course, establishing the many-to-many relationship.
    // The student is added to the course's students list and, conversely, the course
    // is added to the student's courses list.
    fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

mod database_normalization;

fn main() {
    let kenry = Rc::new(RefCell::new(Student::new("Kenry")));
    let kary = Rc::new(RefCell::new(Student::new("Kary")));
    let course = Course::new("Rust Course");
    let magic_course = Rc::new(RefCell::new(course));

    Course::add_student(magic_course.clone(), kenry);
    Course::add_student(magic_course.clone(), kary);

    database_normalization::normalization_demo();
}
