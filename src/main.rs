mod workout ;
use crate::workout::Workout;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to workout JSON
    #[arg(short, long)]
    json_path: String,
}

fn main() {
    let args = Args::parse();
    
    let my_workout = Workout::import_json_workout(args.json_path);  
    
    my_workout.start();
    
}