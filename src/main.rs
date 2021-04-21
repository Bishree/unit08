fn main() {
    enum SpereadSheetcell {
        Int(i32),
        Float(f64),
        Txt(String),
    }
    let row = vec![
        SpereadSheetcell::Float(80.20),
        SpereadSheetcell::Int(55),
        SpereadSheetcell::Txt(String::from("blue")),
    ];

    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = String::from("faksjfowifalf");
    for h in hello.chars(){
        println!("{}", h);
    }
}