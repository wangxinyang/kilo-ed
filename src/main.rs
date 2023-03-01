use crossterm::{
    event::{read, Event::Key, KeyCode},
    Result,
};

fn main() -> Result<()> {
    // loop {
    while let Ok(event) = read() {
        if let Key(key_event) = event {
            if key_event.code == KeyCode::Char('q') {
                break;
            } else {
                println!("{event:?}");
            }
        }
    }
    // else {
    //     break;
    // }
    // }
    Ok(())
}
