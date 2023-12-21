use rand::Rng;

static mut ID:u32=0;
pub struct Person{
    pub id:u32,
    pub name: String,

    pub log_structures: Vec<(u32, u32)>, //vettore log dei luoghi visitati e il momento della visita
    
    pub pause: u32,    //time to move from one zone to another

    pub onboard: bool,
    pub waiting: bool,
    pub dist: u32,
    pub next: u32

}


impl Default for Person {
    fn default() -> Self {

        let mut rng = rand::thread_rng();

        unsafe{
            

            let p = Person{
                id: ID,
                name: String::from("person_name"),
                log_structures: vec![],
                pause: rng.gen_range(0..5),
                waiting: false,
                dist: 0,
                next: 0,
                onboard: false
            };

            ID+=1;

            return p;
        }

    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id: {} name: {} 
log structure: {:?}
next: {}
pause: {}
onboard: {}
waiting: {}
distance: {}", 
            self.id, self.name, self.log_structures, self.next, self.pause, self.waiting, self.onboard, self.dist)
    }
}

impl Person {

}