use rand::Rng;

static mut ID:u32=0;

pub struct Structure{
    pub name: String,
    pub id: u32,
    pub  position: (u32,u32) ,     //xy sulla mappa

    pub capacity: u32, //quante persone porta in una volta
    pub rtt: u32, //quanto ci mette a fare un giro
    pub down_time: u32, //tempo tra i giri, per far salire gente

    pub line: u32, //quante persone in line al momento
    pub line_ppl: Vec<u32>,  //log di person ids
    pub onboard_ppl: Vec<u32>,  //log di person ids
    pub line_log: Vec<u32>, //log per la fine

    pub attractivity: u32,   //numero su una scala, viene messo poi in ordine
    pub finish_at: u32,   //numero su una scala, viene messo poi in ordine
    pub running: bool

}


impl Default for Structure {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let x:u32 = rng.gen_range(1..10);
        let y:u32 = rng.gen_range(1..10);

        unsafe { 
            
            

            let s=Structure{
                name: format!("default_name_{ID}"),
                id: ID,
                position: (x ,y),
                capacity: rng.gen_range(1..10),
                rtt: rng.gen_range(3..5),
                down_time: rng.gen_range(1..4),
            
                line: 0, 
                line_log: vec![],
                line_ppl: vec![],
                onboard_ppl: vec![],

                attractivity: rng.gen_range(5..10),
                finish_at: 0,
                running:false
            
            };
            ID+=1;

            return s;
        };
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {}
id: {}
position: {}, {}
capacity: {}
rtt: {}
down_time: {}
current line: {}
line_log: {:?}
line_ppl: {:?}
attractivity: {}
running: {}
onborard ppl: {:?}", 
            self.name, self.id, self.position.0, self.position.1, self.capacity, self.rtt, self.down_time, self.line, self.line_log,self.line_ppl,  self.attractivity, self.running, self.onboard_ppl)
    }
}


impl Structure {


    pub fn to_string(&self) -> String{
        let s=format!("{}", &self);
        return s;
    }

 
}

