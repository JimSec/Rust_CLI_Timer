use std::fs;
use std::io::BufReader;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Instant,Duration};
use std::thread;
use rodio::{Decoder, OutputStream, source::Source};
use rdev::{listen, Event};
use serde::{Deserialize, Serialize};

static PAUSE_STATE: AtomicBool = AtomicBool::new(false);

pub fn get_pause() -> bool {
    PAUSE_STATE.load(Ordering::Relaxed)
}
pub fn set_pause(level: bool) {
    PAUSE_STATE.store(level, Ordering::Relaxed);
}


#[derive(Serialize, Deserialize)]
pub struct Round {
    round_title: String,
    length: f32,
    rest: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Workout {
    pub workout_title: String,
    pub alarm_path: String,
    pub regimen: Vec<Round>,
}


impl Workout {

    pub fn start(&self) {
        let mut default_playsound = true;
        // Create Keyboard event watcher thread
        let _handle = thread::spawn(move || {
            if let Err(error) = listen( callback) {
                println!("Error: {:?}", error)
            }

            fn callback(event: Event, ) {
                match event.event_type  {
                    // Space to pause
                    rdev::EventType::KeyPress(rdev::Key::Space) => set_pause(!get_pause()),
                    _ => (),
                }
            }
        });

        let mut i = 1; //lazy round counter 
        for r in &self.regimen {
            //let the outer block play the final bell, so I can use a second bell audio file later.
            if i == self.regimen.len() {
                default_playsound = false;
            }
            
            self.countdown(&r.round_title, &r.length, true, &self.alarm_path, default_playsound);
            // don't run a zero sized rest, and don't run the final rest
            if r.rest != 0.0 && i != self.regimen.len() { 
                self.countdown(&r.round_title, &r.rest, false, &self.alarm_path, default_playsound);
            }
            
            i += 1;

        }
        println!("Finished Workout: {}!",&self.workout_title);
        self.play_sound(self.alarm_path.clone(), true);
        exit(0);
    }

    fn countdown(&self, r: &String, t: &f32, work: bool,  alarm_path: &String, playsound: bool) {
        let mut curr_t = Instant::now();
        let mut work_t = *t;
        let mut message_prefix = "";
        if !work {
            message_prefix = "Rest ";
        }

        // Sacrifice a little precision to never display a negative timer number
        while (curr_t.elapsed().as_secs_f32() < work_t) && (work_t - curr_t.elapsed().as_secs_f32() > 0.1) {
            
            //if we're paused, save our state and wait to unpause.
            thread::sleep(Duration::from_millis(100));
            if get_pause() == true {
                let saved_t = work_t - curr_t.elapsed().as_secs_f32();
                while get_pause() == true {
                    println!("Paused {}{}:\n{}\n\nSpace to Unpause.", message_prefix, r,  saved_t);
                    print!("{esc}c", esc = 27 as char); //force clear terminal 
                    thread::sleep(Duration::from_millis(100));
                }
                //Set main loop comparision vars before continuing.
                work_t = saved_t;
                curr_t = Instant::now();
            }
            
            println!("{}{}:\n{}\n\nSpace to Pause.", message_prefix, r,  (work_t - curr_t.elapsed().as_secs_f32()));
            print!("{esc}c", esc = 27 as char); //force clear terminal 

        }
        if playsound == true {
            self.play_sound(alarm_path.clone(), false);
        }
    }

    fn play_sound(&self, path: String, blocking: bool) {

        let handle = thread::spawn( || {
            // Blantently stolen from rodio docs
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(fs::File::open(path.clone()).unwrap());
            let source = Decoder::new(file).unwrap();
            
            stream_handle.play_raw(source.convert_samples()).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(5)); // Should really derive the length of the supplied audio...
            drop(path);
        });

        // Don't cut off final bell exiting
        if blocking {
            handle.join().unwrap();
        }
        
    }

    pub fn import_json_workout(json_path: String) -> Workout {
        let data = fs::read_to_string(json_path).expect("Unable to read file");
        let my_wkt: Workout = serde_json::from_str(&data).unwrap();
        my_wkt
    }

    fn _export_json_workout() {
        todo!()
    }
}
