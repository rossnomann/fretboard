use std::{env, error, fmt, fs, io, path};

use serde::Deserialize;

use crate::{
    theme::ThemeName,
    tuning::{Pitch, Tuning, TuningCollection, TuningError},
};

pub const APPLICATION_ID: &str = "com.rossnomann.fretboard";
pub const APPLICATION_TITLE: &str = "Fretboard";

#[derive(Clone, Debug)]
pub struct Config {
    pub tuning: TuningCollection,
    pub theme_name: ThemeName,
}

impl Config {
    pub fn read_from_file() -> Result<Self, ConfigError> {
        let config_path = match env::var("FRETBOARD_CONFIG_PATH") {
            Ok(value) => {
                log::info!("Reading configuration from FRETBOARD_CONFIG_PATH: {}", value);
                path::PathBuf::from(value)
            }
            Err(_) => {
                let base_dirs = xdg::BaseDirectories::with_prefix("fretboard")?;
                base_dirs.get_config_file("config.toml")
            }
        };
        log::info!("Configration file path: {}", config_path.display());
        if config_path.exists() {
            let data = fs::read_to_string(config_path)?;
            let schema: SchemaConfig = toml::from_str(&data)?;
            Self::try_from(schema)
        } else {
            log::error!("Configuration file is not found, using default");
            Ok(Self::default())
        }
    }
}

impl TryFrom<SchemaConfig> for Config {
    type Error = ConfigError;

    fn try_from(value: SchemaConfig) -> Result<Self, Self::Error> {
        let default_total_frets = value.default_frets;
        let tunings: Vec<Tuning> = value
            .tuning
            .into_iter()
            .map(|x| x.into_tuning(default_total_frets))
            .collect::<Result<_, ConfigError>>()?;
        let default_tuning = value
            .default_tuning
            .and_then(|name| {
                tunings
                    .iter()
                    .enumerate()
                    .find(|(_idx, x)| x.name.as_str() == name)
                    .map(|(idx, _x)| idx)
            })
            .unwrap_or(0);
        Ok(Self {
            tuning: TuningCollection::new(tunings, default_tuning)?,
            theme_name: value.theme_name,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tuning: TuningCollection::default(),
            theme_name: ThemeName::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct SchemaConfig {
    default_frets: u8,
    default_tuning: Option<String>,
    theme_name: ThemeName,
    tuning: Vec<SchemaTuning>,
}

#[derive(Clone, Debug, Deserialize)]
struct SchemaTuning {
    data: Vec<String>,
    frets: Option<u8>,
    name: Option<String>,
}

impl SchemaTuning {
    fn into_tuning(self, default_total_frets: u8) -> Result<Tuning, ConfigError> {
        let pitches: Vec<Pitch> = self
            .data
            .into_iter()
            .map(|x| x.parse::<Pitch>())
            .collect::<Result<_, TuningError>>()?;
        let total_frets = self.frets.unwrap_or(default_total_frets);
        let name = self.name.unwrap_or_else(|| {
            pitches.iter().fold(String::new(), |mut acc, x| {
                acc.extend(x.to_string().chars());
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
    BaseDirectories(xdg::BaseDirectoriesError),
    ParseToml(toml::de::Error),
    ParseTuning(TuningError),
    ReadFile(io::Error),
}

impl From<xdg::BaseDirectoriesError> for ConfigError {
    fn from(value: xdg::BaseDirectoriesError) -> Self {
        Self::BaseDirectories(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::ParseToml(value)
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
            Self::BaseDirectories(err) => err.fmt(out),
            Self::ParseToml(err) => write!(out, "parse TOML: {}", err),
            Self::ParseTuning(err) => write!(out, "parse tuning: {}", err),
            Self::ReadFile(err) => write!(out, "read file: {}", err),
        }
    }
}

impl error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Self::BaseDirectories(err) => err,
            Self::ParseToml(err) => err,
            Self::ParseTuning(err) => err,
            Self::ReadFile(err) => err,
        })
    }
}
