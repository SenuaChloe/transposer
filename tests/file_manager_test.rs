#[cfg(test)]
mod  tests {

    use std::fs::{self, File};
    use std::io::{self, BufRead};
    use transposer::file_manager::fman;
    
    //==============================================================================
    fn compare_files(actual_filepath: &String, reference_filepath: &String) {

        let actual_file = File::open(actual_filepath);
        let reference_file = File::open(reference_filepath);
        if let Err(_) = actual_file {
            panic!("Unable to read file {}", &actual_filepath);
        }
        if let Err(_) = reference_file {
            panic!("Unable to read file {}", &reference_filepath);
        }

        let actual_lines = io::BufReader::new(actual_file.unwrap()).lines();
        let reference_lines = io::BufReader::new(reference_file.unwrap()).lines();
        
        let mut actual_result: Vec<String> = Vec::new();
        let mut reference_result: Vec<String> = Vec::new();
        
        for line in actual_lines {
            actual_result.push(line.unwrap());
        }
        for line in reference_lines {
            reference_result.push(line.unwrap());
        }

        assert_eq!(actual_result, reference_result);
    }
    
    //==============================================================================
    fn make_base() -> Vec<String> {
        let mut base: Vec<String> = Vec::new();
        base.push("A Bm7/G   Cbadd6/Gb".to_string());
        base.push("A house in the sun".to_string());
        base.push("A mixed F# line Cbadd6/Gb".to_string());
        return base;
    }
    
    fn make_transpo() -> Vec<String> {
        let mut transpo: Vec<String> = Vec::new();
        transpo.push("D Em7/C   Eadd6/B  ".to_string());
        transpo.push("A house in the sun".to_string());
        transpo.push("A mixed B  line Eadd6/B  ".to_string());
        return transpo;
    }
        
    //==============================================================================
    const INPUT_SUBDIR: &str = "tests/input/";
    const OUTPUT_SUBDIR: &str = "tests/output/";

    //==============================================================================
    //==============================================================================
    #[test]
    fn test_scan_dir() {
        let mut reference_scan = Vec::new();
        reference_scan.push("test".to_string());
        reference_scan.push("test.txt".to_string());
        let actual_scan = fman::scan_dir(&INPUT_SUBDIR.to_string());
        assert_eq!(actual_scan, reference_scan);
    }

    //==============================================================================
    #[test]
    fn test_break_file_into_line() {
        let reference_break = make_base();
        
        let actual_break = fman::break_file_into_lines(&INPUT_SUBDIR.to_string(), &"test".to_string());
        
        assert_eq!(actual_break, reference_break);
    }
        
    //==============================================================================
    #[test]
    fn test_scan_file_for_chords() {
        let mut reference_scanned = Vec::new();
        reference_scanned.push("Cbadd6/Gb".to_string());
        reference_scanned.push("F#".to_string());
        reference_scanned.push("Bm7/G".to_string());
        reference_scanned.push("A".to_string());
        
        let base = make_base();
        let actual_scanned = fman::scan_file_for_chords(&base);

        assert_eq!(actual_scanned, reference_scanned);
    }
        
    //==============================================================================
    #[test]
    fn test_perform_transposition() {
        let reference_transpo = make_transpo();

        let base = make_base();
        let actual_transpo = fman::perform_transposition(&base,5);

        assert_eq!(actual_transpo, reference_transpo);
    }

    //==============================================================================
    #[test]
    fn test_write_file() {
        fs::remove_dir_all(OUTPUT_SUBDIR).unwrap();
        fs::create_dir(OUTPUT_SUBDIR).unwrap();

        let result = make_transpo();

        let res = fman::write_file(&result, &OUTPUT_SUBDIR.to_string(), &"test".to_string(), 5);
        assert!(res);
        compare_files(&"tests/output/test - Capo 5".to_string(), &"tests/reference/ref".to_string());

        let res = fman::write_file(&result, &OUTPUT_SUBDIR.to_string(), &"test.txt".to_string(), 5);
        assert!(res);
        compare_files(&"tests/output/test - Capo 5.txt".to_string(), &"tests/reference/ref".to_string());
    }
}
