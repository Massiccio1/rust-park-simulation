use crate::structure::Structure;
use rand::Rng;

static mut ID:u32=0;
pub struct Person{
    pub id:u32,
    pub name: String,

    pub log_structures: Vec<(Structure, u32)>, //vettore log dei luoghi visitati e il momento della visita
    
    pub activity: f32,    //valore di attività da 0 a 1

    pub moving: bool,
    pub dist: u32,
    pub structure: Structure

}

impl Default for Person {
    fn default() -> Self {

        let mut rng = rand::thread_rng();
        let r:u32 = rng.gen_range(0..10);
        let s = Structure{..Default::default()};


        return unsafe{
            ID+=1;

            Person{
                id: ID,
                name: String::from("default_name"),
                log_structures: vec![],
                activity: rng.gen_range(0.0..1.0),
                moving: false,
                dist: 0,
                structure: s
            }
        }

    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id: {} name: {}", 
            self.id, self.name)
    }
}

impl Person {

    fn init(&self) {

        println!("init person class");

    }
 
}