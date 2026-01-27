use std::{error, fmt, str};

#[derive(Clone, Copy, Debug, Default)]
pub enum ThemeName {
    Frappe,
    Latte,
    Macchiato,
    #[default]
    Mocha,
}

impl From<ThemeName> for iced::Theme {
    fn from(value: ThemeName) -> Self {
        match value {
            ThemeName::Frappe => Self::CatppuccinFrappe,
            ThemeName::Latte => Self::CatppuccinLatte,
            ThemeName::Macchiato => Self::CatppuccinMacchiato,
            ThemeName::Mocha => Self::CatppuccinMocha,
        }
    }
}

impl From<ThemeName> for Palette {
    fn from(value: ThemeName) -> Self {
        match value {
            ThemeName::Frappe => catppuccin::PALETTE.frappe.colors,
            ThemeName::Latte => catppuccin::PALETTE.latte.colors,
            ThemeName::Macchiato => catppuccin::PALETTE.macchiato.colors,
            ThemeName::Mocha => catppuccin::PALETTE.mocha.colors,
        }
        .into()
    }
}

impl str::FromStr for ThemeName {
    type Err = ThemeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "catppuccin-frappe" => Self::Frappe,
            "catppuccin-latte" => Self::Latte,
            "catppuccin-macchiato" => Self::Macchiato,
            "catppuccin-mocha" => Self::Mocha,
            _ => return Err(ThemeError::Unknown(String::from(value))),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub rosewater: iced::Color,
    pub flamingo: iced::Color,
    pub pink: iced::Color,
    pub mauve: iced::Color,
    pub red: iced::Color,
    pub maroon: iced::Color,
    pub peach: iced::Color,
    pub yellow: iced::Color,
    pub green: iced::Color,
    pub teal: iced::Color,
    pub sky: iced::Color,
    pub sapphire: iced::Color,
    pub blue: iced::Color,
    pub lavender: iced::Color,
    pub text: iced::Color,
    pub subtext1: iced::Color,
    pub subtext0: iced::Color,
    pub overlay2: iced::Color,
    pub overlay1: iced::Color,
    pub overlay0: iced::Color,
    pub surface2: iced::Color,
    pub surface1: iced::Color,
    pub surface0: iced::Color,
    pub base: iced::Color,
    pub mantle: iced::Color,
    pub crust: iced::Color,
}

impl From<catppuccin::FlavorColors> for Palette {
    fn from(value: catppuccin::FlavorColors) -> Self {
        macro_rules! convert_color {
            ($value:expr) => {
                iced::Color::from_rgb8($value.rgb.r, $value.rgb.g, $value.rgb.b)
            };
        }
        Self {
            rosewater: convert_color!(value.rosewater),
            flamingo: convert_color!(value.flamingo),
            pink: convert_color!(value.pink),
            mauve: convert_color!(value.mauve),
            red: convert_color!(value.red),
            maroon: convert_color!(value.maroon),
            peach: convert_color!(value.peach),
            yellow: convert_color!(value.yellow),
            green: convert_color!(value.green),
            teal: convert_color!(value.teal),
            sky: convert_color!(value.sky),
            sapphire: convert_color!(value.sapphire),
            blue: convert_color!(value.blue),
            lavender: convert_color!(value.lavender),
            text: convert_color!(value.text),
            subtext1: convert_color!(value.subtext1),
            subtext0: convert_color!(value.subtext0),
            overlay2: convert_color!(value.overlay2),
            overlay1: convert_color!(value.overlay1),
            overlay0: convert_color!(value.overlay0),
            surface2: convert_color!(value.surface2),
            surface1: convert_color!(value.surface1),
            surface0: convert_color!(value.surface0),
            base: convert_color!(value.base),
            mantle: convert_color!(value.mantle),
            crust: convert_color!(value.crust),
        }
    }
}

#[derive(Debug)]
pub enum ThemeError {
    Unknown(String),
}

impl fmt::Display for ThemeError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(name) => write!(out, "unknown theme: {}", name),
        }
    }
}

impl error::Error for ThemeError {}
