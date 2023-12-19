use rand::Rng;


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
        let r:f32 = rng.gen_range(0.0..100000.0);

        Structure{
            name: format!("default_name_{r}"),
            position: (1 ,1),
            capacity: 10,
            rtt: 5,
            down_time: 2,
        
            line: 0, 
            line_log: vec![],

            attractivity: 5
        
        }
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {}\n
            position: {}, {}\n
            capacity: {}\n
            rtt: {}\n
            down_time: {}\n
        
            current line: {}\n
            line_log: {:?}\n

            attractivity: {}", 
            self.name, self.position.0, self.position.1, self.capacity, self.rtt, self.down_time, self.line, self.line_log, self.attractivity)
    }
}


impl Structure {

    fn init(&self) {

        println!("init Park class");

    }
 
}