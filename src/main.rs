struct Rocket {
    name: String, 
    fuel: u32, 
    speed: u32,
}

impl Rocket { 
    
    fn status(&self) { 
        println!("Name: {}, Fuel: {}, Speed: {}", self.name, self.fuel, self.speed);
    }

    fn accelerate(&mut self) { 
        if self.fuel > 0 {
            self.fuel -= 10; 
            self.speed += 100; 
        } else {
            println!("No fuel!"); 
        }
    }

    fn new(name: &str) -> Self { 
        Self {
            name: String::from(name), 
            fuel: 100, 
            speed: 0,
        }
    }
}

fn main() {
    let mut falcon = Rocket::new("Zenit"); 

    falcon.status();
    falcon.accelerate();
    falcon.status();

    let mut fleet: Vec<Rocket> = Vec::new(); 

    fleet.push(Rocket::new("Apollo"));
    fleet.push(Rocket::new("Gemini"));  

    println!("Fleet Status:");

    for rocket in &fleet { 
        rocket.status();
    }

    for rocket in &mut fleet { 
        rocket.accelerate();
    }
    
    fleet.push(Rocket::new("Artemis"));  

    for rocket in &mut fleet { 
        rocket.accelerate();
    }
    
    for rocket in &fleet { 
        rocket.status();
    }
}
