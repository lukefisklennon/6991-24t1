trait VocalAnimal {
    fn get_sound(&self) -> String;
}

struct Cat {
    name: String,
}

impl Cat {
    fn new(name: String) -> Self {
        Cat { name }
    }
}

impl VocalAnimal for Cat {
    fn get_sound(&self) -> String {
        format!("Meow! My name is {}", self.name)
    }
}

struct Bird {
    wingspan: usize,
}

impl Bird {
    fn new(wingspan: usize) -> Self {
        Bird { wingspan }
    }

    pub fn fly(&self) {
        println!("Flew to {}m", self.wingspan * 100);
    }
}

impl VocalAnimal for Bird {
    fn get_sound(&self) -> String {
        "Chirp!".to_string()
    }
}

fn show_sound<T: VocalAnimal>(animal: &T) {
    println!("{}", animal.get_sound());
}

fn show_sounds(animals: &Vec<Box<dyn VocalAnimal>>) {
    for animal in animals {
        println!("{}", animal.get_sound());
    }
}

fn main() {
    let cat = Cat::new("Kip".to_string());
    let cat2 = Cat::new("Casper".to_string());
    let cat3 = Cat::new("Fluffy".to_string());
    let bird = Bird::new(10);

    let animals: Vec<Box<dyn VocalAnimal>> = vec![
        Box::new(cat),
        Box::new(cat2),
        Box::new(bird),
        Box::new(cat3),
    ];

    show_sounds(&animals);

    let cat4 = Cat::new("Chicken".to_string());
    let bird2 = Bird::new(20);

    show_sound(&cat4);
    show_sound(&bird2);

    bird2.fly();
}
