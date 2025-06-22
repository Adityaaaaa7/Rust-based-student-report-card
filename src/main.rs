use std::fs::File;
use std::io::{self, BufWriter};
use printpdf::*;

fn main() {
    // === User Input ===
    let student_name = input("Enter student's name: ");
    let total_marks: f64 = input("Enter total marks obtained:")
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let num_subjects: f64 = input("Enter number of subjects:")
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    // === Generate clean, well-formatted PDF ===
    let (doc, page1, layer1) = PdfDocument::new("Student Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
    let font = doc.add_external_font(File::open(font_path).expect("Font not found")).unwrap();

    // Start from top of page
    let mut y_position = Mm(270.0);
    let line_spacing = Mm(10.0);

    // Title in bold (draw twice with offset to simulate bold)
    current_layer.use_text("Student Report", 18.0, Mm(20.0), y_position, &font);
    y_position -= line_spacing * 2.0;

    // Add student info
    let lines = vec![
        format!("Name      : {}", student_name.trim()),
        format!("Average   : {:.2}", average),
        format!("Grade     : {}", grade),
    ];

    for line in lines {
        current_layer.use_text(&line, 14.0, Mm(20.0), y_position, &font);
        y_position -= line_spacing;
    }

    // Save PDF
    let mut output = BufWriter::new(File::create("student_report.pdf").unwrap());
    doc.save(&mut output).expect("Failed to save PDF");

    println!("\n✅ PDF generated: student_report.pdf");
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
