
//==============================================================================
//==============================================================================
pub mod fman {
    use std::collections::HashMap;
    use crate::transposition::chord_checker as cc;
    use crate::transposition::chord_transposer as ct;
    use crate::transposition::false_positive_chord as fpc;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    //==============================================================================
    #[derive(Ord,Eq,PartialOrd,PartialEq)]
    struct ChordOrd {
        quantity: u64,
        chord: String
    }

    //==============================================================================
    pub fn scan_dir(dirname: &String) -> Vec<String> {
        let mut file_list: Vec<String> = Vec::new();
        let file_paths = fs::read_dir(dirname).unwrap();
        for path in file_paths {
            file_list.push(path.unwrap().path().display().to_string().strip_prefix(dirname).unwrap().to_string());
        }
        file_list.sort();
        return file_list;
    }

    //==============================================================================
    pub fn break_file_into_lines(dirname: &String, filename: &String) -> Vec<String> {
        let mut filepath = dirname.clone();
        filepath += filename;
        let mut result: Vec<String> = Vec::new();

        let file = File::open(&filepath);
        if let Err(_) = file {
            panic!("Unable to read file {}", &filepath);
        }
        let lines = io::BufReader::new(file.unwrap()).lines();
        
        for line in lines {
            result.push(line.unwrap());
        }

        return result;
    }

    //==============================================================================
    pub fn scan_file_for_chords(lines: &Vec<String>) -> Vec<String> {
        let mut hash_map: HashMap<String,u64> = HashMap::new();
        let mut temp_vec: Vec<ChordOrd> = Vec::new();
        let mut result_vec: Vec<String> = Vec::new();

        for input_line in lines {
            let mut line = input_line.clone();
            if fpc::check_false_positive(&line) {
                fpc::process_implaced_line(&mut line);
            }
            for word in line.split_whitespace() {
                let word = word.to_string();
                if cc::is_chord(&word) {
                    let mut quantity = 1;
                    if let Some(q) = hash_map.get(&word) { quantity = q+1; }
                    hash_map.insert( word, quantity );
                }
            }
        }

        for (chord,quantity) in hash_map {
            temp_vec.push( ChordOrd { chord:chord, quantity:quantity } );
        }

        temp_vec.sort();
        temp_vec.reverse();

        for item in temp_vec {
            result_vec.push(item.chord);
        }

        return result_vec;
    }

    //==============================================================================
    pub fn perform_transposition(lines: &Vec<String>, transposition_value: u32) -> Vec<String> {
       let mut result = Vec::new();
       for line in lines {
           result.push(ct::transpose_line(line, transposition_value));
       }
       return result;
    }

    //==============================================================================
    pub fn write_file(lines: &Vec<String>, dirname: &String, filename: &String, transpo_value: u32) -> bool {
        let new_filename: String;
        let sfilename: Vec<&str> = filename.splitn(2, '.').collect();
        if sfilename.len() == 2 {
            new_filename = sfilename[0].to_string() + " - Capo " + &transpo_value.to_string() + "." + sfilename[1];
        }
        else {
            new_filename = filename.to_string() + " - Capo " + &transpo_value.to_string();
        }
        let mut filepath = dirname.clone();
        filepath += &new_filename.to_string();
        let output = File::create(filepath);
      
        match output {
            Ok(mut fout) => {
                for line in lines {
                    let wres = writeln!(fout, "{}", line.as_str());
                    if let Err(_) = wres { return false }
                }
                return true;
            },
            Err(_) => return false
        }
    }
}
