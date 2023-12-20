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

pub fn dist(s_x:i32,s_y:i32,e_x:i32,e_y:i32) -> u32{
    let tmp =  ((s_x-e_x).pow(2)-(s_y-e_y).pow(2)) as f32;
    return f32::sqrt(tmp).ceil() as u32;
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
    pub fn get_ppl_mut(&mut self) -> &Vec<Person>{
        return &self.ppl;
    }

    pub fn random_struct(&self) -> &Structure{
        let mut rng = rand::thread_rng();

        let weighted_index: WeightedIndex<u32> = WeightedIndex::new(&self.get_weights()).expect("Invalid weights");
        // println!("pesi: {:?}",weighted_index);
        let st = &self.structures[weighted_index.sample(&mut rng)];

        return st;
        
    }
    pub fn random_struct_id(&self) -> u32{
        return self.random_struct().id;
        
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
            let weighted_index = WeightedIndex::new(&self.get_weights()).expect("Invalid weights");
            let st = &mut self.get_struct_mut()[weighted_index.sample(&mut rng)];
            // let mut st = self.get_struct_mut().choose_weighted_mut(&mut rng, ).unwrap();
            // p.next=self.random_struct();
            // st.push_log(p.id.clone());
            // st.line+=1;
            // st.line_ppl.push(p.id.clone());
            //p.log_structures.push((p.id,0));
            // p.dist
            p.next=st.id;
            let (e_x,e_y)=st.position; //final position of next
            p.dist=dist(0_i32,0_i32,e_x as i32,e_y as i32)+1;

            self.log_ppl.push((p.id,true)); //add random person
            self.ppl.push(p); //add random person

            
        }
    }

    pub fn add_structure(&mut self, number:u32){

        for _ in 0..number{
            let st: Structure = Structure{..Default::default()};
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

            // self.add_person(1);

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
        // let sts = ;

        for st in &mut self.structures{
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
                for _ in 0..batch{
                    
                    st.running=true;
                    // println!("pre error line_ppl: {:?}",st.line_ppl);
                    // println!("batch: {}",batch);
                    // println!("st.line: {}",st.line);
                    // println!("i: {}",i);
                    let ppl_id=st.line_ppl[0];
                    st.onboard_ppl.push( ppl_id);
                    // println!("ppl_id: {}", ppl_id);
                    // println!("{}", vec_to_string_ppl(&self.ppl));
                    self.ppl[ppl_id as usize].onboard=true;
                    st.line_ppl.remove(0);//always the first because it's the first that came
                    st.line-=1;
                    
                }
                st.finish_at = current+st.rtt;
            }
        }

        // let ppl=self.get_ppl_mut();

        for i in tbf{
            //free ppl tat got off the train
            // let p = &mut self.ppl[i as usize];
            self.ppl[i as usize].onboard=false;
            self.ppl[i as usize].waiting=false;
            let next = self.random_struct_id();
            self.ppl[i as usize].next=next;
            self.ppl[i as usize].pause=rng.gen_range(0..10);

            let (s_x,s_y)=self.ppl[i as usize].log_structures.last().unwrap().clone();
            let ( e_x, e_y) = self.structures[next as usize].position;



            self.ppl[i as usize].dist=dist(s_x as i32, s_y as i32, e_x as i32, e_y as i32);
        }

        for p in &mut self.ppl{

            // println!("{}-{}",p.name,p.id);
            // println!("{}-{}",p.pause,p.dist);
            // println!("-------------------------------------");

            // print!("helloooo");
            // p.name="test".to_string();
            //random if they chose another site
            // let max = self.get_struct().last().unwrap().id.clone();
            // let (mut s_x,mut s_y)=(0 as u32, 0 as u32);
            // let (mut e_x,mut e_y)=(0 as u32, 0 as u32);

            // let (mut e_x,mut e_y)=self.get_struct()[rng.gen_range(0..max) as usize].position.clone();


            // let new = self.random_struct_id();
            // (e_x,e_y)=self.get_struct_mut()[new as usize].position.clone();
    
            
             //final position of next

            if p.dist>0{            //mi avvicino
                p.dist-=1;
                // println!("ridotto distance")
            }
            if p.pause>0{            //posso rimettermi subito in fila
                p.pause-=1;
            }


            // if p.log_structures.is_empty(){        //se è vuoto
            //     (s_x,s_y)=p.log_structures.last().unwrap().clone(); //ultimo posto visitato come start
            //     p.dist=dist(e_x as i32,e_y as i32,s_x as i32,s_y as i32)+1; //+1 per travel time
            //     // p.next=new
            // }

            if !p.onboard && !p.waiting && p.pause<=0 && p.dist<=0{            
                //non è a bordo
                //non è in fila
                //non è in pausa
                //è arrivato a destinazione

                println!("libero");

                //allora lo aggiungo alla linea aggiorno le strutture visiteate
                p.waiting=true;
                p.log_structures.push( (p.next,current));
                self.structures[p.next as usize].line_ppl.push(p.id);
                self.structures[p.next as usize].line+=1;
                // let st = self.structures[new as usize];

            }



        }
    
        // for ppl in self.get_ppl_mut(){

        //     if ppl.ready && !ppl.onboard{
        //         // println!("mr. {} {} è afk", ppl.name, ppl.id);
        //     }
        // }
    }
}
