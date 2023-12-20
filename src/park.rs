use core::num;
use std::collections::HashSet;
use std::{env, primitive};
use std::path::PathBuf;

// mod structure;
use crate::structure::Structure;
use crate::person::Person;
use config::Config;
use rand::Rng;
use rand::distributions::{WeightedIndex, Distribution};
use rand::seq::SliceRandom;
use std::fs;
use std::{thread, time};

pub struct Park{
    pub name: String,
    pub structures: Vec<Structure>,
    pub map: Vec<(u32, (u32,u32))>, //vettore di id strutture e posizione

    pub ppl: Vec<Person>, //
    pub log_ppl: Vec<(u32, bool)>, //vettore log delle entrate id:true/false

    pub config: Config
}

impl Default for Park {
    fn default() -> Self {

        // let s1:Structure = Structure{name: String::from("check in"), running:true, onboard_ppl: vec![] , attractivity: 0, finish_at: 0, position: (0,0), id:0, capacity: 1, line_ppl: vec![] ,rtt:0,down_time:0,line:0,line_log:vec![]};
        // let s2:Structure = Structure{name: String::from("seconds strutture"), ..Default::default()};

        let c = Config::builder()
                .add_source(config::File::with_name("config/park.json"))
                .build()
                .unwrap();


        Park{
            name: String::from("parco Outer Heaven"),
            //ap: vec![ (s1.id.clone(),(1,1)) ],
            map: vec![  ],
            // structures: vec![s1],
            structures: vec![],
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
number of ppl: {}
people: {}", self.name, self.get_struct_string(), self.ppl.len(), self.get_person_string())
    }
}

pub fn vec_to_string_str(vs:&Vec<Structure>) -> String{
    let mut s:String = String::from("");
        for st in vs{
            s.push_str(st.to_string().as_str());
            s.push_str("\n---\n");
        }
        return s;
}

pub fn vec_to_string_ppl(vs:&Vec<Person>) -> String{
    let mut s:String = String::from("");
        for st in vs{
            s.push_str(st.to_string().as_str());
            s.push_str("\n---\n");
        }
        return s;
}
// pub fn dist(p:&Person,st:&Structure) -> u32{
//     return 
// }

impl Park {
    pub fn init(&self) {

        println!("init Park class");
        println!("tet print park: {self}");

    }
    
    pub fn get_struct(&self) -> &Vec<Structure>{
        return &self.structures;
    }
    pub fn get_struct_mut(&mut self) -> &mut Vec<Structure>{
        return &mut self.structures;
    }
    pub fn get_ppl(&self) -> &Vec<Person>{
        return &self.ppl;
    }
    pub fn get_ppl_mut(&mut self) -> &mut Vec<Person>{
        return &mut self.ppl;
    }

    fn get_struct_string(&self) -> String{
        let mut s:String = String::from("");
        for t in &self.structures{
            s.push_str(&t.to_string());
            s.push_str("\n\n")
        }
        return s;
    }
    fn get_person_string(&self) -> String{
        // let mut s:String = String::from("");
        // for t in &self.ppl{
        //     s.push_str(" id:");
        //     s.push_str(&t.id.to_string());
        //     s.push_str(" name:");
        //     s.push_str(&t.name);
        //     s.push_str(" log:");
        //     s.push_str(&t.name);
        //     s.push_str("\n\n");
        // }        
        // return s;
        return vec_to_string_ppl(&self.ppl);

        
    }

    pub fn add_person(&mut self, number:u32){

        let mut rng = rand::thread_rng();

        for _ in 0..number{
            let mut p = Person{..Default::default()};
            p.ready=false; //already has  a target
            let weighted_index = WeightedIndex::new(&self.get_weights()).expect("Invalid weights");
            let st = &mut self.get_struct_mut()[weighted_index.sample(&mut rng)];
            // let mut st = self.get_struct_mut().choose_weighted_mut(&mut rng, ).unwrap();
            p.structure_id=st.id.clone();
            // st.push_log(p.id.clone());
            st.line+=1;
            st.line_ppl.push(p.id.clone());
            p.log_structures.push((0,0));
            // p.dist

            self.log_ppl.push((p.id,true)); //add random person
            self.ppl.push(p); //add random person
        }
    }

    pub fn add_structure(&mut self, number:u32){
        let mut rng = rand::thread_rng();

        for _ in 0..number{
            let mut st: Structure = Structure{..Default::default()};
            self.structures.push(st);

        }
    }

    pub fn get_weights(&self)-> Vec<u32>{
        let mut w:Vec<u32>=vec![];
        for s in self.get_struct(){
            w.push(s.attractivity);
        }
        return w;
    }

    pub fn dump(&self){

        let data = vec_to_string_str(&self.structures);
        fs::write("files/structures.txt", data).expect("Unable to write file");

        let ppl = vec_to_string_ppl(&self.ppl);
        fs::write("files/people.txt", ppl).expect("Unable to write file");
        
        let park = format!("{}",&self);
        fs::write("files/park.txt", park).expect("Unable to write file");
    }

    pub fn run(&mut self, iter:u32){
        for it in 0..iter{
            println!("epoc: {}",it);

            self.add_person(5);

            self.advance(it);



            self.dump();


            println!("sleeping...");
            let duration = time::Duration::from_secs(2);
            thread::sleep(duration);
        }
    }

    pub fn advance(&mut self, current:u32){

        let mut tbf:Vec<u32>=vec![];

        let mut rng = rand::thread_rng();
        let sts = self.get_struct_mut();

        for st in sts{
            if st.running && st.finish_at<=current{     //stava andando e ha finito
                // println!("[time: {current}] {} finished", st.name.as_str());
                st.running=false;
                
                for p in &st.onboard_ppl{ //libero le persone
                    tbf.push(*p);
                }
                st.onboard_ppl=vec![]; //svuoto il treno

            }
            // println!(" °°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°°");
            if !st.running && st.finish_at+st.down_time<=current && st.line>0{   //è fermo e adesso è pronto e ci sono persone
                //pronto e ci sono persone in line
                //sposto le prsone dalla fila a l treno e cambio stato in unning
                //aggiungo pressimo ready
                let batch= std::cmp::min(st.line,st.capacity);  //massimo disponibile
                for i in 0..batch{
                    
                    st.running=true;
                    // println!("pre error line_ppl: {:?}",st.line_ppl);
                    // println!("batch: {}",batch);
                    // println!("st.line: {}",st.line);
                    // println!("i: {}",i);
                    st.onboard_ppl.push( st.line_ppl[0]);
                    st.line_ppl.remove(0);//always the first because it's the first that came
                    st.line-=1;
                }
                st.finish_at = current+st.rtt;
            }
        }

        let ppl=self.get_ppl_mut();

        for i in tbf{
            //free ppl tat got off the train
            let p = &mut ppl[i as usize];
            p.ready=true;
            p.onboard=false;
        }

        for p in ppl{
            //random if they chose another site
        }
    
        // for ppl in self.get_ppl_mut(){

        //     if ppl.ready && !ppl.onboard{
        //         // println!("mr. {} {} è afk", ppl.name, ppl.id);
        //     }
        // }
    }
}
