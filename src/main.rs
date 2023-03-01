use crossterm::{
    event::{read, Event::Key, KeyCode},
    terminal, Result,
};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    // loop {
    while let Ok(event) = read() {
        if let Key(key_event) = event {
            if key_event.code == KeyCode::Char('q') {
                break;
            } else {
                println!("{event:?}\r");
            }
        }
    }
    // else {
    //     break;
    // }
    // }
    terminal::disable_raw_mode()?;
    Ok(())
}
