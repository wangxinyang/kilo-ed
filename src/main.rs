use crossterm::{
    event::{poll, read, Event::Key, KeyCode},
    terminal, Result,
};
use errno::errno;
use std::time::Duration;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        let mut c = None;
        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Ok(event) = read() {
                    if let Key(key_event) = event {
                        c = Some(key_event);
                    }
                } else {
                    die("read failed");
                    break;
                }
            }
            Ok(false) => {}
            _ => {
                die("poll failed");
                break;
            }
        }

        if let Some(c) = c {
            if c.code == KeyCode::Char('q') {
                break;
            } else {
                println!("{c:?}\r");
            }
        } else {
            println!("no key\r");
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn die<S: Into<String>>(message: S) {
    let _ = terminal::disable_raw_mode();
    eprintln!("{} : {}", message.into(), errno());
    std::process::exit(1);
}
