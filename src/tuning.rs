use std::{error, fmt, str};

use crate::theme::Palette;

#[derive(Clone, Debug)]
pub struct TuningCollection {
    pub items: Vec<Tuning>,
    selected_idx: usize,
}

impl TuningCollection {
    pub fn new(items: Vec<Tuning>, selected_idx: usize) -> Result<Self, TuningError> {
        let mut result = Self { items, selected_idx };
        result.select(selected_idx)?;
        Ok(result)
    }

    pub fn select(&mut self, idx: usize) -> Result<(), TuningError> {
        if self.items.is_empty() {
            Err(TuningError::CollectionSelectEmpty)
        } else if idx >= self.items.len() {
            Err(TuningError::CollectionSelectIdx(idx))
        } else {
            self.selected_idx = idx;
            Ok(())
        }
    }

    pub fn get_selected(&self) -> &Tuning {
        &self.items[self.selected_idx]
    }
}

impl Default for TuningCollection {
    fn default() -> Self {
        Self {
            items: vec![Tuning::default()],
            selected_idx: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tuning {
    pub pitches: Vec<Pitch>,
    pub total_frets: u8,
    pub name: String,
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
            total_frets: 24,
            name: String::from("Default"),
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

impl fmt::Display for Pitch {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}{}", self.note.format_sharp(), self.octave)
    }
}

impl str::FromStr for Pitch {
    type Err = TuningError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let err = move || TuningError::parse_pitch(value);
        let mut chars = value.chars().rev();
        let octave: i8 = chars
            .next()
            .and_then(|x| x.to_digit(10))
            .ok_or_else(err)
            .and_then(|x| x.try_into().map_err(|_| err()))?;
        let c = chars.next().ok_or_else(err)?;
        let (octave, c) = if c == '-' {
            (-octave, chars.next().ok_or_else(err)?)
        } else {
            (octave, c)
        };
        let acc: i8 = match c {
            '#' => 1,
            'b' => -1,
            _ => 0,
        };
        let note = if acc != 0 { chars.next().ok_or_else(err)? } else { c };
        let note = match (note, acc) {
            ('A', 0) => Note::A,
            ('A', 1) | ('B', -1) => Note::Bb,
            ('B', 0) => Note::B,
            ('C', 0) => Note::C,
            ('C', 1) | ('D', -1) => Note::Db,
            ('D', 0) => Note::D,
            ('D', 1) | ('E', -1) => Note::Eb,
            ('E', 0) => Note::E,
            ('F', 0) => Note::F,
            ('F', 1) | ('G', -1) => Note::Gb,
            ('G', 0) => Note::G,
            ('G', 1) | ('A', -1) => Note::Ab,
            (_, _) => return Err(err()),
        };
        Ok(Self { note, octave })
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
    pub fn get_color(self, palette: Palette) -> iced::Color {
        match self {
            Self::A => palette.sapphire,
            Self::Bb => palette.sky,
            Self::B => palette.teal,
            Self::C => palette.green,
            Self::Db => palette.yellow,
            Self::D => palette.peach,
            Self::Eb => palette.maroon,
            Self::E => palette.red,
            Self::F => palette.mauve,
            Self::Gb => palette.pink,
            Self::G => palette.lavender,
            Self::Ab => palette.blue,
        }
    }

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

#[derive(Debug)]
pub enum TuningError {
    CollectionSelectEmpty,
    CollectionSelectIdx(usize),
    ParsePitch(String),
}

impl TuningError {
    fn parse_pitch(value: impl Into<String>) -> Self {
        Self::ParsePitch(value.into())
    }
}

impl fmt::Display for TuningError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CollectionSelectEmpty => write!(out, "collection is empty"),
            Self::CollectionSelectIdx(idx) => write!(out, "invalid tuning index: {}", idx),
            Self::ParsePitch(value) => write!(out, "parse pitch: {}", value),
        }
    }
}

impl error::Error for TuningError {}
