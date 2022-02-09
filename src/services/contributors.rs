use std::path::Path;
use std::fs::File;


pub struct Contributor {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    url : String,
}

pub fn get_contributors()-> Vec<Contributor> {
    let json_file_path = Path::new("./data/contributors.json");
    let file = File::open(json_file_path);
    println!("{:?}",file);
    let first_contrib = Contributor { 
        name: String::from("Luca"), 
        url: String::from("Luca"),
    };       
    vec![first_contrib]
}