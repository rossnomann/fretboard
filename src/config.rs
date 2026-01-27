use std::{env, error, fmt, fs, io, path};

use miette::Report;

use crate::{
    theme::{ThemeError, ThemeName},
    tuning::{NoteFormat, NoteFormatError, Pitch, Tuning, TuningCollection, TuningError},
};

pub const APPLICATION_ID: &str = "com.rossnomann.fretboard";
pub const APPLICATION_TITLE: &str = "Fretboard";

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub note_format: NoteFormat,
    pub tuning: TuningCollection,
    pub theme_name: ThemeName,
}

impl Config {
    pub fn read_from_file() -> Result<Self, ConfigError> {
        let config_path = match env::var("FRETBOARD_CONFIG_PATH") {
            Ok(value) => {
                log::info!("Reading configuration from FRETBOARD_CONFIG_PATH: {}", value);
                Some(path::PathBuf::from(value))
            }
            Err(_) => {
                let base_dirs = xdg::BaseDirectories::with_prefix("fretboard");
                base_dirs
                    .get_config_file("config.kdl")
                    .inspect(|x| {
                        log::info!("Configration file path: {}", x.display());
                    })
                    .or_else(|| {
                        log::warn!("Configuration path is not found, using default config");
                        None
                    })
            }
        }
        .and_then(|x| {
            if x.exists() {
                Some(x)
            } else {
                log::error!("No such file: {}, using default config", x.display());
                None
            }
        });
        if let Some(config_path) = config_path {
            let data = fs::read_to_string(&config_path)?;
            let file_name = config_path
                .file_name()
                .map(|x| format!("{}", x.display()))
                .unwrap_or(String::from("config.kdl"));
            let schema: Schema = knus::parse(file_name, &data)?;
            Self::try_from(schema)
        } else {
            Ok(Self::default())
        }
    }
}

impl TryFrom<Schema> for Config {
    type Error = ConfigError;

    fn try_from(value: Schema) -> Result<Self, Self::Error> {
        let default_total_frets = value.default.frets.unwrap_or(Tuning::DEFAULT_TOTAL_FRETS);
        let tunings: Vec<Tuning> = value
            .tuning
            .into_iter()
            .map(|x| x.try_into_tuning(default_total_frets))
            .collect::<Result<_, ConfigError>>()?;
        let default_tuning = value
            .default
            .tuning
            .and_then(|name| {
                tunings
                    .iter()
                    .enumerate()
                    .find(|(_idx, x)| x.name.as_str() == name)
                    .map(|(idx, _x)| idx)
            })
            .unwrap_or(0);
        Ok(Self {
            note_format: match value.default.note_format {
                Some(x) => x.parse()?,
                None => NoteFormat::default(),
            },
            tuning: TuningCollection::new(tunings, default_tuning)?,
            theme_name: match value.default.theme_name {
                Some(x) => x.parse()?,
                None => ThemeName::default(),
            },
        })
    }
}

#[derive(Clone, Debug, knus::Decode)]
struct Schema {
    #[knus(child)]
    default: SchemaDefault,
    #[knus(children(name = "tuning"))]
    tuning: Vec<SchemaTuning>,
}

#[derive(Clone, Debug, knus::Decode)]
struct SchemaDefault {
    #[knus(child, unwrap(argument))]
    frets: Option<u8>,
    #[knus(child, unwrap(argument))]
    tuning: Option<String>,
    #[knus(child, unwrap(argument))]
    note_format: Option<String>,
    #[knus(child, unwrap(argument))]
    theme_name: Option<String>,
}

#[derive(Clone, Debug, knus::Decode)]
struct SchemaTuning {
    #[knus(property)]
    frets: Option<u8>,
    #[knus(property)]
    name: Option<String>,
    #[knus(arguments)]
    data: Vec<String>,
}

impl SchemaTuning {
    fn try_into_tuning(self, default_total_frets: u8) -> Result<Tuning, ConfigError> {
        let pitches: Vec<Pitch> = self
            .data
            .into_iter()
            .map(|x| x.parse::<Pitch>())
            .collect::<Result<_, TuningError>>()?;
        let total_frets = self.frets.unwrap_or(default_total_frets);
        let name = self.name.unwrap_or_else(|| {
            pitches.iter().fold(String::new(), |mut acc, x| {
                acc.push_str(&x.to_string());
                acc
            })
        });
        Ok(Tuning {
            pitches,
            total_frets,
            name,
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    ParseKdl(Report),
    ParseNoteFormat(NoteFormatError),
    ParseTheme(ThemeError),
    ParseTuning(TuningError),
    ReadFile(io::Error),
}

impl From<knus::Error> for ConfigError {
    fn from(value: knus::Error) -> Self {
        Self::ParseKdl(Report::new(value))
    }
}

impl From<NoteFormatError> for ConfigError {
    fn from(value: NoteFormatError) -> Self {
        Self::ParseNoteFormat(value)
    }
}

impl From<ThemeError> for ConfigError {
    fn from(value: ThemeError) -> Self {
        Self::ParseTheme(value)
    }
}

impl From<TuningError> for ConfigError {
    fn from(value: TuningError) -> Self {
        Self::ParseTuning(value)
    }
}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self::ReadFile(value)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseKdl(err) => write!(out, "{}", err),
            Self::ParseNoteFormat(err) => write!(out, "parse note format: {}", err),
            Self::ParseTheme(err) => write!(out, "parse theme: {}", err),
            Self::ParseTuning(err) => write!(out, "parse tuning: {}", err),
            Self::ReadFile(err) => write!(out, "read file: {}", err),
        }
    }
}

impl error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Self::ParseKdl(_) => return None,
            Self::ParseNoteFormat(err) => err,
            Self::ParseTheme(err) => err,
            Self::ParseTuning(err) => err,
            Self::ReadFile(err) => err,
        })
    }
}
