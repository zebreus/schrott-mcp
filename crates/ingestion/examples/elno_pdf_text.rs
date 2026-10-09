fn main() {
    let bytes = std::fs::read(std::env::args().nth(1).expect("PDF path")).unwrap();
    print!("{}", pdf_extract::extract_text_from_mem(&bytes).unwrap());
}
