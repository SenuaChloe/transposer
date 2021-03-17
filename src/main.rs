use transposer::transposition::chord_checker as cc;
use transposer::transposition::false_positive_chord as fpc;
use transposer::transposition::chord_transposer as ct;
use transposer::transposition::file_scanner as fscan;
use transposer::ui::io;

const DEBUG_MODE:bool = false;

fn main() -> Result<(), ()> {
    if DEBUG_MODE { print_tests() }

    let files = fscan::scan_dir();

    loop {
        match io::loop_file_selection(&files) {
            None => break,
            Some(filename) => {

                let lines = fscan::break_file_into_lines(&filename);

                let chord_list = fscan::scan_file_for_chords(&lines);

                if let Some(transpo_value) = io::loop_transpo_selection(&chord_list) {
                    
                    let result = fscan::perform_transposition(&lines, transpo_value);

                    let success = fscan::write_file(&filename, &result);

                    if success {
                        println!("Operation succeeded !");
                        return Ok(());
                    }
                    else {
                        println!("Operation failed !");
                        return Err(());
                    }
                }
            }
        }
    }
    println!("Exiting program");
    return Ok(());

}

fn print_tests() {
    println!("A {}", cc::is_chord(&"A".to_string()));
    println!("Bm {}", cc::is_chord(&"Bm".to_string()));
    println!("C#M {}", cc::is_chord(&"C#M".to_string()));
    println!("D7 {}", cc::is_chord(&"D7".to_string()));
    println!("E#m11 {}", cc::is_chord(&"E#m11".to_string()));
    println!("Fsus4 {}", cc::is_chord(&"Fsus4".to_string()));
    println!("Gbadd8 {}", cc::is_chord(&"Gbadd8".to_string()));
    println!("A/F {}", cc::is_chord(&"A/F".to_string()));
    println!("Bbsus3/G {}", cc::is_chord(&"Bbsus3/G".to_string()));
    println!("T {}", cc::is_chord(&"T".to_string()));
    println!("Dpro {}", cc::is_chord(&"Dpro".to_string()));
    println!("Esus/H {}", cc::is_chord(&"Esus/H".to_string()));
    println!("Fdim/Gm {}", cc::is_chord(&"Fdim/Gm".to_string()));
    println!("Gadd/Fb {}", cc::is_chord(&"Gadd/Fb".to_string()));
    println!("Hadd {}", cc::is_chord(&"Hadd".to_string()));
    println!("Isus/Fb {}", cc::is_chord(&"GIsus/Fb".to_string()));
    println!("G# {}", cc::is_chord(&"G#".to_string()));
    println!("Gadd/F7 {}", cc::is_chord(&"Gadd/F7".to_string()));
    println!("Gsus_yo_mama_is_flat {}", cc::is_chord(&"Gsus_yo_mama_is_flat".to_string()));
    println!("");
    let mut test: String = "Une petite A maison.".to_string();
    println!("Une petite A maison. {}", fpc::check_false_positive(&test));
    fpc::process_implaced_line(&mut test);
    println!("Une petite A maison. {}", test);
    fpc::restore_implaced_line(&mut test);
    println!("Une petite A maison. {}", test);
    let mut test: String = "A little house.".to_string();
    println!("A little house. {}", fpc::check_false_positive(&test));
    fpc::process_implaced_line(&mut test);
    println!("A little house. {}", test);
    fpc::restore_implaced_line(&mut test);
    println!("A little house. {}", test);
    println!("");
    println!("Yay this is a B#m/Fb !!!! Kudos ! {}", ct::transpose_line(&"Yay this is a B#m/Fb !!!! Kudos !".to_string(), 3));
    println!("GM7 this A is a B#m/Fb !!!! Kudos ! {}", ct::transpose_line(&"GM7 this A is a B#m/Fb !!!! Kudos !".to_string(), 3));
    println!("A this is a B#m/Fb !!!! Kudos ! {}", ct::transpose_line(&"A this is a B#m/Fb !!!! Kudos !".to_string(), 12));
}
