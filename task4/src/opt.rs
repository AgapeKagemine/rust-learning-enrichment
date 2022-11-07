use structopt::StructOpt;

use crate::{reports, foods};

#[derive(Debug, StructOpt)]
enum Command {
    List,
    Report,
    Buy {
        name: String,
        stocks: u64
    },
    Add {
        name: String,
        stocks: u64,
        price: u128
    },
    Delete {
        name: String
    }
}

#[derive(StructOpt)]
#[structopt(about = "Dev Restaurant")]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool
}

impl Opt {
    pub fn run(opt: Opt) -> Result<(), std::io::Error> {
        match opt.cmd {
            Command::List => {
                let foods = foods::load_foods(opt.verbose)?;
                for food in foods.into_vec() {
                    println!("{:?}", food)
                }
                Ok(())
            }
            Command::Report => {
                let reports = reports::load_reports(opt.verbose)?;
                for report in reports.into_vec() {
                    println!("{:?}", report)
                }
                Ok(())
            }
            Command::Add { name, stocks, price } => {
                let mut foods = foods::load_foods(opt.verbose)?;

                match foods::parse_food(&format!("0,{name},{stocks},{price}")) {
                    Ok(food) => {
                        foods.insert(food);
                        foods::save_foods(foods)?;
                    },
                    Err(e) => println!("{:?}", e)
                }

                Ok(())
            }
            Command::Buy { name, stocks } => {
                let mut reports = reports::load_reports(opt.verbose)?;
                let foods = foods::load_foods(opt.verbose)?;

                let income = match foods::parse_food(&format!("0,{name},{stocks},0")) {
                    Ok(food) => foods.sell(food),
                    Err(_e) => 0
                };

                let date = chrono::offset::Local::now().date().format("%Y-%m-%d").to_string();
                println!("{:?}", date);
                
                match reports::parse_report(&format!("{},{date},{stocks},{income}", reports.next_id())) {
                    Ok(report) => {
                        reports.add(report);
                        reports::save_reports(reports)?;
                    },
                    Err(e) => println!("{:?}", e)
                }

                Ok(())
            }
            Command::Delete { name } => {
                let mut foods = foods::load_foods(opt.verbose)?;

                if foods.remove(&name) { // True == Success
                    foods::save_foods(foods)?;
                    println!("Berhasil menghapus {name} dari daftar");
                }else {
                    println!("Tidak ada makanan dengan nama {name} di data");
                };

                Ok(())
            }
        }
    }
}