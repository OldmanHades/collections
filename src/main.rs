use std::collections::HashMap;

fn main() {
    // Initialize a vector of students with the first student "Timothy" (age 20)
    let mut students: Vec<Student> = vec![
        Student { name: "Timothy".to_string(), age: 20 }
    ];
    
    // Add another student "Lisa" (age 22) to the students vector
    students.push(Student { name: "Lisa".to_string(), age: 22 });

    // Validate the first student using assertions (equality now considers age)
    assert!(&students[0] == &Student { name: "Timothy".to_string(), age: 20 });
    assert!(students.get(0) == Some(&Student { name: "Timothy".to_string(), age: 20 }));
    
    // Confirm that accessing an out-of-bound index returns None
    assert!(students.get(1000) == None);
    
    // Iterate over the students vector and print out each student's information
    for student in students.iter() {
        println!("{} is {} years old", student.name, student.age);
    }
    
    // Create a HashMap to store course enrollments (course name to list of students)
    let mut enrollment: HashMap<String, Vec<Student>> = HashMap::new();
    enrollment.insert("Math".to_string(), students);
    
    // Retrieve and print details of students enrolled in the Math course
    if let Some(math_students) = enrollment.get("Math") {
        println!("Math course has {} students", math_students.len());
        for student in math_students {
            println!("Student: {} ({} years old)", student.name, student.age);
        }
    }
    
    // Remove the Math course enrollment and display the number of removed students
    if let Some(removed_students) = enrollment.remove("Math") {
        println!("Removed Math course enrollment with {} students", removed_students.len());
    }
} // End of main function

// Define the Student struct with PartialEq implementation for comparisons
#[derive(PartialEq, Eq)]
struct Student {
    name: String,
    age: u8,
}