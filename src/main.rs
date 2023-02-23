use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{env, process, thread, time::Duration};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};
use vlc::MediaPlayerAudioEx;

extern crate vlc;

fn main() {
    // Ensure VLC is installed
    if vlc::Instance::new().is_none() {
        println!("VLC is not installed!");
        return;
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone(); // make a mutable copy of termios
                                           // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    let stdin_channel = create_stdin_channel(stdin, termios, new_termios);

    let file = &args[1];

    let vlc_instance = vlc::Instance::new().unwrap();

    let media = vlc::Media::new_path(&vlc_instance, file).unwrap();

    let media_player = vlc::MediaPlayer::new(&vlc_instance).unwrap();
    media_player.set_media(&media);

    media_player.play().unwrap();
    loop {
        // Clear
        print!("\x1B[2J\x1B[1;1H");
        if media_player.state() == vlc::State::Ended {
            println!("Finished playing!");
            process::exit(0);
        }

        println!("{:?} {}", media_player.state(), file);
        println!("Volume: {}%", media_player.get_volume());
        println!(
            "(q)quit (space | p){} (n)backwards (m)forwards ([)volume up (])volume down",
            if media_player.is_playing() {
                "pause"
            } else {
                "play"
            }
        );

        let duration = media.duration();
        let curr_time = media_player.get_time();

        if duration.is_some() && curr_time.is_some() {
            println!(
                "{}/{}",
                parse_time(curr_time.unwrap()),
                parse_time(duration.unwrap())
            );
        }

        match stdin_channel.try_recv() {
            Ok(key) => match key {
                'q' => {
                    println!("Stopping...");
                    process::exit(0);
                }
                'p' => {
                    media_player.pause();
                }
                'n' => {
                    if curr_time.is_some() {
                        if curr_time.unwrap() - 5000 < 0 {
                            media_player.set_time(0);
                        } else {
                            media_player.set_time(curr_time.unwrap() - 5000);
                        }
                    }
                }
                'm' => {
                    if curr_time.is_some() {
                        if curr_time.unwrap() + 5000 > duration.unwrap() {
                            media_player.set_time(duration.unwrap());
                        } else {
                            media_player.set_time(curr_time.unwrap() + 5000);
                        }
                    }
                }
                '[' => {
                    if media_player.get_volume() + 5 > 100 {
                        match media_player.set_volume(100) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                    } else {
                        match media_player.set_volume(media_player.get_volume() + 5) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                    }
                }
                ']' => {
                    if media_player.get_volume() - 5 < 0 {
                        match media_player.set_volume(0) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                    } else {
                        match media_player.set_volume(media_player.get_volume() - 5) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                    }
                }
                ' ' => {
                    media_player.pause();
                }
                _ => {}
            },
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn create_stdin_channel(stdin: i32, termios: Termios, mut new_termios: Termios) -> Receiver<char> {
    let (tx, rx) = mpsc::channel::<char>();
    thread::spawn(move || loop {
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0; 1]; // read exactly one byte
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        tx.send(buffer[0] as char).unwrap();
        tcsetattr(stdin, TCSANOW, &termios).unwrap();
    });
    return rx;
}

fn parse_time(millis: i64) -> String {
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    let formatted_seconds = seconds % 60;
    let formatted_minutes = minutes % 60;
    let formatted_hours = hours % 24;
    let formatted_days = days % 24;

    let mut time = String::new();

    if days > 0 {
        let space = if hours > 0 { " " } else { "" };
        time.push_str(&format!("{}d{}", formatted_days, space));
    }

    if hours > 0 {
        let space = if hours > 0 { " " } else { "" };
        time.push_str(&format!("{}h{}", formatted_hours, space));
    }

    if minutes > 0 {
        let space = if seconds > 0 { " " } else { "" };
        time.push_str(&format!("{}m{}", formatted_minutes, space));
    }

    time.push_str(&format!("{}s", formatted_seconds));

    return time;
}
