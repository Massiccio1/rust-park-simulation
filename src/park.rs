use std::env;
use std::path::PathBuf;

// mod structure;
use crate::structure::Structure;
use crate::person::Person;
use config::Config;
use rand::Rng;


pub struct Park{
    pub name: String,
    pub structures: Vec<(Structure, (u32,u32))>, //vettore di strutture e posizione

    pub ppl: Vec<Person>,
    pub log_ppl: Vec<(Person, bool)>, //vettore log delle entrate

    pub config: Config
}

impl Default for Park {
    fn default() -> Self {

        let s:Structure = Structure{name: String::from("prima strutture"), ..Default::default()};

        let mut c = Config::builder()
                .add_source(config::File::with_name("config/park.json"))
                .build()
                .unwrap();


        Park{
            name: String::from("parco Outer Heaven"),
            structures: vec![ (s,(1,1)) ],
            ppl: vec![],
            log_ppl: vec![],
            config: c
        
        }
    }
}


impl std::fmt::Display for Park {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {        
        write!(f, "park name: {}\nstructures: {}\npeople: wip", self.name, self.get_struct_string())
    }
}

impl Park {
    pub fn init(&self) {

        println!("init Park class");
        println!("tet print park: {self}");

    }
    pub fn get_struct_string(&self) -> String{
        let mut s:String = String::from("");
        for t in &self.structures{
            s.push_str(&t.0.to_string());
            s.push_str(" x:");
            s.push_str(&t.1.0.to_string());
            s.push_str(" y:");
            s.push_str(&t.1.0.to_string());
            s.push_str("\n")
        }
        return s;
    }

}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
