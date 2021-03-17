// It's CONSTANTS time !

// Plain old notes
pub const C_FLAT: &'static str = "Cb";
pub const C_NATURAL: &'static str = "C";
pub const C_SHARP: &'static str = "C#";
pub const D_FLAT: &'static str = "Db";
pub const D_NATURAL: &'static str = "D";
pub const D_SHARP: &'static str = "D#";
pub const E_FLAT: &'static str = "Eb";
pub const E_NATURAL: &'static str = "E";
pub const E_SHARP: &'static str = "E#";
pub const F_FLAT: &'static str = "Fb";
pub const F_NATURAL: &'static str = "F";
pub const F_SHARP: &'static str = "F#";
pub const G_FLAT: &'static str = "Gb";
pub const G_NATURAL: &'static str = "G";
pub const G_SHARP: &'static str = "G#";
pub const A_FLAT: &'static str = "Ab";
pub const A_NATURAL: &'static str = "A";
pub const A_SHARP: &'static str = "A#";
pub const B_FLAT: &'static str = "Bb";
pub const B_NATURAL: &'static str = "B";
pub const B_SHARP: &'static str = "B#";

// Order is important, naturals should be after sharps and flats
pub const ALL_NOTES: &'static [&'static str] = &[
    C_FLAT,
    C_SHARP,
    C_NATURAL,
    D_FLAT,
    D_SHARP,
    D_NATURAL,
    E_FLAT,
    E_SHARP,
    E_NATURAL,
    F_FLAT,
    F_SHARP,
    F_NATURAL,
    G_FLAT,
    G_SHARP,
    G_NATURAL,
    A_FLAT,
    A_SHARP,
    A_NATURAL,
    B_FLAT,
    B_SHARP,
    B_NATURAL
]; 


// Compound chords
pub const CHORD_INVERSION: &'static str = "/";

// May also include any other numbered suffixes
pub const ALL_TYPES: &'static [&'static str] = &["m","M","sus","add","dim","5","6","7","9","11",CHORD_INVERSION];

