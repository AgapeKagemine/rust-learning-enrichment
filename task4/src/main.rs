use structopt::StructOpt;

pub mod error;

// Data module file
pub mod foods; // Foods data definition and processing
pub mod reports; // Reports data definition and processing

// StructOpt module file
pub mod opt;

fn main() {
    let arguments = opt::Opt::from_args();
    if let Err(e) = opt::Opt::run(arguments) {
        println!("Error: {}", e);
    }
    
}
