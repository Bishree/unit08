
use std::collections::HashMap;

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
    let teams = vec![String::from("blue Team"), String::from("Yellow Team")];
    let initiate_Score = vec![10, 20];
    let scores:HashMap<_,_> = teams.into_iter().zip(initiate_Score.into_iter()).collect();

    let mut scoree = HashMap::new();
    scoree.insert(String::from("blue"), 10);
    scoree.insert(String::from("Yellow"),50);
    println!("{:?}",scoree);

}