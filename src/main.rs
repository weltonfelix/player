use colored::Colorize;
use std::sync::mpsc::Receiver;
use std::{env, process, thread, time::Duration};
use vlc::MediaPlayerAudioEx;

extern crate vlc;

mod cli_helpers;
mod keymaps;
mod time_parser;
mod user_input;

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
    let file = &args[1];

    let user_input_reader = user_input::create_user_input_reader();

    let vlc_instance = vlc::Instance::new().unwrap();
    let media = vlc::Media::new_path(&vlc_instance, file).unwrap();
    let media_player = vlc::MediaPlayer::new(&vlc_instance).unwrap();
    media_player.set_media(&media);

    media_player.play().unwrap();
    loop {
        play_loop(&media_player, &media, file, &user_input_reader)
    }
}

fn play_loop(
    media_player: &vlc::MediaPlayer,
    media: &vlc::Media,
    file: &str,
    user_input_reader: &Receiver<char>,
) {
    cli_helpers::clear_screen();
    if media_player.state() == vlc::State::Ended {
        println!("{}", "Finished playing!".green());
        process::exit(0);
    }

    let duration = media.duration();
    let curr_time = media_player.get_time();

    print_play_ui(&media_player, file, duration, curr_time);
    handle_keymaps(&media_player, &user_input_reader, duration, curr_time);

    thread::sleep(Duration::from_millis(100));
}

fn print_play_ui(
    media_player: &vlc::MediaPlayer,
    file: &str,
    duration: Option<i64>,
    curr_time: Option<i64>,
) {
    println!("{:?} {}", media_player.state(), file.cyan().italic());

    if duration.is_some() && curr_time.is_some() {
        println!(
            "{}",
            format!(
                "{} / {}\n",
                time_parser::parse_time(curr_time.unwrap()),
                time_parser::parse_time(duration.unwrap())
            )
            .on_blue()
            .bold()
        );
    }

    println!(
        "Volume: {}",
        format!("{}%", media_player.get_volume()).purple().bold(),
    );
    println!(
        "{}",
        format!(
            "(q)quit (space | p){} (n)backwards (m)forwards ([)volume up (])volume down",
            if media_player.is_playing() {
                "pause"
            } else {
                "play"
            }
        )
        .dimmed()
    );
}

fn handle_keymaps(
    media_player: &vlc::MediaPlayer,
    user_input_reader: &Receiver<char>,
    duration: Option<i64>,
    curr_time: Option<i64>,
) {
    keymaps::read_keymaps(
        &user_input_reader,
        &|| {
            println!("Stopping...");
            process::exit(0);
        },
        &|| {
            media_player.pause();
        },
        &|| {
            if curr_time.is_some() {
                if curr_time.unwrap() - 5000 < 0 {
                    media_player.set_time(0);
                } else {
                    media_player.set_time(curr_time.unwrap() - 5000);
                }
            }
        },
        &|| {
            if curr_time.is_some() {
                if curr_time.unwrap() + 5000 > duration.unwrap() {
                    media_player.set_time(duration.unwrap());
                } else {
                    media_player.set_time(curr_time.unwrap() + 5000);
                }
            }
        },
        &|| {
            if media_player.get_volume() + 5 > 100 {
                match media_player.set_volume(100) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            } else {
                match media_player.set_volume(media_player.get_volume() + 5) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        },
        &|| {
            if media_player.get_volume() - 5 < 0 {
                match media_player.set_volume(0) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            } else {
                match media_player.set_volume(media_player.get_volume() - 5) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        },
    );
}
