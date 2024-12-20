#[derive(Clone, Debug)]
pub struct Tuning {
    pub pitches: Vec<Pitch>,
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            pitches: vec![
                Pitch::new(Note::E, 2),
                Pitch::new(Note::A, 2),
                Pitch::new(Note::D, 3),
                Pitch::new(Note::G, 3),
                Pitch::new(Note::B, 3),
                Pitch::new(Note::E, 4),
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pitch {
    pub note: Note,
    pub octave: i8,
}

impl Pitch {
    fn new(note: Note, octave: i8) -> Self {
        Self { note, octave }
    }

    pub fn next(self) -> Self {
        let next_note = self.note.next();
        Self::new(
            next_note,
            if let (Note::B, Note::C) = (self.note, next_note) {
                self.octave + 1
            } else {
                self.octave
            },
        )
    }
}

pub struct PitchIter {
    pitch: Pitch,
}

impl IntoIterator for Pitch {
    type Item = Pitch;
    type IntoIter = PitchIter;
    fn into_iter(self) -> Self::IntoIter {
        PitchIter { pitch: self }
    }
}

impl Iterator for PitchIter {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.pitch);
        self.pitch = self.pitch.next();
        result
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Note {
    A,
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
}

impl Note {
    pub fn format_flat(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::Bb => "Bb",
            Self::B => "B",
            Self::C => "C",
            Self::Db => "Db",
            Self::D => "D",
            Self::Eb => "Eb",
            Self::E => "E",
            Self::F => "F",
            Self::Gb => "Gb",
            Self::G => "G",
            Self::Ab => "Ab",
        }
    }

    pub fn format_sharp(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::Bb => "A#",
            Self::B => "B",
            Self::C => "C",
            Self::Db => "C#",
            Self::D => "D",
            Self::Eb => "D#",
            Self::E => "E",
            Self::F => "F",
            Self::Gb => "F#",
            Self::G => "G",
            Self::Ab => "G#",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::A => Self::Bb,
            Self::Bb => Self::B,
            Self::B => Self::C,
            Self::C => Self::Db,
            Self::Db => Self::D,
            Self::D => Self::Eb,
            Self::Eb => Self::E,
            Self::E => Self::F,
            Self::F => Self::Gb,
            Self::Gb => Self::G,
            Self::G => Self::Ab,
            Self::Ab => Self::A,
        }
    }
}
