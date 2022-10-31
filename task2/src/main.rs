use std::io::{self, Write};
use rand::Rng;

trait Print {
    fn print_all(&self);
}

#[derive(PartialEq, Debug)]
enum Weapon {
    Hand,
    Wood,
    Copper,
    Iron,
    Gold
}

fn weapon_to_string(w: &Weapon) -> String {
    match w {
        Weapon::Hand => String::from("Hand"),
        Weapon::Wood => String::from("Wood"),
        Weapon::Copper => String::from("Copper"),
        Weapon::Iron => String::from("Iron"),
        Weapon::Gold => String::from("Gold"),
    }
}

#[derive(Debug)]
struct Equipment {
    weapon: Weapon,
    quantity: u8
}

struct Equipments {
    equips: Vec<Equipment>
}

impl Equipments {
    fn new() -> Self {
        Self {
            equips: vec![
                Equipment {
                    weapon: Weapon::Hand,
                    quantity: 0
                },
                Equipment {
                    weapon: Weapon::Wood,
                    quantity: 0
                },
                Equipment {
                    weapon: Weapon::Copper,
                    quantity: 0
                },
                Equipment {
                    weapon: Weapon::Iron,
                    quantity: 0
                },
                Equipment {
                    weapon: Weapon::Gold,
                    quantity: 0
                },
            ]
        }
    }

    fn print_all(&self) {
        for (i, equip) in self.equips.iter().enumerate() {
            println!("{}. Weapon {} - Quantity {}", i+1, weapon_to_string(&equip.weapon), equip.quantity);
        }
    }

    fn list_all_filter(&self) -> bool {
        let mut flag = false;
        for (i, equip) in self.equips.iter().enumerate() {
            if equip.quantity > 0 {
                println!("{}. Weapon {} - Quantity {}", i+1, weapon_to_string(&equip.weapon), equip.quantity);
                flag = true;
            }
        };
        flag
    }

    fn add_quantity(&mut self) {
        let index: usize;
        match rand::thread_rng().gen_range(1..5) {
            1 => {
                index = self.equips.iter().position(|e| e.weapon == Weapon::Wood).unwrap();
                println!("Get Wood Weapon");
            },
            2 => {
                index = self.equips.iter().position(|e| e.weapon == Weapon::Copper).unwrap();
                println!("Get Copper Weapon")
            },
            3 => {
                index = self.equips.iter().position(|e| e.weapon == Weapon::Iron).unwrap();
                println!("Get Iron Weapon")
            },
            4 => {
                index = self.equips.iter().position(|e| e.weapon == Weapon::Gold).unwrap();
                println!("Get Gold Weapon")
            },
            _ => index = 0
        };

        self.equips.get_mut(index).unwrap().quantity += 1;
    }

    fn reduce_quantity(&mut self, index: usize) {
        if index > 5 {
            clear_screen();
            drop_items(self);
        }

        if self.equips.get(index - 1).unwrap().quantity.eq(&0) {
            clear_screen();
            drop_items(self);
        }

        print!("How much to be dropped? [1 - {:?}]: ", self.equips.get(index - 1).unwrap().quantity);
        flush();

        let input = match get_input() {
            Some(input) => input.parse::<u8>().unwrap_or(255),
            None => return
        };

        if input > self.equips.get(index - 1).unwrap().quantity || input < self.equips.get(index - 1).unwrap().quantity {
            clear_screen();
            println!("Input too big or too small");
            drop_items(self);
        }

        self.equips.get_mut(index - 1).unwrap().quantity -= input;

        clear_screen();

        println!("You now have {} {} weapon", weapon_to_string(&self.equips.get(index - 1).unwrap().weapon), self.equips.get(index - 1).unwrap().quantity);
    }
}

struct Player {
    name: String,
    weapon: Equipment
}

impl Player {
    fn new(name: String, weapon: Equipment) -> Self {
        Self { name, weapon }
    }

    fn change_weapon(&mut self, equipments: &mut Equipments) {
        if !equipments.list_all_filter() {
            println!("No Equipments");
            return
        }
        println!();

        print!("Input which weapon to change into: ");
        flush();

        let index = match get_input() {
            Some(index) => index.parse::<usize>().unwrap_or(0),
            None => return
        };

        match equipments.equips.get(index - 1).unwrap().weapon {
            Weapon::Wood => {
                self.weapon.weapon = Weapon::Wood;
                self.weapon.quantity = 1;
            },
            Weapon::Copper => {
                self.weapon.weapon = Weapon::Copper;
                self.weapon.quantity = 1;
            },
            Weapon::Iron => {
                self.weapon.weapon = Weapon::Iron;
                self.weapon.quantity = 1;
            },
            Weapon::Gold => {
                self.weapon.weapon = Weapon::Gold;
                self.weapon.quantity = 1;
            },
            _ => return
        };

        equipments.equips.get_mut(index - 1).unwrap().quantity -= self.weapon.quantity;
    }

    fn drop_hand(&mut self) {
        if self.weapon.weapon.eq(&Weapon::Hand) {
            println!("You are not equipped with a weapon");
            return
        }

        println!("Dropped {}, Its now long gone", weapon_to_string(&self.weapon.weapon));

        self.weapon.weapon = Weapon::Hand;
        self.weapon.quantity = 1;
    }
}

impl Print for Player {
    fn print_all(&self) {
        println!("Hi {}, you are using {}", self.name, weapon_to_string(&self.weapon.weapon));
    }
}

fn drop_items(equipments: &mut Equipments) {
    if !equipments.list_all_filter() {
        println!("No Equipments");
        return
    }
    println!();

    print!("Input which weapon to be dropped: ");
    flush();

    let index = match get_input() {
        Some(index) => index.parse::<usize>().unwrap_or(255),
        None => return
    };

    equipments.reduce_quantity(index);
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    
    io::stdin().read_line(&mut buffer).unwrap_or(0);

    let input = buffer.trim().to_owned();

    if input.is_empty() {
        return None;
    }
    Some(input)
}

fn flush() {
    let x = io::stdout().flush();
    if x.is_err() {
        println!("Fail to flush: {:?}", x.unwrap());
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    // Clear Screen... ???
    clear_screen();
    print!("Input your name: ");
    flush();

    let input = match get_input() {
        Some(input) => input,
        None => return
    };

    clear_screen();

    let hand = Equipment {
        weapon: Weapon::Hand,
        quantity: 1
    };

    let mut player = Player::new(input, hand);

    let mut equipments = Equipments::new();

    loop {
        player.print_all();
        println!("=== === === === === === === === === === === ===");
        println!("1. View Equipments");
        println!("2. Gacha");
        println!("3. Drop Equipments");
        println!("4. Change Equipments");
        println!("5. Drop Hand Equipped");
        println!("Just enter to exit\n");
        print!("Enter Selection: ");
        flush();

        let input = match get_input() {
            Some(input) => input,
            None => return
        };

        clear_screen();

        match input.as_str() {
            "1" => equipments.print_all(),
            "2" => equipments.add_quantity(),
            "3" => drop_items(&mut equipments),
            "4" => player.change_weapon(&mut equipments), 
            "5" => player.drop_hand(),
            _ => break
        };

        println!();
    }
}
