//==============================================================================
//==============================================================================
pub mod chord_checker {

    use crate::transposition_constants as tc;

    //==============================================================================
    pub fn compute_transposition_circle() -> Vec< Vec< &'static str > > {
        let mut result = Vec::new();
        result.push(vec![tc::C_NATURAL, tc::B_SHARP]);
        result.push(vec![tc::C_SHARP, tc::D_FLAT]);
        result.push(vec![tc::D_NATURAL]);
        result.push(vec![tc::E_FLAT, tc::D_SHARP]);
        result.push(vec![tc::E_NATURAL, tc::F_FLAT]);
        result.push(vec![tc::F_NATURAL, tc::E_SHARP]);
        result.push(vec![tc::F_SHARP, tc::G_FLAT]);
        result.push(vec![tc::G_NATURAL]);
        result.push(vec![tc::G_SHARP, tc::A_FLAT]);
        result.push(vec![tc::A_NATURAL]);
        result.push(vec![tc::B_FLAT, tc::A_SHARP]);
        result.push(vec![tc::B_NATURAL, tc::C_FLAT]);
        return result;
    }

    //==============================================================================
    fn is_note(word: &String) -> bool {
        for n in tc::ALL_NOTES {
            if word == n {
                return true;
            }
        }
        return false;
    }

    //==============================================================================
    fn is_chord_base(word: &String) -> bool {
        for n in tc::ALL_NOTES {
            if word.starts_with(n) {
                return true;
            }
        }
        return false;
    }

    //==============================================================================
    pub fn split_chord(word: &String) -> (String,String) {
        for n in tc::ALL_NOTES {
            if word.starts_with(n) {
                let (lhs,rhs) = word.split_at(n.len());
                return (lhs.to_string(), rhs.to_string());
            }
        }
        panic!("Argument is not a valid chord!");
    }

    //==============================================================================
    pub fn get_note_from_chord(word: &String) -> String {
        return split_chord(word).0;
    }

    //==============================================================================
    pub fn trim_note_from_chord(word: &String) -> String {
        return split_chord(word).1;
    }

    //==============================================================================
    fn is_basic_chord_type(word: &String) -> bool {
        for t in tc::ALL_TYPES {
            if word.starts_with(t) {
                return true;
            }
        }
        return false;
    }

    //==============================================================================
    pub fn has_compound_chord(word: &String) -> bool {
        return word.contains(tc::CHORD_INVERSION);
    }

    //==============================================================================
    pub fn split_compound_chord(word: &String) -> (String,String) {
        let split_id = word.find(tc::CHORD_INVERSION);
        match split_id {
            Some(id) => {
                return (
                    word.split_at(id).0.to_string(),
                    word.split_at(id+1).1.to_string()
                )
            },
            None => panic!("Not a compound chord!")
        }        
    }

    //==============================================================================
    pub fn trim_compound_chord(word: &String) -> String {
        return split_compound_chord(word).0;
    }

    //==============================================================================
    pub fn get_compound_chord(word: &String) -> String {
        return split_compound_chord(word).1;
    }

    //==============================================================================
    pub fn is_chord(word: &String) -> bool {
        if !is_chord_base(word) { return false }
        
        let subword = trim_note_from_chord(word);
        if !(subword == "" || is_basic_chord_type(&subword)) { return false }

        if has_compound_chord(&subword) {
            let subword = get_compound_chord(&subword);
            if !is_note(&subword) { return false }
        }

        return true;
    }
}

//==============================================================================
//==============================================================================
pub mod false_positive_chord {
    use crate::transposition_constants as tc;
    use crate::transposition::chord_checker as cc;

    //==============================================================================
    const IMPLACING_SYMBOL: &'static str = ">>>FPC<<<";

    //==============================================================================
    // Returns true if there WILL be a false positive
    pub fn check_false_positive(line: &String) -> bool {
        let mut iter = line.split_whitespace();
        let risky_word = iter.next();
        let next_word = iter.next();

        if let Some(risky_word) = risky_word {
            if let Some(next_word) = next_word {
                return risky_word == tc::A_NATURAL && !cc::is_chord(&next_word.to_string())
            }
        }
        return false;
    }

    //==============================================================================
    pub fn process_implaced_line(line: &mut String) {
        *line = line.replacen(tc::A_NATURAL, IMPLACING_SYMBOL, 1);
    }

    //==============================================================================
    pub fn restore_implaced_line(line: &mut String) {
        *line = line.replacen(IMPLACING_SYMBOL, tc::A_NATURAL, 1);
    }
}

pub mod chord_transposer {
    use crate::transposition_constants as tc;
    use crate::transposition::chord_checker as cc;
    use crate::transposition::false_positive_chord as fpc;

    //==============================================================================
    fn transpose_note(note: &String, value: usize) -> String {
        let transpo_circle = cc::compute_transposition_circle();
        for (i,x) in transpo_circle.iter().enumerate() {
            if x.contains(&note.as_str()) {
                return transpo_circle[(i+value)%transpo_circle.len()][0].to_string();
            }
        }
        panic!("{} is obviously not a note!", note);

    }

    //==============================================================================
    fn transpose_chord(chord: &String, value: usize) -> String {
        let (note, ctype) = cc::split_chord(chord);
        let new_note = transpose_note(&note, value);

        if cc::has_compound_chord(chord) {
            let (ctype, inversion) = cc::split_compound_chord(&ctype);
            let new_inversion = transpose_note(&inversion, value);
            return format!("{}{}{}{}", new_note, ctype, tc::CHORD_INVERSION, new_inversion);
        }
        else {
            return format!("{}{}", new_note, ctype);
        }
    }

    //==============================================================================
    pub fn transpose_line(line: &String, value: usize) -> String {
        let mut output_line = String::new();
        let mut input_line = line.clone();

        if fpc::check_false_positive(&input_line) {
            fpc::process_implaced_line(&mut input_line);
        }

        for word in line.split_whitespace() {
            let word = word.to_string();
            if cc::is_chord(&word) {
                output_line.push_str(transpose_chord(&word, value).as_str());
            }
            else {
                output_line.push_str(word.as_str());
            }
            output_line.push(' ');
        }

        fpc::restore_implaced_line(&mut output_line);

        return output_line;
    }
}
