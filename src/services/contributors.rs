pub struct Contributor {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub url : String,
}



pub fn get_contributors()-> Vec<Contributor> {

    vec![
    Contributor { 
        name: String::from("Luca Vignali"), 
        url: String::from("https://github.com/orgs/affidaty-blockchain/people/vignaliaffidaty"),
    },
    Contributor { 
        name: String::from("Stefano Setti"), 
        url: String::from("https://github.com/orgs/affidaty-blockchain/people/StefanoSetti"),
    },
    Contributor { 
        name: String::from("Aliaksei Vitsiuk"), 
        url: String::from("https://github.com/orgs/affidaty-blockchain/people/avitsiuk"),
    },
    
    ]
}