

//==============================================================================
//==============================================================================
pub mod io {
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
        // TODO
    }

    //==============================================================================
    fn get_input_command() -> Command {
        // TODO
        return Command::None;
    }

    //==============================================================================
    fn print_file_selection(file_list: &Vec<String>, selection: usize) {
        clear();
        for (index, filename) in file_list.iter().enumerate() {
            let mut left_carret: String = "   ".to_string();
            let mut right_carret: String = "".to_string();
            if index == selection {
                left_carret = "==>".to_string();
                right_carret = "<==".to_string();
            }
            println!("{} {} {}", left_carret, filename, right_carret); 
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

