use std::{
    collections::HashMap,
    path::PathBuf, 
    io::{Read, Write}
};

use crate::error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Food {
    id: u64,
    name: String,
    stocks: u64,
    price: u128
}

pub struct Foods {
    foods: HashMap<u64, Food>
}

impl Foods {
    pub fn new() -> Self { Self { foods: HashMap::new() } }

    pub fn next_id(&self) -> u64 {
        let mut ids: Vec<_> = self.foods.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1
        }
    }

    pub fn into_vec(mut self) -> Vec<Food> {
        let mut fds: Vec<_> = self.foods.drain().map(|kv| kv.1).collect();
        fds.sort_by_key(|fd| fd.id);
        fds
    }

    fn add(&mut self, food: Food) {
        self.foods.insert(food.id, food);
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let temp: HashMap<_, _> = self.foods.clone().drain().filter(|kv| kv.1.name != name ).collect();

        if temp.len() == self.foods.len() {
            return false
        }

        self.foods = temp;
        true
    }

    pub fn sell(mut self, food: Food) -> u128 {
        let temp: Vec<_> = self.foods.clone().drain().filter(|kv| kv.1.name == food.name ).collect();

        let new = parse_food(&format!("{},{},{},{}", temp.first().unwrap().1.id, 
                                                                                temp.first().unwrap().1.name, 
                                                                                temp.first().unwrap().1.stocks - food.stocks, 
                                                                                temp.first().unwrap().1.price)
        );

        self.add(new.unwrap());

        match save_foods(self){
            Ok(()) => (),
            Err(_e) => println!("Save foods error...")
        };

        temp.first().unwrap().1.price * (food.stocks as u128)
    }

    pub fn insert(&mut self, mut food: Food) {
        let new: HashMap<_, _> = self.foods.clone().drain().filter(|kv| kv.1.name != food.name).collect();

        if new.len() != self.foods.len() { // Food is in Data
            let food_data: Vec<_> = self.foods.clone().drain().filter(|kv| kv.1.name == food.name).collect();
            food.id = food_data.get(0).unwrap().0;
            food.price = food_data.get(0).unwrap().1.price;
            food.stocks += &food_data.get(0).unwrap().1.stocks;
            println!("Berhasil mengubah data makanan, {}, dengan stock {}, dan harga Rp {},00", food.name, food.stocks, food.price)
        }else { // Equals => Food not found
            food.id = self.next_id();
            println!("Berhasil menambahkan makanan baru, {}, dengan stock {}, dan harga Rp {},00", food.name, food.stocks, food.price);
        }

        self.add(food);
    }

    pub fn is_empty(self) -> bool {
        self.is_empty()
    }
}

impl Default for Foods {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_food(food: &str) -> Result<Food, error::ParseError> {
    let strings: Vec<&str> = food.split(',').collect();

    let id = match strings.first() {
        Some(id) => id.parse::<u64>()?,
        None => return Err(error::ParseError::InvalidInput("id - foods"))
    };

    let name = match strings.get(1).filter(|name| !name.is_empty()) {
        Some(name) => name.to_string(),
        None => return Err(error::ParseError::MissingField("name"))
    };

    let stocks = match strings.get(2) {
        Some(stocks) => stocks.parse::<u64>()?,
        None => return Err(error::ParseError::InvalidInput("stocks"))
    };

    let price = match strings.get(3) {
        Some(price) => price.trim().parse::<u128>()?,
        None => return Err(error::ParseError::InvalidInput("price"))
    };

    Ok(Food { id, name, stocks, price})
}

fn parse_foods(foods: String, verbose: bool) -> Foods {
    let mut fds = Foods::new();

    for(i, food) in foods.split('\n').enumerate() {
        if !food.is_empty() {
            match parse_food(food) {
                Ok(fd) => fds.add(fd),
                Err(err) => if verbose { println!("Error on {}: {} - {}\n", i+1, err, food); }
            }
        }
    }
    fds
}

pub fn load_foods(verbose: bool) -> std::io::Result<Foods> {
    let mut file = std::fs::File::open(PathBuf::from("bin/food.csv"))?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_foods(buffer, verbose))
}

pub fn save_foods(foods: Foods) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(PathBuf::from("bin/food.csv"))?;

    file.write_all( b"id,name,stock,price\n")?;

    file.flush()?;

    for food in foods.into_vec().into_iter() {
        file.write_all(format!("{},{},{},{}\n", food.id, food.name, food.stocks, food.price).as_bytes())?;
    }

    file.flush()?;

    Ok(())
}