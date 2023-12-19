use core::num;
use std::collections::HashSet;
use std::{env, primitive};
use std::path::PathBuf;

// mod structure;
use crate::structure::Structure;
use crate::person::Person;
use config::Config;
use rand::Rng;
use rand::seq::SliceRandom;


pub struct Park{
    pub name: String,
    pub structures: Vec<(Structure, (u32,u32))>, //vettore di strutture e posizione

    pub ppl: Vec<Person>,
    pub log_ppl: Vec<(u32, bool)>, //vettore log delle entrate id:true/false

    pub config: Config
}

impl Default for Park {
    fn default() -> Self {

        let s1:Structure = Structure{name: String::from("prima strutture"), ..Default::default()};
        let s2:Structure = Structure{name: String::from("seconds strutture"), ..Default::default()};

        let c = Config::builder()
                .add_source(config::File::with_name("config/park.json"))
                .build()
                .unwrap();


        Park{
            name: String::from("parco Outer Heaven"),
            structures: vec![ (s1,(1,1)), (s2,(2,4)) ],
            ppl: vec![],
            log_ppl: vec![],
            config: c
        }
    }
}


impl std::fmt::Display for Park {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {        
        write!(f, "park name: {}
        structures: {}
        people: {}", self.name, self.get_struct_string(), self.get_person_string())
    }
}

impl Park {
    pub fn init(&self) {

        println!("init Park class");
        println!("tet print park: {self}");

    }
    pub fn get_struct(&mut self) -> Vec<Structure>{
        let (v1, _): (Vec<Structure>, Vec<_>) = self.structures.into_iter().unzip();
        return v1;
    }
    fn get_struct_string(&self) -> String{
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
    fn get_person_string(&self) -> String{
        let mut s:String = String::from("");
        for t in &self.ppl{
            s.push_str(" id:");
            s.push_str(&t.id.to_string());
            s.push_str(" name:");
            s.push_str(&t.name);
            s.push_str("\n");
        }
        return s;
    }

    pub fn add_person(&mut self, number:u32){

        let mut rng = rand::thread_rng();

        for _ in 0..number{
            let mut p = Person{..Default::default()};
            p.moving=true;
            // p.structure=self.get_struct().choose(&mut rand::thread_rng());
            // let sample: Vec<Structure> = self.get_struct().choose(&mut rand::thread_rng()).unwrap();
            let mut st = self.get_struct();

            let c=st.choose(&mut rng);
            
            self.log_ppl.push((p.id.clone(),false)); //add random person
            self.ppl.push(p); //add random person
        }
    }
}
