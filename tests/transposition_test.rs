#[cfg(test)]
mod  tests {
    
    use transposer::transposition as t;

    //==============================================================================
    
    // Just a remainder that the circle is correct
    // Notr eally useful but hey, what are you gonna do ?
    #[test]
    fn test_transposition_circle() {
        let tcirc = t::chord_checker::compute_transposition_circle();
        assert_eq!( tcirc.len(), 12 );
        assert_eq!( tcirc[0].len(), 2 );
        assert_eq!( tcirc[0][0], "C" );
        assert_eq!( tcirc[0][1], "B#" );
        assert_eq!( tcirc[1].len(), 2 );
        assert_eq!( tcirc[1][0], "C#" );
        assert_eq!( tcirc[1][1], "Db" );
        assert_eq!( tcirc[2].len(), 1 );
        assert_eq!( tcirc[2][0], "D" );
        assert_eq!( tcirc[3].len(), 2 );
        assert_eq!( tcirc[3][0], "Eb" );
        assert_eq!( tcirc[3][1], "D#" );
        assert_eq!( tcirc[4].len(), 2 );
        assert_eq!( tcirc[4][0], "E" );
        assert_eq!( tcirc[4][1], "Fb" );
        assert_eq!( tcirc[5].len(), 2 );
        assert_eq!( tcirc[5][0], "F" );
        assert_eq!( tcirc[5][1], "E#" );
        assert_eq!( tcirc[6].len(), 2 );
        assert_eq!( tcirc[6][0], "F#" );
        assert_eq!( tcirc[6][1], "Gb" );
        assert_eq!( tcirc[7].len(), 1 );
        assert_eq!( tcirc[7][0], "G" );
        assert_eq!( tcirc[8].len(), 2 );
        assert_eq!( tcirc[8][0], "G#" );
        assert_eq!( tcirc[8][1], "Ab" );
        assert_eq!( tcirc[9].len(), 1 );
        assert_eq!( tcirc[9][0], "A" );
        assert_eq!( tcirc[10].len(), 2 );
        assert_eq!( tcirc[10][0], "Bb" );
        assert_eq!( tcirc[10][1], "A#" );
        assert_eq!( tcirc[11].len(), 2 );
        assert_eq!( tcirc[11][0], "B" );
        assert_eq!( tcirc[11][1], "Cb" );
    }

    #[test]
    fn test_split_chord() {
        assert_eq!( t::chord_checker::split_chord( &"A".to_string() ), ( "A".to_string(), "".to_string() ) );
        assert_eq!( t::chord_checker::split_chord( &"G#m/Fb".to_string() ), ( "G#".to_string(), "m/Fb".to_string() ) );
    }
    
    #[test]
    #[should_panic]
    fn test_split_chord_panic() {
        t::chord_checker::split_chord( &"IbM".to_string() );
    }

    #[test]
    fn test_get_trim_note_from_chord() {
        assert_eq!( t::chord_checker::get_note_from_chord(&"D".to_string()), "D".to_string() );
        assert_eq!( t::chord_checker::get_note_from_chord(&"F#m".to_string()), "F#".to_string() );
        assert_eq!( t::chord_checker::get_note_from_chord(&"Gbsus4".to_string()), "Gb".to_string() );
        assert_eq!( t::chord_checker::trim_note_from_chord(&"D".to_string()), "".to_string() );
        assert_eq!( t::chord_checker::trim_note_from_chord(&"F#m".to_string()), "m".to_string() );
        assert_eq!( t::chord_checker::trim_note_from_chord(&"Gbsus4".to_string()), "sus4".to_string() );
    }
    
    #[test]
    #[should_panic]
    fn test_get_note_from_chord_panic() {
        t::chord_checker::get_note_from_chord(&"K".to_string());
    }

    #[test]
    #[should_panic]
    fn test_trim_note_from_chord_panic() {
        t::chord_checker::trim_note_from_chord(&"K".to_string());
    }

    #[test]
    fn test_has_compound_chord() {
        assert!( t::chord_checker::has_compound_chord(&"A/G".to_string()) );
        assert!( t::chord_checker::has_compound_chord(&"Bmsus4/C#".to_string()) );
        assert!( !t::chord_checker::has_compound_chord(&"GM7".to_string()) );
    }

    #[test]
    fn test_split_compound_chord() {
        assert_eq!( t::chord_checker::split_compound_chord( &"Ab/G".to_string() ), ( "Ab".to_string(), "G".to_string() ) );
        assert_eq!( t::chord_checker::split_compound_chord( &"Am7/F#".to_string() ), ( "Am7".to_string(), "F#".to_string() ) );
    }

    #[test]
    #[should_panic]
    fn test_split_compound_chord_panic() {
        t::chord_checker::split_compound_chord( &"AMsus4".to_string() );
    }
    
    #[test]
    fn test_get_trim_compound_chord() {
        assert_eq!( t::chord_checker::get_compound_chord( &"Ab/G".to_string() ), "G".to_string() );
        assert_eq!( t::chord_checker::get_compound_chord( &"Am7/F#".to_string() ), "F#".to_string() );
        assert_eq!( t::chord_checker::trim_compound_chord( &"Ab/G".to_string() ), "Ab".to_string() );
        assert_eq!( t::chord_checker::trim_compound_chord( &"Am7/F#".to_string() ), "Am7".to_string() );
    }

    #[test]
    #[should_panic]
    fn test_get_compound_chord_panic() {
        t::chord_checker::get_compound_chord( &"AMsus4".to_string() );
    }

    #[test]
    #[should_panic]
    fn test_trim_compound_chord_panic() {
        t::chord_checker::trim_compound_chord( &"AMsus4".to_string() );
    }

    #[test]
    fn test_is_chord() {
        assert!( t::chord_checker::is_chord(&"E".to_string()) );
        assert!( t::chord_checker::is_chord(&"E#msus4".to_string()) );
        assert!( t::chord_checker::is_chord(&"GbM7add5/Eb".to_string()) );
        assert!( ! t::chord_checker::is_chord(&"K".to_string()) );
        assert!( ! t::chord_checker::is_chord(&"A#u".to_string()) );
        assert!( ! t::chord_checker::is_chord(&"A#m/pp".to_string()) );
        assert!( ! t::chord_checker::is_chord(&"EbM/G#m".to_string()) );
    }


    //==============================================================================
    
    #[test]
    fn test_check_false_positive() {
        assert!( t::false_positive_chord::check_false_positive(&"A house in a tree".to_string()) );
        assert!( t::false_positive_chord::check_false_positive(&"A little house, CM7/F# Bsus4/F tata".to_string()) );
        assert!( ! t::false_positive_chord::check_false_positive(&"A B# Cb".to_string()) );
        assert!( ! t::false_positive_chord::check_false_positive(&"Am house in a tree".to_string()) );
        assert!( ! t::false_positive_chord::check_false_positive(&"Yes! A house in a tree".to_string()) );
    }
    
    #[test]
    fn test_process_restore_implaced_line() {
        let mut res1 = "A house in a tree".to_string();
        let mut res2 = "A little house, CM7/F# Bsus4/F tata".to_string();
        let mut res3 = "A B# Cb".to_string();
        t::false_positive_chord::process_implaced_line(&mut res1);
        t::false_positive_chord::process_implaced_line(&mut res2);
        t::false_positive_chord::process_implaced_line(&mut res3);
        assert_eq!( res1, ">>>FPC<<< house in a tree".to_string() );
        assert_eq!( res2, ">>>FPC<<< little house, CM7/F# Bsus4/F tata".to_string() );
        assert_eq!( res3, ">>>FPC<<< B# Cb".to_string() );
        t::false_positive_chord::restore_implaced_line(&mut res1);
        t::false_positive_chord::restore_implaced_line(&mut res2);
        t::false_positive_chord::restore_implaced_line(&mut res3);
        assert_eq!( res1, "A house in a tree".to_string() );
        assert_eq!( res2, "A little house, CM7/F# Bsus4/F tata".to_string() );
        assert_eq!( res3, "A B# Cb".to_string() );
    }
    
    //==============================================================================

    #[test]
    fn test_transpose_chord() {
        assert_eq!( t::chord_transposer::transpose_chord( &"B#".to_string(), 0 ), "B#".to_string() );
        assert_eq!( t::chord_transposer::transpose_chord( &"Bm".to_string(), 1 ), "Cm".to_string() );
        assert_eq!( t::chord_transposer::transpose_chord( &"Am7".to_string(), 11 ), "G#m7".to_string() );
        assert_eq!( t::chord_transposer::transpose_chord( &"CM7/F#".to_string(), 5 ), "FM7/B".to_string() );
        assert_eq!( t::chord_transposer::transpose_chord( &"C".to_string(), 50 ), "D".to_string() );
    }

    #[test]
    fn test_transpose_line() {
        assert_eq!( t::chord_transposer::transpose_line( 
                &"A little house, CM7/F# Bsus4/F tata".to_string(), 5 ), 
                 "A little house, FM7/B  Esus4/Bb tata".to_string() );
    }

    //==============================================================================
}
