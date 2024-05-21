#[derive(Debug)]
enum Note {
    C3,
    Cs3,
    D3,
    Ds3,
    E3,
    F3,
    Fs3,
    G3,
    Gs3,
    A3,
    As3,
    B3,
    C4,
    Cs4,
    D4,
    Ds4,
    E4,
    F4,
    Fs4,
    G4,
    Gs4,
    A4,
    As4,
    B4,
    C5,
    Cs5,
    D5,
    Ds5,
    E5,
    F5,
    Fs5,
    G5,
    Gs5
}


pub struct Playing{
    pub hole: u8,
    pub draw: bool,
    pub slide_in: bool,
}

fn note_to_playing(note: &Note) -> Vec<Playing>{
    match note {
        Note::C3 => vec![Playing{ hole: 1, draw: false, slide_in: false}],
        Note::Cs3 => vec![Playing{ hole: 1, draw: false, slide_in: true}],
        Note::D3 => vec![Playing{ hole: 1, draw: true, slide_in: false}],
        Note::Ds3 => vec![Playing{ hole: 1, draw: false, slide_in: true}],

        Note::E3 => vec![Playing{ hole: 2, draw: false, slide_in: false}],
        Note::F3 => vec![Playing{ hole: 2, draw: true, slide_in: false},
                         Playing{ hole: 2, draw: false, slide_in: true}],
        Note::Fs3 => vec![Playing{ hole: 2, draw: true, slide_in: true}],

        Note::G3 => vec![Playing{ hole: 3, draw: false, slide_in: false}],
        Note::Gs3 => vec![Playing{ hole: 3, draw: false, slide_in: true}],
        Note::A3 => vec![Playing{ hole: 3, draw: true, slide_in: false}],
        Note::As3 => vec![Playing{ hole: 3, draw: true, slide_in: true}],

        Note::B3 => vec![Playing{ hole: 4, draw: true, slide_in: false}],
        Note::C4 => vec![Playing{ hole: 4, draw: false, slide_in: false},
                         Playing{ hole: 4, draw: true, slide_in: true},
                         Playing{ hole: 5, draw: false, slide_in: false}],
        Note::Cs4 => vec![Playing{ hole: 4, draw: false, slide_in: true},
                          Playing{ hole: 5, draw: false, slide_in: true}],

        Note::D4 => vec![Playing{ hole: 5, draw: true, slide_in: false}],
        Note::Ds4 => vec![Playing{ hole: 5, draw: true, slide_in: true}],

        Note::E4 => vec![Playing{ hole: 6, draw: false, slide_in: true}],
        Note::F4 => vec![Playing{ hole: 6, draw: true, slide_in: false},
                         Playing{ hole: 6, draw: false, slide_in: true}],
        Note::Fs4 => vec![Playing{ hole: 6, draw: true, slide_in: true}],

        Note::G4 => vec![Playing{ hole: 7, draw: false, slide_in: false}],
        Note::Gs4 => vec![Playing{ hole: 7, draw: false, slide_in: true}],
        Note::A4 => vec![Playing{ hole: 7, draw: true, slide_in: false}],
        Note::As4 => vec![Playing{ hole: 7, draw: true, slide_in: true}],

        Note::B4 => vec![Playing{ hole: 8, draw: true, slide_in: false}],
        Note::C5 => vec![Playing{ hole: 8, draw: false, slide_in: false},
                         Playing{ hole: 8, draw: true, slide_in: true}],
        Note::Cs5 => vec![Playing{ hole: 8, draw: false, slide_in: true}],

        Note::D5 => vec![Playing{ hole: 9, draw: true, slide_in: false}],
        Note::Ds5 => vec![Playing{ hole: 9, draw: true, slide_in: true}],
        Note::E5 => vec![Playing{ hole: 9, draw: false, slide_in: false}],
        Note::F5 => vec![Playing{ hole: 9, draw: false, slide_in: true},
                         Playing{ hole: 10, draw: true, slide_in: false}],

        Note::Fs5 => vec![Playing{ hole: 10, draw: true, slide_in: true}],
        Note::G5 => vec![Playing{ hole: 10, draw: false, slide_in: false}],
        Note::Gs5 => vec![Playing{ hole: 10, draw: false, slide_in: true}],
    }
}

fn str_to_note(note: &str) -> Note{
    match note {
        "C3" => Note::C3,
        "C#3" => Note::Cs3,
        "Db3" => Note::Cs3,
        "D3" => Note::D3,
        "D#3" => Note::Ds3,
        "Eb3" => Note::Ds3,
        "E3" => Note::E3,
        "Fb3" => Note::E3,
        "E#3" => Note::F3,
        "F3" => Note::F3,
        "F#3" => Note::Fs3,
        "Gb3" => Note::Fs3,
        "G3" => Note::G3,
        "G#3" => Note::Gs3,
        "Ab3" => Note::Gs3,
        "A3" => Note::A3,
        "A#3" => Note::As3,
        "Bb3" => Note::As3,
        "B3" => Note::B3,
        "Cb4" => Note::B3,
        "B#3" => Note::C4,
        "C4" => Note::C4,
        "C#4" => Note::Cs4,
        "Db4" => Note::Cs4,
        "D4" => Note::D4,
        "D#4" => Note::Ds4,
        "Eb4" => Note::Ds4,
        "E4" => Note::E4,
        "Fb4" => Note::E4,
        "E#4" => Note::F4,
        "F4" => Note::F4,
        "F#4" => Note::Fs4,
        "Gb4" => Note::Fs4,
        "G4" => Note::G4,
        "G#4" => Note::Gs4,
        "Ab4" => Note::Gs4,
        "A4" => Note::A4,
        "A#4" => Note::As4,
        "Bb4" => Note::As4,
        "B4" => Note::B4,
        "Cb5" => Note::B4,
        "B#4" => Note::C5,
        "C5" => Note::C5,
        "C#5" => Note::Cs5,
        "Db5" => Note::Cs5,
        "D5" => Note::D5,
        "D#5" => Note::Ds5,
        "Eb5" => Note::Ds5,
        "E5" => Note::E5,
        "Fb5" => Note::E5,
        "E#5" => Note::F5,
        "F5" => Note::F5,
        "F#5" => Note::Fs5,
        "Gb5" => Note::Fs5,
        "G5" => Note::G5,
        "G#5" => Note::Gs5,
        "Ab5" => Note::Gs5,
        _ => Note::C3
    }
}

fn playing_to_string(playing: Playing) -> String{
    let draw: String = match playing.draw {
        true => String::from("-"),
        false => String::from("+"),
    };

    let hole: String = playing.hole.to_string();

    let slide_in: String = match playing.slide_in {
        true => String::from("^"),
        false => String::from(" "),
    };

    draw + &*hole + &*slide_in

}

fn format_sequence(sequence: String) -> Vec<Note> {
    sequence.split(' ').map(
        |n| {str_to_note(n)}
    ).collect()
}

fn format_optimal_playings(sequence: Vec<Note>) -> u8{
    let playings: Vec<Vec<Playing>> = sequence.iter().map(
        |n| {note_to_playing(n)}
    ).collect();

    // Here we count how much notes we have to play with slide in
    let slide_in_sum = playings.iter().map(
        |p| {
            if (p.len() == 1) && (p[0].slide_in) {
                1u8
            }
            else {
                0u8
            }
        }
    ).collect::<Vec<u8>>().iter().fold(0u8, |acc: u8, x: &u8| acc + x);
    let choose_slide_in: bool = slide_in_sum < playings.len() as u8;

    // If there is a lot of slide in notes, we choose to play with slide in when we have multiple
    // playing options
    let _ = playings.iter().map(
        |ps:Vec<&Playing>|
            {
                if ps.len() > 1 {
                    ps.iter().filter(
                        |p|
                            p.slide_in
                    ).collect::<Vec<&Playing>>()
                } else { ps }

            }
    ).collect::<Vec<Vec<&Playing>>>();
    1u8
}



fn main() {
    println!("{:?}",format_sequence(String::from("C#3 D3 D4")));
}
