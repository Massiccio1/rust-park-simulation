use rand::Rng;

static mut ID:u32=0;

pub struct Structure{
    pub name: String,
    pub  position: (u32,u32) ,     //xy sulla mappa

    pub capacity: u32, //quante persone porta in una volta
    pub rtt: u32, //quanto ci mette a fare un giro
    pub down_time: u32, //tempo tra i giri, per far salire gente

    pub line: u32, //quante persone in line al momento
    pub line_log: Vec<u32>, //log per la fine

    pub attractivity: u32   //numero su una scala, viene messo poi in ordine

}


impl Default for Structure {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let x:u32 = rng.gen_range(0..10);
        let y:u32 = rng.gen_range(0..10);

        return unsafe { 
            
            ID+=1;

            Structure{
                name: format!("default_name_{ID}"),
                position: (x ,y),
                capacity: rng.gen_range(1..100),
                rtt: rng.gen_range(3..20),
                down_time: rng.gen_range(1..10),
            
                line: 0, 
                line_log: vec![],

                attractivity: rng.gen_range(0..5)
            
            }
        };
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {}
position: {}, {}
capacity: {}
rtt: {}
down_time: {}
current line: {}
line_log: {:?}
attractivity: {}", 
            self.name, self.position.0, self.position.1, self.capacity, self.rtt, self.down_time, self.line, self.line_log, self.attractivity)
    }
}


impl Structure {

    fn init(&self) {

        println!("init Park class");

    }
 
}