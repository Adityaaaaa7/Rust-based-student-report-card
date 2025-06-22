#  Rust-Based Student Report Card Generator

A simple Rust-based console application to manage student academic records, calculate grades, and generate a PDF report card.

##  Features

- Input student's name, total marks, and number of subjects.
- Automatically calculates:
  - Average marks
  - Grade based on average:
    - A: 90+
    - B: 75–89
    - C: 60–74
    - D: Below 60
- Generates a professional PDF report card using `printpdf`.

##  Tech Stack

- Language: [Rust](https://www.rust-lang.org/)
- PDF Generation: [`printpdf`](https://docs.rs/printpdf/latest/printpdf/)
- Font: Roboto-Regular.ttf

##  Installation

```bash
# Clone the repository
git clone https://github.com/Adityaaaaa7/Rust-based-student-report-card.git
cd Rust-based-student-report-card

# Build and run the project
cargo run
