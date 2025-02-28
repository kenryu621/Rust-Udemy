//! This module demonstrates a normalized (database redesign) approach to managing many-to-many
//! relationships between Students and Courses. Instead of having direct circular references,
//! an intermediate Enrollment struct is introduced to decouple the relationships. This is similar
//! to database normalization techniques, where a join table helps avoid complex cyclic dependencies
//! and facilitates independent management of related entities.

/// Represents a student with a name.
struct Student {
    name: String,
}

impl Student {
    /// Returns a list of course names that the student is enrolled in, based on the enrollments in the given platform.
    ///
    /// # Arguments
    ///
    /// * `platform` - A Platform instance containing enrollment records.
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

/// Represents a course with a name.
struct Course {
    name: String,
}

/// Represents an enrollment record linking a student and a course. This acts as a join table in the
/// normalized design, decoupling direct references between Students and Courses.
struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a> {
    /// Creates a new Enrollment record linking a student and a course.
    ///
    /// # Arguments
    ///
    /// * `student` - A reference to a Student.
    /// * `course` - A reference to a Course.
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

/// Represents the platform that manages enrollments. This structure maintains a list of enrollment
/// records, each representing a relationship between a Student and a Course.
struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    /// Creates a new, empty Platform.
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }

    /// Enrolls a student in a course by creating a new enrollment record.
    ///
    /// # Arguments
    ///
    /// * `student` - A reference to a Student.
    /// * `course` - A reference to a Course.
    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}

/// Demonstrates the normalized approach by enrolling a student in a course and printing the courses
/// that the student is taking.
pub fn normalization_demo() {
    println!("Normalization demo:");
    let kenry = Student {
        name: "Kenry".into(),
    };
    let course = Course {
        name: "Intro to Rust".into(),
    };
    let mut p = Platform::new();
    p.enroll(&kenry, &course);

    for c in kenry.courses(p) {
        println!("Kenry is taking {}", c);
    }
}
