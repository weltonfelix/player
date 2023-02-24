use std::sync::mpsc::{Receiver, TryRecvError};

pub fn read_keymaps(
    user_input_reader: &Receiver<char>,
    on_stop: &dyn Fn(),
    on_pause: &dyn Fn(),
    on_go_backwards: &dyn Fn(),
    on_go_forwards: &dyn Fn(),
    on_volume_up: &dyn Fn(),
    on_volume_down: &dyn Fn(),
) {
    match user_input_reader.try_recv() {
        Ok(key) => match key {
            'q' => on_stop(),
            'p' => on_pause(),
            ' ' => on_pause(),
            'n' => on_go_backwards(),
            'm' => on_go_forwards(),
            '[' => on_volume_up(),
            ']' => on_volume_down(),
            _ => {}
        },
        Err(TryRecvError::Empty) => {}
        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }
}

/*
 match user_input_reader.try_recv() {
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

*/
