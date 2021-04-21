fn main() {
    enum spreadSheetcell {
        Int(i32),
        Float(f64),
        Text(String),

    }
    let row = vec![
        spreadSheetcell::Int(77),
        spreadSheetcell::Text(String::from("blue")),
        spreadSheetcell::Float(10.20),
    ];
    println!("{}",row);

}