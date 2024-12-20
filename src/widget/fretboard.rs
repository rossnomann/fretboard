use crate::tuning::{Pitch, Tuning};

#[derive(Debug)]
pub struct Fretboard {
    frets_count: u8,
    tuning: Tuning,
}

impl Fretboard {
    pub fn new(frets_count: u8, tuning: Tuning) -> Self {
        Self { frets_count, tuning }
    }
}

impl<M, T, R> iced::advanced::Widget<M, T, R> for Fretboard
where
    R: iced::advanced::renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn size(&self) -> iced::Size<iced::Length> {
        let width = iced::Length::Fill;
        let height = iced::Length::Fill;
        iced::Size::new(width, height)
    }

    fn layout(
        &self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &R,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let size = iced::advanced::Widget::<M, T, R>::size(self);
        let width = size.width;
        let height = size.height;
        iced::advanced::layout::atomic(limits, width, height)
    }

    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        renderer: &mut R,
        _theme: &T,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::layout::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let frets_count = self.frets_count;
        if frets_count == 0 {
            return;
        }

        let pitches = &self.tuning.pitches;
        let strings_count = pitches.len();
        if strings_count == 0 {
            return;
        }

        let layout_bounds = layout.bounds();
        let max_size = layout_bounds.size();
        if max_size == iced::Size::ZERO {
            return;
        }

        let mut fill_rect = |bounds: iced::Rectangle, color: iced::Color| {
            renderer.fill_quad(
                iced::advanced::renderer::Quad {
                    bounds,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                },
                iced::Background::Color(color),
            );
        };

        fill_rect(
            iced::Rectangle::new(iced::Point::ORIGIN, max_size),
            iced::Color::from_rgb(0.53, 0.69, 0.30),
        );

        let widget_layout = Layout::new(frets_count, strings_count, max_size);
        let pitches = match widget_layout.orientation {
            Orientation::Horizontal => &mut pitches.iter().rev() as &mut dyn Iterator<Item = &Pitch>,
            Orientation::Vertical => &mut pitches.iter() as &mut dyn Iterator<Item = &Pitch>,
        };

        fill_rect(widget_layout.bbox, iced::Color::BLACK);
        fill_rect(widget_layout.bbox_nut, iced::Color::from_rgb(255.0, 0.0, 0.0));
        let fret_background_color = iced::Color::from_rgb(0.0, 255.0, 0.0);
        (1..=frets_count)
            .map(|x| widget_layout.calculate_fret(x))
            .for_each(|fret| fill_rect(fret, fret_background_color));
        let fret_marker_background_color = iced::Color::from_rgb(0.0, 0.0, 255.0);
        (1..=frets_count)
            .zip(Layout::FRET_MARKERS_MARKUP.into_iter().cycle())
            .map(|(fret_number, markers_count)| widget_layout.calculate_fret_marker(fret_number, markers_count))
            .for_each(|fret_marker| match fret_marker {
                FretMarker::Single(bounds) => fill_rect(bounds, fret_marker_background_color),
                FretMarker::Double(bounds_a, bounds_b) => {
                    fill_rect(bounds_a, fret_marker_background_color);
                    fill_rect(bounds_b, fret_marker_background_color);
                }
                FretMarker::None => {}
            });
        let string_background_color = iced::Color::from_rgb(255.0, 255.0, 0.0);
        (1..=strings_count)
            .map(|x| widget_layout.calculate_string(x))
            .for_each(|string| fill_rect(string, string_background_color));
        let note_label_color_bg = iced::Color::from_rgb(50.0, 50.0, 50.0);
        let note_label_color_fg = iced::Color::from_rgb(0.0, 0.0, 0.0);
        pitches
            .enumerate()
            .flat_map(|(pitch_number, pitch_origin)| {
                let pitch_number = (pitch_number + 1) as f32;
                (0..=frets_count).zip(*pitch_origin).map(move |(fret_number, pitch)| {
                    widget_layout.calculate_note_label(fret_number, pitch_number, pitch)
                })
            })
            .for_each(move |note_label| {
                renderer.fill_quad(
                    iced::advanced::renderer::Quad {
                        bounds: note_label.clip_bounds,
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    },
                    iced::Background::Color(note_label_color_bg),
                );
                renderer.fill_text(
                    note_label.text,
                    note_label.location,
                    note_label_color_fg,
                    note_label.clip_bounds,
                )
            });
    }
}

impl<M, T, R> From<Fretboard> for iced::Element<'_, M, T, R>
where
    R: iced::advanced::renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn from(value: Fretboard) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn transform_point(&self, value: iced::Point) -> iced::Point {
        match self {
            Self::Horizontal => value,
            Self::Vertical => iced::Point::new(value.y, value.x),
        }
    }
    fn transform_size(&self, value: iced::Size) -> iced::Size {
        match self {
            Self::Horizontal => value,
            Self::Vertical => iced::Size::new(value.height, value.width),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Layout {
    bbox: iced::Rectangle,
    bbox_nut: iced::Rectangle,
    orientation: Orientation,
    origin: iced::Point,
    origin_fret: f32,
    origin_fret_marker_double_a: f32,
    origin_fret_marker_double_b: f32,
    origin_fret_marker_single: f32,
    origin_nut: f32,
    size_fret: iced::Size,
    size_fret_marker: iced::Size,
    size_note_label: NoteLabelSize,
    size_string: iced::Size,
    spacing_fret: f32,
    spacing_string: f32,
}

impl Layout {
    const FRET_MARKER_SCALE: f32 = 0.1;
    const FRET_MARKERS_MARKUP: [u8; 12] = [0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 2];
    const FRET_SCALE: f32 = 0.002;
    const NUT_SCALE: f32 = 0.002;
    const STRING_SCALE: f32 = 0.005;

    fn new(frets_count: u8, strings_count: usize, max_size: iced::Size) -> Self {
        let frets_count = frets_count as f32;
        let strings_count = strings_count as f32;

        let (width_frets, width_pitches, orientation) = if max_size.width > max_size.height {
            (max_size.width, max_size.height, Orientation::Horizontal)
        } else {
            (max_size.height, max_size.width, Orientation::Vertical)
        };
        let ratio = frets_count / strings_count;
        let length_frets = (width_frets / frets_count) * frets_count;
        let length_pitches = length_frets / ratio;

        let origin = iced::Point::new(
            (width_frets - length_frets) / 2.0,
            (width_pitches - length_pitches) / 2.0,
        );

        let size_note_label = NoteLabelSize::new(length_frets);
        let nut_width = length_frets * Self::NUT_SCALE;

        let origin_nut = origin.x + size_note_label.width * 1.25;
        let origin_fret = origin_nut + nut_width;

        let spacing_fret = (length_frets - origin_fret) / (frets_count + 1.0);
        let spacing_string = length_pitches / (strings_count + 1.0);

        let fret_marker_width = spacing_fret * Self::FRET_MARKER_SCALE;
        let origin_fret_marker_single = origin.y + ((length_pitches / 2.0) - (fret_marker_width / 2.0));

        Self {
            bbox: iced::Rectangle::new(
                orientation.transform_point(origin),
                orientation.transform_size(iced::Size::new(length_frets, length_pitches)),
            ),
            bbox_nut: iced::Rectangle::new(
                orientation.transform_point(iced::Point::new(origin_nut, origin.y)),
                orientation.transform_size(iced::Size::new(nut_width, length_pitches)),
            ),
            orientation,
            origin,
            origin_fret,
            origin_fret_marker_double_a: origin_fret_marker_single - fret_marker_width,
            origin_fret_marker_double_b: origin_fret_marker_single + fret_marker_width,
            origin_fret_marker_single,
            origin_nut,
            size_fret: orientation.transform_size(iced::Size::new(length_frets * Self::FRET_SCALE, length_pitches)),
            size_fret_marker: iced::Size::new(fret_marker_width, fret_marker_width),
            size_note_label,
            size_string: orientation.transform_size(iced::Size::new(
                length_frets - origin_nut,
                length_pitches * Self::STRING_SCALE,
            )),
            spacing_fret,
            spacing_string,
        }
    }

    fn calculate_fret(&self, fret_number: u8) -> iced::Rectangle {
        let x = self.origin_fret + (self.spacing_fret * fret_number as f32);
        let point = self.orientation.transform_point(iced::Point::new(x, self.origin.y));
        iced::Rectangle::new(point, self.size_fret)
    }

    fn calculate_fret_marker(&self, fret_number: u8, markers_count: u8) -> FretMarker {
        let x = self.origin_fret + (self.spacing_fret * fret_number as f32) - (self.spacing_fret / 2.0);
        let orientation = self.orientation;
        let pos = |y: f32| orientation.transform_point(iced::Point::new(x, y));
        match markers_count {
            1 => FretMarker::Single(iced::Rectangle::new(
                pos(self.origin_fret_marker_single),
                self.size_fret_marker,
            )),
            2 => FretMarker::Double(
                iced::Rectangle::new(pos(self.origin_fret_marker_double_a), self.size_fret_marker),
                iced::Rectangle::new(pos(self.origin_fret_marker_double_b), self.size_fret_marker),
            ),
            _ => FretMarker::None,
        }
    }

    fn calculate_note_label(&self, fret_number: u8, pitch_number: f32, pitch: Pitch) -> NoteLabel {
        let x = (self.origin_fret + fret_number as f32 * self.spacing_fret) - self.size_note_label.value * 1.2;
        let y = self.origin.y + pitch_number * self.spacing_string;
        let point = self.orientation.transform_point(iced::Point::new(x, y));
        NoteLabel::new(
            point,
            format!("{}{}", pitch.note.format_sharp(), pitch.octave),
            self.size_note_label,
        )
    }

    fn calculate_string(&self, string_number: usize) -> iced::Rectangle {
        let y = self.origin.y + self.spacing_string * string_number as f32;
        let point = self.orientation.transform_point(iced::Point::new(self.origin_nut, y));
        iced::Rectangle::new(point, self.size_string)
    }
}

#[derive(Debug)]
enum FretMarker {
    Single(iced::Rectangle),
    Double(iced::Rectangle, iced::Rectangle),
    None,
}

#[derive(Clone, Copy, Debug)]
struct NoteLabelSize {
    bounds: iced::Size,
    font: iced::Pixels,
    line_heigth: iced::advanced::text::LineHeight,
    value: f32,
    width: f32,
}

impl NoteLabelSize {
    const FONT_SCALE: f32 = 0.012;

    fn new(frets_length: f32) -> Self {
        let font = frets_length * Self::FONT_SCALE;
        let width = font * 1.9;
        Self {
            bounds: iced::Size::new(width, width),
            font: iced::Pixels::from(font),
            line_heigth: iced::advanced::text::LineHeight::Relative(1.0),
            value: font,
            width,
        }
    }

    fn calculate_clip_bounds(&self, location: iced::Point) -> iced::Rectangle {
        iced::Rectangle::new(
            iced::Point::new(location.x - self.value, location.y - self.value),
            self.bounds,
        )
    }
}

#[derive(Debug)]
struct NoteLabel {
    clip_bounds: iced::Rectangle,
    location: iced::Point,
    text: iced::advanced::Text,
}

impl NoteLabel {
    fn new(location: iced::Point, text: String, size: NoteLabelSize) -> Self {
        Self {
            clip_bounds: size.calculate_clip_bounds(location),
            location,
            text: iced::advanced::text::Text {
                content: text,
                bounds: size.bounds,
                size: size.font,
                line_height: size.line_heigth,
                font: iced::Font::MONOSPACE,
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                shaping: iced::advanced::text::Shaping::Advanced,
                wrapping: iced::advanced::text::Wrapping::None,
            },
        }
    }
}
