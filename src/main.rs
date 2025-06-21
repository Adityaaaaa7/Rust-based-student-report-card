use std::io;

fn main() {
    let student_name = input("Enter student's name: ");
    let total_marks: f64 = input("Enter total marks obtained: ")
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let num_subjects: f64 = input("Enter number of subjects: ")
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    println!("\nStudent: {}", student_name.trim());
    println!("Average marks: {:.2}", average);
    println!("Grade: {}", grade);
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read input");
    input_str
}

fn calculate_average(total: f64, subjects: f64) -> f64 {
    total / subjects
}

fn assign_grade(avg: f64) -> char {
    match avg {
        avg if avg >= 90.0 => 'A',
        avg if avg >= 75.0 => 'B',
        avg if avg >= 60.0 => 'C',
        _ => 'D',
    }
}
