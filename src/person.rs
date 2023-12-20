use crate::structure::Structure;
use rand::Rng;

static mut ID:u32=0;
pub struct Person{
    pub id:u32,
    pub name: String,

    pub log_structures: Vec<(u32, u32)>, //vettore log dei luoghi visitati e il momento della visita
    
    pub pause: u32,    //valore di attività da 0 a 1

    pub ready: bool,
    pub onboard: bool,
    pub dist: u32,
    pub structure_id: u32

}

impl Default for Person {
    fn default() -> Self {

        let mut rng = rand::thread_rng();

        return unsafe{
            ID+=1;

            Person{
                id: ID,
                name: String::from("person_name"),
                log_structures: vec![],
                pause: rng.gen_range(0..5),
                ready: true,
                dist: 0,
                structure_id: 0,
                onboard: false
            }
        }

    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id: {} name: {} log structure: {:?}", 
            self.id, self.name, self.log_structures)
    }
}

impl Person {

    fn init(&self) {

        println!("init person class");

    }
 
}