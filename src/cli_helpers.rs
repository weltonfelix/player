use std::process::Command;

/// Clears the screen depending on the OS (Windows or Unix)
pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    };
}
