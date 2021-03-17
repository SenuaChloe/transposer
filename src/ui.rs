

//==============================================================================
//==============================================================================
pub mod io {
    use crossterm::cursor;
    use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal::{Clear, ClearType};
    use std::io::stdout;

    use crate::transposition::chord_transposer as ct;
    
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
    pub fn loop_file_selection(file_list: &Vec<String>) -> Option<String> {
        let mut file_selection_index = 0;
        loop {
            print_file_selection(file_list, file_selection_index);

            match get_input_command() {
                Command::Cancel => return None,
                Command::Proceed => return Some(file_list[file_selection_index].clone()),
                Command::Next => {
                    if file_selection_index == file_list.len() { file_selection_index = 0 }
                    else { file_selection_index += 1 }
                },
                Command::Previous => {
                    if file_selection_index == 0 { file_selection_index = file_list.len() }
                    else { file_selection_index -= 1 }
                },
                Command::None => {}
            }
        }
    }

    //==============================================================================
    fn print_transpo_selection(chords: &Vec<String>, transpo_value: u32) {
        println!("Please choose a transposition :");
        println!("");
        print!("   [");
        for index in 0..11 {
            if transpo_value == index {
                print!("0");
            }
            else {
                print!("-");
            }
        }
        println!("]");
        println!("   [ Capo {}{}    ]", transpo_value, if transpo_value >= 10 { " " } else { "" } );
        println!("");
        print!("     Original: ");
        for chord in chords {
            print!("{} ", chord);
        }
        print!("   Transposed: ");
        for chord in chords {
            print!("{} ", ct::transpose_note(chord, transpo_value));
        }
    }

    //==============================================================================
    pub fn loop_transpo_selection(chord_list: &Vec<String>) -> Option<u32> {
        let mut transpo_value = 0;
        loop {
            print_transpo_selection(chord_list, transpo_value);

            match get_input_command() {
                Command::Cancel => return None,
                Command::Proceed => return Some(transpo_value),
                Command::Next => {
                    if transpo_value == 11 { transpo_value = 0 }
                    else { transpo_value += 1 }
                },
                Command::Previous => {
                    if transpo_value == 0 { transpo_value = 11 }
                    else { transpo_value -= 1 }
                },
                Command::None => {}
            }
        }
    }

}

