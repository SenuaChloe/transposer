

//==============================================================================
//==============================================================================
pub mod io {
    use crossterm::cursor;
    use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
    use std::io::{stdout, Write};

    use crate::transposition::chord_checker as cc;
    
    //==============================================================================
    enum Command {
        Next,
        Previous,
        Cancel,
        Proceed,
        None
    }

    //==============================================================================
    fn clear() {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();        
    }

    //==============================================================================
    fn get_input_command() -> Command {
        println!("");
        println!("[Use arrows to navigate, Enter/Space to proceed, Escape/Backspace to cancel.]");
        match read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::NONE }) | 
            Event::Key(KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE }) 
                => return Command::Previous,
            Event::Key(KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE }) | 
            Event::Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE }) 
                => return Command::Next,
            Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE }) | 
            Event::Key(KeyEvent { code: KeyCode::Backspace, modifiers: KeyModifiers::NONE }) 
                => return Command::Cancel,
            Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) | 
            Event::Key(KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE }) 
                => return Command::Proceed,
            _ => return Command::None,
        }
    }

    //==============================================================================
    fn print_file_selection(file_list: &Vec<String>, selection: usize) {
        clear();
        println!("Please choose the file you want to transpose :");
        for (index, filename) in file_list.iter().enumerate() {
            let mut left_carret: String = "   ".to_string();
            let mut right_carret: String = "".to_string();
            if index == selection {
                left_carret = "==>".to_string();
                right_carret = "<==".to_string();
            }
            println!("  {} {} {}", left_carret, filename, right_carret); 
        }
    }

    //==============================================================================
    pub fn loop_file_selection() -> Option<String> {
        // TODO
        return None;
    }

    //==============================================================================
    fn print_transpo_selection(chords: &Vec<String>, transpo_value: u32) {
        // TODO
    }

    //==============================================================================
    pub fn loop_transpo_selection(chord_list: &Vec<String>) -> Option<u32> {
        // TODO
        return None;
    }

}

