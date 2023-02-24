use std::{
    io::{self, Read, Write},
    sync::mpsc::{self, Receiver},
    thread,
};

use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

/**
 * Based on <https://stackoverflow.com/a/37416107>
 *
 * This function will create a new thread that will read user input from stdin
 */
pub fn create_user_input_reader() -> Receiver<char> {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut mut_termios = termios.clone();
    mut_termios.c_lflag &= !(ICANON | ECHO);

    let (tx, rx) = mpsc::channel::<char>();
    thread::spawn(move || loop {
        tcsetattr(stdin, TCSANOW, &mut mut_termios).unwrap();

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

#[cfg(test)]
mod test {
    use std::sync::mpsc::TryRecvError;

    use super::*;

    #[test]
    fn test_create_user_input_reader() {
        let user_input_reader = create_user_input_reader();
        // The user input reader should be empty at the start
        assert_eq!(user_input_reader.try_recv(), Err(TryRecvError::Empty));
    }
}
