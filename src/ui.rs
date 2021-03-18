

//==============================================================================
//==============================================================================
pub mod io {
    use crossterm::cursor;
    use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
    use std::io::{stdout, Write};

    use crate::transposition::chord_transposer as ct;
    
    //==============================================================================
    enum Command {
        Next,
        Previous,
        Cancel,
        Proceed,
    }

    //==============================================================================
    fn clear() {
        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();        
    }

    //==============================================================================
    fn get_input_command() -> Command {
        let cmd: Command;
        println!();
        println!();
        print!("[Use arrows to navigate, Enter/Space to proceed, Escape/Backspace to cancel.] ");
        stdout().flush().unwrap();
        enable_raw_mode().unwrap();
        loop {
            match read().unwrap() {
                Event::Key(KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::NONE }) | 
                Event::Key(KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE }) 
                    => { cmd = Command::Previous; break },
                Event::Key(KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE }) | 
                Event::Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE })
                    => { cmd = Command::Next; break },
                Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE }) | 
                Event::Key(KeyEvent { code: KeyCode::Backspace, modifiers: KeyModifiers::NONE }) 
                    => { cmd = Command::Cancel; break },
                Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) | 
                Event::Key(KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE }) 
                    => { cmd = Command::Proceed; break },
                _ => ()
            }
        }
        disable_raw_mode().unwrap();
        println!();
        println!();
        return cmd;
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
                    if file_selection_index == file_list.len()-1 { file_selection_index = 0 }
                    else { file_selection_index += 1 }
                },
                Command::Previous => {
                    if file_selection_index == 0 { file_selection_index = file_list.len()-1 }
                    else { file_selection_index -= 1 }
                },
            }
        }
    }

    //==============================================================================
    fn print_transpo_selection(chords: &Vec<String>, transpo_value: u32) {
        clear();
        println!("Please choose a transposition :");
        println!();
        print!("   [");
        for index in 1..12 {
            if transpo_value == index {
                print!("0");
            }
            else {
                print!("-");
            }
        }
        println!("]");
        println!("   [ Capo {}{}   ]", transpo_value, if transpo_value >= 10 { "" } else { " " } );
        println!();
        print!("     Original: ");
        for chord in chords {
            print!("{} ", chord);
        }
        println!();
        print!("   Transposed: ");
        for chord in chords {
            print!("{} ", ct::transpose_chord(chord, transpo_value));
        }
        println!();
    }

    //==============================================================================
    pub fn loop_transpo_selection(chord_list: &Vec<String>) -> Option<u32> {
        let mut transpo_value = 1;
        loop {
            print_transpo_selection(chord_list, transpo_value);

            match get_input_command() {
                Command::Cancel => return None,
                Command::Proceed => return Some(transpo_value),
                Command::Next => {
                    if transpo_value == 11 { transpo_value = 1 }
                    else { transpo_value += 1 }
                },
                Command::Previous => {
                    if transpo_value == 1 { transpo_value = 11 }
                    else { transpo_value -= 1 }
                },
            }
        }
    }

}

