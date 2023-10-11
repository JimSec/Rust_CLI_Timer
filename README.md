## Rust CLI variable length countdown timer.

I like to do weird non-standard length workouts, and one of the biggest challenges has been getting a timer that can accommodate something beyond ((3min work, 1min rest)x however many rounds). As an excuse to actually build something in Rust (and because I usually have my laptop with me in my garage...) I made a CLI countdown timer that feeds in custom json workouts for fun.

Neither Sounds nor Workouts have to live in the respective folders.

I've only ran on ubuntu, but it required `libx11-dev` and `librust-alsa-sys-dev` system packages.

#### Usage:

```
cargo build --release;
target/release/timers --json-path workout_json/test.json
```

or just
```
cargo run --release -- --json-path workout_json/test.json
``` 
but the double `--` weirds me out so I don't do that. 
