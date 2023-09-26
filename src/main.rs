mod workout ;
use crate::workout::Workout;
use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to workout JSON
    #[arg(short, long)]
    json_path: String,
}

fn main() {
    let args = Args::parse();

    // import workout here
    
    let mut my_workout = Workout::import_json_workout(args.json_path);

    println!("Test if this imports properly: {:?}, {:?}", my_workout.workout_title, my_workout.alarm_path);
     
    my_workout.start();
    
    
    //handle.join().unwrap();
}