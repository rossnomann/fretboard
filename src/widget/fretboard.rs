use crate::{
    theme::Palette,
    tuning::{Pitch, Tuning},
};

#[derive(Debug)]
pub struct Fretboard {
    tuning: Tuning,
    palette: Palette,
}

impl Fretboard {
    pub fn new(tuning: Tuning, palette: impl Into<Palette>) -> Self {
        Self {
            tuning,
            palette: palette.into(),
        }
    }
}

impl<M, R> iced::advanced::Widget<M, iced::Theme, R> for Fretboard
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
        let size = iced::advanced::Widget::<M, iced::Theme, R>::size(self);
        let width = size.width;
        let height = size.height;
        iced::advanced::layout::atomic(limits, width, height)
    }

    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        renderer: &mut R,
        _theme: &iced::Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::layout::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let frets_count = self.tuning.total_frets;
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

        Bounds::new(layout_bounds, self.palette.mantle).render(renderer);

        let widget_layout = Layout::new(frets_count, strings_count, layout_bounds, self.palette);
        let pitches = match widget_layout.cx.orientation {
            Orientation::Horizontal => &mut pitches.iter().rev() as &mut dyn Iterator<Item = &Pitch>,
            Orientation::Vertical => &mut pitches.iter() as &mut dyn Iterator<Item = &Pitch>,
        };

        widget_layout.calculate_nut().render(renderer);
        (1..=frets_count)
            .map(|x| widget_layout.calculate_fret(x))
            .for_each(|x| x.render(renderer));
        (1..=frets_count)
            .zip(FretMarkerType::MARKUP.into_iter().cycle())
            .filter_map(|(fret_number, marker_type)| {
                marker_type.map(|marker_type| widget_layout.calculate_fret_marker(fret_number, marker_type))
            })
            .for_each(|x| x.render(renderer));
        (1..=strings_count)
            .map(|x| widget_layout.calculate_string(x))
            .for_each(|x| x.render(renderer));
        pitches
            .enumerate()
            .flat_map(|(pitch_number, pitch_origin)| {
                let string_number = pitch_number + 1;
                (0..=frets_count).zip(*pitch_origin).map(move |(fret_number, pitch)| {
                    widget_layout.note_label.calculate(fret_number, string_number, pitch)
                })
            })
            .for_each(move |note_label| note_label.render(renderer));
    }
}

impl<M, R> From<Fretboard> for iced::Element<'_, M, iced::Theme, R>
where
    R: iced::advanced::text::Renderer<Font = iced::Font>,
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
struct Cx {
    note_label_bounds_width: f32,
    note_label_font_size: f32,
    orientation: Orientation,
    origin: iced::Point,
    origin_fret: f32,
    origin_fret_marker_double_a: f32,
    origin_fret_marker_double_b: f32,
    origin_fret_marker_single: f32,
    origin_nut: f32,
    size_fret: iced::Size,
    size_fret_marker: iced::Size,
    size_nut: iced::Size,
    size_string: iced::Size,
    spacing_fret: f32,
    spacing_string: f32,
}

impl Cx {
    const SCALE_FRET: f32 = 0.002;
    const SCALE_FRET_MARKER: f32 = 0.07;
    const SCALE_NOTE_LABEL_BOUNDS: f32 = 2.5;
    const SCALE_NOTE_LABEL_FONT: f32 = 0.009;
    const SCALE_NUT: f32 = 0.002;
    const SCALE_STRING: f32 = 0.005;

    fn new(frets_count: u8, strings_count: usize, bounds: iced::Rectangle) -> Self {
        let origin = bounds.position();
        let max_size = bounds.size();
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
            origin.x + (width_frets - length_frets) / 2.0,
            origin.y + (width_pitches - length_pitches) / 2.0,
        );

        let nut_width = length_frets * Self::SCALE_NUT;

        let note_label_font_size = length_frets * Self::SCALE_NOTE_LABEL_FONT;
        let note_label_bounds_width = note_label_font_size * Self::SCALE_NOTE_LABEL_BOUNDS;

        let origin_nut = origin.x + note_label_bounds_width * 1.25;
        let origin_fret = origin_nut + nut_width;

        let spacing_fret = (length_frets - origin_fret) / (frets_count + 1.0);
        let spacing_string = length_pitches / (strings_count + 1.0);

        let fret_marker_width = spacing_fret * Self::SCALE_FRET_MARKER;
        let origin_fret_marker_single = origin.y + ((length_pitches / 2.0) - (fret_marker_width / 2.0));

        Self {
            note_label_bounds_width,
            note_label_font_size,
            orientation,
            origin,
            origin_fret,
            origin_fret_marker_double_a: origin_fret_marker_single - fret_marker_width,
            origin_fret_marker_double_b: origin_fret_marker_single + fret_marker_width,
            origin_fret_marker_single,
            origin_nut,
            size_nut: orientation.transform_size(iced::Size::new(nut_width, length_pitches)),
            size_fret: orientation.transform_size(iced::Size::new(length_frets * Self::SCALE_FRET, length_pitches)),
            size_fret_marker: iced::Size::new(fret_marker_width, fret_marker_width),
            size_string: orientation.transform_size(iced::Size::new(
                length_frets - origin_nut,
                length_pitches * Self::SCALE_STRING,
            )),
            spacing_fret,
            spacing_string,
        }
    }

    fn calculate_fret_position_x(&self, number: u8) -> f32 {
        self.spacing_fret * number as f32 + self.origin_fret
    }

    fn calculate_fret_marker_position_x(&self, fret_number: u8) -> f32 {
        self.calculate_fret_position_x(fret_number) - (self.spacing_fret / 2.0)
    }

    fn calculate_string_position_y(&self, number: usize) -> f32 {
        self.spacing_string * number as f32 + self.origin.y
    }
}

#[derive(Clone, Copy, Debug)]
struct Layout {
    note_label: LayoutNoteLabel,
    cx: Cx,
    palette: Palette,
}

impl Layout {
    fn new(frets_count: u8, strings_count: usize, bounds: iced::Rectangle, palette: Palette) -> Self {
        let cx = Cx::new(frets_count, strings_count, bounds);
        Self {
            cx,
            note_label: LayoutNoteLabel::new(cx, palette),
            palette,
        }
    }

    fn calculate_nut(&self) -> Bounds {
        let origin = self
            .cx
            .orientation
            .transform_point(iced::Point::new(self.cx.origin_nut, self.cx.origin.y));
        Bounds::new(iced::Rectangle::new(origin, self.cx.size_nut), self.palette.peach)
    }

    fn calculate_fret(&self, fret_number: u8) -> Bounds {
        let x = self.cx.calculate_fret_position_x(fret_number);
        let point = self
            .cx
            .orientation
            .transform_point(iced::Point::new(x, self.cx.origin.y));
        let bounds = iced::Rectangle::new(point, self.cx.size_fret);
        Bounds::new(bounds, self.palette.overlay0)
    }

    fn calculate_fret_marker(&self, fret_number: u8, marker_type: FretMarkerType) -> FretMarker {
        let x = self.cx.calculate_fret_marker_position_x(fret_number);
        let orientation = self.cx.orientation;
        let pos = |y: f32| orientation.transform_point(iced::Point::new(x, y));
        match marker_type {
            FretMarkerType::Single => FretMarker::Single(Bounds::new(
                iced::Rectangle::new(pos(self.cx.origin_fret_marker_single), self.cx.size_fret_marker),
                self.palette.text,
            )),
            FretMarkerType::Double => FretMarker::Double(
                Bounds::new(
                    iced::Rectangle::new(pos(self.cx.origin_fret_marker_double_a), self.cx.size_fret_marker),
                    self.palette.text,
                ),
                Bounds::new(
                    iced::Rectangle::new(pos(self.cx.origin_fret_marker_double_b), self.cx.size_fret_marker),
                    self.palette.text,
                ),
            ),
        }
    }

    fn calculate_string(&self, string_number: usize) -> Bounds {
        let y = self.cx.calculate_string_position_y(string_number);
        let point = self
            .cx
            .orientation
            .transform_point(iced::Point::new(self.cx.origin_nut, y));
        let bounds = iced::Rectangle::new(point, self.cx.size_string);
        Bounds::new(bounds, self.palette.lavender)
    }
}

#[derive(Clone, Copy, Debug)]
enum FretMarkerType {
    Double,
    Single,
}

impl FretMarkerType {
    const MARKUP: [Option<FretMarkerType>; 12] = [
        None,
        None,
        Some(Self::Single),
        None,
        Some(Self::Single),
        None,
        Some(Self::Single),
        None,
        Some(Self::Single),
        None,
        None,
        Some(Self::Double),
    ];
}

#[derive(Clone, Copy, Debug)]
#[allow(clippy::large_enum_variant)]
enum FretMarker {
    Single(Bounds),
    Double(Bounds, Bounds),
}

impl FretMarker {
    fn render(self, renderer: &mut impl iced::advanced::Renderer) {
        match self {
            FretMarker::Single(bounds) => bounds.render(renderer),
            FretMarker::Double(bounds_a, bounds_b) => {
                bounds_a.render(renderer);
                bounds_b.render(renderer);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct LayoutNoteLabel {
    bounds_size: iced::Size,
    clip_border: iced::Border,
    cx: Cx,
    font_size: iced::Pixels,
    padding: f32,
    palette: Palette,
}

impl LayoutNoteLabel {
    const BORDER_RADIUS: f32 = 0.5;
    const BORDER_WIDTH: f32 = 1.0;
    const FONT: iced::Font = iced::Font::MONOSPACE;
    const SCALE_PADDING: f32 = 1.25;
    const TEXT_ALIGN_H: iced::alignment::Horizontal = iced::alignment::Horizontal::Center;
    const TEXT_ALIGN_V: iced::alignment::Vertical = iced::alignment::Vertical::Center;
    const TEXT_LINE_HEIGHT: iced::advanced::text::LineHeight = iced::advanced::text::LineHeight::Relative(1.0);
    const TEXT_SHAPING: iced::advanced::text::Shaping = iced::advanced::text::Shaping::Advanced;
    const TEXT_WRAPPING: iced::advanced::text::Wrapping = iced::advanced::text::Wrapping::None;

    fn new(cx: Cx, palette: Palette) -> Self {
        let bounds_width = cx.note_label_bounds_width;
        let clip_border = iced::Border {
            color: palette.base,
            width: Self::BORDER_WIDTH,
            radius: iced::border::Radius::new(bounds_width * Self::BORDER_RADIUS),
        };
        Self {
            bounds_size: iced::Size::new(bounds_width, bounds_width),
            clip_border,
            cx,
            font_size: iced::Pixels::from(cx.note_label_font_size),
            padding: cx.note_label_font_size * Self::SCALE_PADDING,
            palette,
        }
    }

    fn calculate(&self, fret_number: u8, string_number: usize, pitch: Pitch) -> NoteLabel {
        let x = self.cx.calculate_fret_position_x(fret_number) - self.padding;
        let y = self.cx.calculate_string_position_y(string_number);
        let location = self.cx.orientation.transform_point(iced::Point::new(x, y));
        let clip_bounds = Bounds::new(
            iced::Rectangle::new(
                iced::Point::new(location.x - self.padding, location.y - self.padding),
                self.bounds_size,
            ),
            pitch.note.get_color(self.palette),
        )
        .with_border(self.clip_border);
        NoteLabel {
            clip_bounds,
            location,
            text: iced::advanced::text::Text {
                bounds: self.bounds_size,
                content: format!("{}{}", pitch.note.format_sharp(), pitch.octave),
                font: Self::FONT,
                horizontal_alignment: Self::TEXT_ALIGN_H,
                line_height: Self::TEXT_LINE_HEIGHT,
                shaping: Self::TEXT_SHAPING,
                size: self.font_size,
                vertical_alignment: Self::TEXT_ALIGN_V,
                wrapping: Self::TEXT_WRAPPING,
            },
            text_color: self.palette.crust,
        }
    }
}

#[derive(Debug)]
struct NoteLabel {
    clip_bounds: Bounds,
    location: iced::Point,
    text: iced::advanced::Text,
    text_color: iced::Color,
}

impl NoteLabel {
    fn render(self, renderer: &mut impl iced::advanced::text::Renderer<Font = iced::Font>) {
        self.clip_bounds.render(renderer);
        renderer.fill_text(self.text, self.location, self.text_color, self.clip_bounds.quad.bounds);
    }
}

#[derive(Clone, Copy, Debug)]
struct Bounds {
    quad: iced::advanced::renderer::Quad,
    background: iced::Background,
}

impl Bounds {
    fn new(bounds: iced::Rectangle, color: iced::Color) -> Self {
        Self {
            quad: iced::advanced::renderer::Quad {
                bounds,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            background: iced::Background::Color(color),
        }
    }

    fn with_border(mut self, value: iced::Border) -> Self {
        self.quad.border = value;
        self
    }

    fn render(self, renderer: &mut impl iced::advanced::renderer::Renderer) {
        renderer.fill_quad(self.quad, self.background);
    }
}
