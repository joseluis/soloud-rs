# soloud-rs


[![Documentation](https://docs.rs/soloud/badge.svg)](https://docs.rs/soloud)
[![Crates.io](https://img.shields.io/crates/v/soloud.svg)](https://crates.io/crates/soloud)
[![License](https://img.shields.io/crates/l/soloud.svg)](https://github.com/MoAlyousef/soloud-rs/blob/master/LICENSE)
[![Build](https://github.com/MoAlyousef/soloud-rs/workflows/Build/badge.svg)](https://github.com/MoAlyousef/soloud-rs/actions)

A crossplatform Rust bindings for the soloud audio engine library.

Supported formats: wav, mp3, ogg, flac. The library also comes with a speech synthesizer.

- The official soloud [website](https://sol.gfxile.net/soloud/index.html)
- The official soloud [repo](https://github.com/jarikomppa/soloud)

## Usage
```toml
[dependencies]
soloud = "0.1"
```

Or to use the git repo:
```toml
[dependencies]
soloud = { git = "https://github.com/moalyousef/soloud-rs" }
```

To play audio:
```rust
use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;

    let mut wav = audio::Wav::default();

    wav.load(&std::path::Path::new("sample.wav"))?;

    sl.play(&wav); // calls to play are non-blocking, so we put the thread to sleep
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    wav.load(&std::path::Path::new("Recording.mp3"))?;
    
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
```

To play speech:
```rust
use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;

    let mut speech = audio::Speech::default();

    speech.set_text("Hello World")?;

    sl.play(&speech);
    while sl.active_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    speech.set_text("1 2 3")?;

    sl.play(&speech);
    while sl.active_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    speech.set_text("Can you hear me?")?;

    sl.play(&speech);
    while sl.active_voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    Ok(())
}
```

To add a filter:
```rust
use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;

    let mut wav = audio::Wav::default();
    let mut filt = filter::EchoFilter::default();
    filt.set_params(0.2)?; // sets the delay

    wav.load(&std::path::Path::new("sample.wav"))?;
    wav.set_filter(0, Some(&filt));

    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
```

The examples can be found in the soloud/examples directory. They can be run using:
```
$ cargo run --example simple
$ cargo run --example speech
$ cargo run --example filter
$ cargo run --example load_mem
```
You will need to have valid "sample.wav" and "Recording.mp3" audio files in the project root. Or you can change the paths to point to any supported audio file.

There is also a demo file in the demos directory showing the use of this crate with a gui library like fltk.

## Dependencies
A Rust compiler, C++ compiler, Cargo, CMake and git (all these need to be in your PATH). This crate uses the miniaudio backend which assumes default sound drivers are functional.

