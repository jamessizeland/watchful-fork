#![no_std]

use core::fmt::Write as _;

use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565 as Rgb;
use embedded_graphics::prelude::{DrawTarget, *};
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Text, TextStyleBuilder};
use embedded_iconoir::prelude::*;
use embedded_layout::layout::linear::{spacing, LinearLayout};
use embedded_layout::prelude::*;
use embedded_text::style::TextBoxStyleBuilder;
use embedded_text::TextBox;
use u8g2_fonts::{fonts, U8g2TextStyle};
use time::PrimitiveDateTime;

const WIDTH: u32 = 240;
const HEIGHT: u32 = 240;
const GRID_ITEMS: u32 = 4;

fn watch_text_style(color: Rgb) -> U8g2TextStyle<Rgb> {
    //U8g2TextStyle::new(fonts::u8g2_font_unifont_t_symbols, Rgb::YELLOW)
    //U8g2TextStyle::new(fonts::u8g2_font_inb57_mn, color)
    U8g2TextStyle::new(fonts::u8g2_font_logisoso58_tn, color) //u8g2_font_logisoso78_tn, color) //u8g2_font_logisoso92_tn, color) //u8g2_font_fub49_tn, color) //u8g2_font_spleen16x32_mf, color)
}

fn menu_text_style(color: Rgb) -> U8g2TextStyle<Rgb> {
    //U8g2TextStyle::new(fonts::u8g2_font_unifont_t_symbols, Rgb::YELLOW)
    U8g2TextStyle::new(fonts::u8g2_font_spleen16x32_mf, color)
}

fn date_text_style(color: Rgb) -> U8g2TextStyle<Rgb> {
    U8g2TextStyle::new(fonts::u8g2_font_spleen12x24_mf, color)
}

fn text_text_style(color: Rgb) -> U8g2TextStyle<Rgb> {
    U8g2TextStyle::new(fonts::u8g2_font_unifont_t_symbols, color)
}

pub enum ButtonEvent {
    ShortPress,
    LongPress,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InputEvent {
    Touch(TouchGesture),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TouchGesture {
    SingleTap(Point),
    DoubleTap(Point),
    SwipeUp(Point),
    SwipeDown(Point),
    SwipeLeft(Point),
    SwipeRight(Point),
}

#[derive(PartialEq)]
pub struct TimeView {
    pub time: time::PrimitiveDateTime,
    pub battery_level: u32,
    pub battery_charging: bool,
}

impl TimeView {
    pub fn new(time: time::PrimitiveDateTime, battery_level: u32, battery_charging: bool) -> Self {
        Self {
            time,
            battery_level,
            battery_charging,
        }
    }
    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        display.clear(Rgb::BLACK)?;

        let mut buf: heapless::String<16> = heapless::String::new();
        write!(buf, "{:02}:{:02}", self.time.hour(), self.time.minute()).unwrap();
        let hm = Text::with_text_style(
            &buf,
            display.bounding_box().center(),
            watch_text_style(Rgb::CSS_DARK_CYAN),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .baseline(embedded_graphics::text::Baseline::Alphabetic)
                .build(),
        );

        let mut buf: heapless::String<16> = heapless::String::new();
        write!(buf, "{}", self.time.weekday()).unwrap();
        buf.truncate(3);
        write!(buf, " {}", self.time.day()).unwrap();
        let date = Text::with_text_style(
            &buf,
            display.bounding_box().center(),
            date_text_style(Rgb::CSS_DARK_CYAN),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .baseline(embedded_graphics::text::Baseline::Alphabetic)
                .build(),
        );

        let display_area = display.bounding_box();
        LinearLayout::vertical(Chain::new(date).append(hm))
            .with_spacing(spacing::FixedMargin(10))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .draw(display)?;

        let display_area = display_area.offset(-5);
        let top_right_y = display_area.top_left.y;
        let top_right_x = display_area.top_left.x + display_area.size.width as i32 - 30;
        let pos = Point::new(top_right_x, top_right_y);
        if self.battery_charging {
            Image::new(&icons::size24px::system::BatteryCharging::new(Rgb::CSS_DARK_CYAN), pos).draw(display)?
        } else {
            if self.battery_level > 85 {
                Image::new(&icons::size24px::system::BatteryFull::new(Rgb::CSS_DARK_CYAN), pos).draw(display)?
            } else if self.battery_level > 65 {
                Image::new(&icons::size24px::system::BatterySevenFive::new(Rgb::CSS_DARK_CYAN), pos).draw(display)?
            } else if self.battery_level > 35 {
                Image::new(&icons::size24px::system::BatteryFiveZero::new(Rgb::CSS_DARK_CYAN), pos).draw(display)?
            } else if self.battery_level > 10 {
                Image::new(&icons::size24px::system::BatteryTwoFive::new(Rgb::CSS_DARK_CYAN), pos).draw(display)?
            } else {
                Image::new(&icons::size24px::system::BatteryEmpty::new(Rgb::CSS_DARK_CYAN), pos).draw(display)?
            }
        };

        Ok(())
    }
}

pub struct WorkoutView {
    hr: u32,
    duration: time::Duration,
}

impl WorkoutView {
    pub fn new(hr: u32, duration: time::Duration) -> Self {
        Self { hr, duration }
    }
    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        display.clear(Rgb::BLACK)?;

        let mut buf: heapless::String<16> = heapless::String::new();
        write!(buf, "{:03}", self.hr).unwrap();
        let hr = Text::with_text_style(
            &buf,
            display.bounding_box().center(),
            watch_text_style(Rgb::CSS_DARK_CYAN),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .baseline(embedded_graphics::text::Baseline::Alphabetic)
                .build(),
        );

        let mut buf: heapless::String<16> = heapless::String::new();
        write!(
            buf,
            "{:03}:{:02}",
            self.duration.whole_minutes(),
            self.duration.whole_seconds()
        )
        .unwrap();
        let secs = Text::with_text_style(
            &buf,
            display.bounding_box().center(),
            date_text_style(Rgb::CSS_DARK_CYAN),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .baseline(embedded_graphics::text::Baseline::Alphabetic)
                .build(),
        );

        let display_area = display.bounding_box();
        LinearLayout::vertical(Chain::new(hr).append(secs))
            .with_spacing(spacing::FixedMargin(10))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .draw(display)?;

        Ok(())
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct TimerView {
    pub remaining: time::Duration,
    pub running: bool,
}

impl TimerView {
    pub fn new(remaining: time::Duration, running: bool) -> Self {
        Self { remaining, running }
    }
    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        display.clear(Rgb::BLACK)?;
        
        let mut buf: heapless::String<16> = heapless::String::new();
        write!(
            buf,
            //"{:02}:{:02}:{:02}",
            "{:02}:{:02}",
            //self.remaining.whole_hours(),
            self.remaining.whole_minutes(),
            self.remaining.whole_seconds() % 60,
            //self.remaining.subsec_milliseconds() / 100
        )
        .unwrap();

		let cd = Text::with_text_style(
            &buf,
            display.bounding_box().center(),
            watch_text_style(Rgb::CSS_DARK_CYAN),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .baseline(embedded_graphics::text::Baseline::Alphabetic)
                .build(),
        );

        let display_area = display.bounding_box();
        LinearLayout::vertical(Chain::new(cd))
            .with_spacing(spacing::FixedMargin(10))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .draw(display)?;

		Ok(())
	}
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MenuAction {
    Workout,
    Timer,
    FindPhone,
    Settings,
    FirmwareSettings,
    ValidateFirmware,
    Brightness,
    TimeSettings,
    ChangeTimeMinInc,
    ChangeTimeHourInc,
    ChangeTimeMinDec,
    ChangeTimeHourDec,
    Reset,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuView {
    Main {
        workout: MenuItem,
        timer: MenuItem,
        find_phone: MenuItem,
        settings: MenuItem,
    },
    Settings {
        firmware: MenuItem,
        brightness: MenuItem,
        time_settings: MenuItem,
        reset: MenuItem,
    },
    Firmware {
        details: FirmwareDetails,
        item: MenuItem,
    },
    TimeSettings {
    	details: TimeDetails,
        min_inc: MenuControl,
        hour_inc: MenuControl,
        min_dec: MenuControl,
        hour_dec: MenuControl,
    },
}

impl MenuView {
    pub fn main() -> Self {
        Self::Main {
            workout: MenuItem::new("Workout", 0),
            timer: MenuItem::new("Timer", 1),
            find_phone: MenuItem::new("Find Phone", 2),
            settings: MenuItem::new("Settings", 3),
        }
    }

    pub fn settings() -> Self {
        Self::Settings {
            firmware: MenuItem::new("Firmware", 0),
            brightness: MenuItem::new("Brightness", 1),
            time_settings: MenuItem::new("Time", 2),
            reset: MenuItem::new("Reset", 3),
        }
    }

    pub fn firmware_settings(details: FirmwareDetails) -> Self {
        let valid = details.validated;
        Self::Firmware {
            details,
            item: MenuItem::new(if valid { "Validated" } else { "Validate" }, 3),
        }
    }

    pub fn time_settings(details: TimeDetails) -> Self {
        Self::TimeSettings {
            details,
            hour_inc: MenuControl::new("+", 0),
            min_inc: MenuControl::new("+", 1),
            hour_dec: MenuControl::new("-", 2),
            min_dec: MenuControl::new("-", 3),
        }
    }
    
    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        display.clear(Rgb::BLACK)?;

        match self {
            Self::Main {
                workout,
                timer,
                find_phone,
                settings,
            } => {
                workout.draw(display)?;
                timer.draw(display)?;
                find_phone.draw(display)?;
                settings.draw(display)?;
            }

            Self::Settings { firmware, brightness, time_settings, reset } => {
                firmware.draw(display)?;
                brightness.draw(display)?;
                time_settings.draw(display)?;
                reset.draw(display)?;
            }

            Self::Firmware { details, item } => {
                details.draw(display)?;
                item.draw(display)?;
            }

			Self::TimeSettings { details, min_inc, hour_inc, min_dec, hour_dec } => {
                details.draw(display)?;
                min_inc.draw(display)?;
                hour_inc.draw(display)?;
                min_dec.draw(display)?;
                hour_dec.draw(display)?;
            }
        }

        Ok(())
    }

    pub fn on_event(&self, input: InputEvent) -> Option<MenuAction> {
        match self {
            Self::Main {
                workout,
                timer,
                find_phone,
                settings,
            } => {
                if workout.is_clicked(input) {
                    Some(MenuAction::Workout)
                } else if timer.is_clicked(input) {
                    Some(MenuAction::Timer)
                } else if find_phone.is_clicked(input) {
                    Some(MenuAction::FindPhone)
                } else if settings.is_clicked(input) {
                    Some(MenuAction::Settings)
                } else {
                    None
                }
            }
            Self::Settings { firmware, brightness, time_settings, reset } => {
                if firmware.is_clicked(input) {
                    Some(MenuAction::FirmwareSettings)
                } else if brightness.is_clicked(input) {
                    Some(MenuAction::Brightness)
                } else if time_settings.is_clicked(input) {
                	Some(MenuAction::TimeSettings)
                } else if reset.is_clicked(input) {
                    Some(MenuAction::Reset)
                } else {
                    None
                }
            }
            Self::Firmware { details: _, item } => {
                if item.is_clicked(input) {
                    Some(MenuAction::ValidateFirmware)
                } else {
                    None
                }
            }
            Self::TimeSettings { details: _, min_inc, hour_inc, min_dec, hour_dec } => {
                if min_inc.is_clicked(input) {
                    Some(MenuAction::ChangeTimeMinInc)
                } else if hour_inc.is_clicked(input) {
					Some(MenuAction::ChangeTimeHourInc)
				} else if min_dec.is_clicked(input) {
                    Some(MenuAction::ChangeTimeMinDec)
                } else if hour_dec.is_clicked(input) {
					Some(MenuAction::ChangeTimeHourDec)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct MenuItem {
    text: &'static str,
    idx: u32,
}

impl MenuItem {
    pub fn new(text: &'static str, idx: u32) -> Self {
        Self { text, idx }
    }

    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        let line_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb::CSS_DARK_CYAN)
            .stroke_width(1)
            .fill_color(Rgb::CSS_DARK_CYAN)
            .build();
        let (start, end) = self.placement();
        Rectangle::with_corners(start, end)
            .into_styled(line_style)
            .draw(display)?;

        Text::with_text_style(
            self.text,
            Point::new(
                (WIDTH as i32) / 2,
                self.idx as i32 * (HEIGHT as i32 / GRID_ITEMS as i32) + 40,
            ),
            menu_text_style(Rgb::CSS_CORNSILK),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .build(),
        )
        .draw(display)?;
        Ok(())
    }

    fn placement(&self) -> (Point, Point) {
        let start = Point::new(10, (self.idx * HEIGHT / GRID_ITEMS + 10) as i32);
        let end = Point::new(
            start.x + WIDTH as i32 - 20,
            start.y + (HEIGHT as i32 / GRID_ITEMS as i32) - 20,
        );
        (start, end)
    }

    // Check if point is within our range
    pub fn is_clicked(&self, event: InputEvent) -> bool {
        if let InputEvent::Touch(TouchGesture::SingleTap(pos)) = event {
            let (c1, c2) = self.placement();
            c1.x <= pos.x && c1.y <= pos.y && c2.x >= pos.x && c2.y >= pos.y
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct MenuControl {
    text: &'static str,
    idx: u32,
}

impl MenuControl {
    pub fn new(text: &'static str, idx: u32) -> Self {
        Self { text, idx }
    }

    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        let line_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb::CSS_DARK_CYAN)
            .stroke_width(1)
            .fill_color(Rgb::CSS_DARK_CYAN)
            .build();
        let (start, end) = self.placement();
        Rectangle::with_corners(start, end)
            .into_styled(line_style)
            .draw(display)?;

        Text::with_text_style(
            self.text,
			Point::new(
                (start.x + end.x) / 2,
                (start.y + end.y) / 2 + 8,
            ),
            menu_text_style(Rgb::CSS_CORNSILK),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .build(),
        )
        .draw(display)?;
        Ok(())
    }

    fn placement(&self) -> (Point, Point) {
    	match self.idx {
    		0 => {
    			let start = Point::new(20, 20);
    			let end = Point::new(
    				WIDTH as i32 / 2 - 10,
    				start.y + HEIGHT as i32 / 4 - 10
    			);
    			(start, end)
    		}
    		1 => {
    			let start = Point::new(WIDTH as i32 / 2 + 10, 20);
    			let end = Point::new(
    				WIDTH as i32 - 20,
    				start.y + HEIGHT as i32 / 4 - 10
    			);
    			(start, end)
    		}
    		2 => {
    			let start = Point::new(
    				20,
    				HEIGHT as i32 / 2 + 50
    			);
    			let end = Point::new(
    				WIDTH as i32 / 2 - 10,
    				start.y + HEIGHT as i32 / 4 - 10
    			);
    			(start, end)
    		}
    		3 => {
    			let start = Point::new(
    				WIDTH as i32 / 2 + 10,
    				HEIGHT as i32 / 2 + 50
    			);
    			let end = Point::new(
    				WIDTH as i32 - 20,
    				start.y + HEIGHT as i32 / 4 - 10
    			);
    			(start, end)
    		}
    		_ => (Point::new(0, 0), Point::new(0, 0))
        }
    }

    // Check if point is within our range
    pub fn is_clicked(&self, event: InputEvent) -> bool {
        if let InputEvent::Touch(TouchGesture::SingleTap(pos)) = event {
            let (c1, c2) = self.placement();
            c1.x <= pos.x && c1.y <= pos.y && c2.x >= pos.x && c2.y >= pos.y
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct FirmwareDetails {
    name: &'static str,
    version: &'static str,
    commit: &'static str,
    build_timestamp: &'static str,
    battery_level: u32,
    battery_charging: bool,
    validated: bool,
}

impl FirmwareDetails {
    pub const fn new(
        name: &'static str,
        version: &'static str,
        commit: &'static str,
        build_timestamp: &'static str,
        battery_level: u32,
        battery_charging: bool,
        validated: bool,
    ) -> Self {
        Self {
            name,
            version,
            commit,
            build_timestamp,
            battery_level,
            battery_charging,
            validated,
        }
    }

    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        let start = Point::new(0, 0);
        let end = Size::new(WIDTH as u32, 2 * (HEIGHT as u32 / GRID_ITEMS as u32) - 20);

        let bounds = Rectangle::new(start, end);

        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(embedded_text::style::HeightMode::FitToText)
            .alignment(embedded_text::alignment::HorizontalAlignment::Justified)
            .paragraph_spacing(6)
            .build();

        let character_style = text_text_style(Rgb::CSS_LIGHT_CORAL);

        let mut info: heapless::String<512> = heapless::String::new();
        write!(
            info,
            "Name: {}\nVersion: {}\nCommit: {}\nBuild: {}\nBattery: {}{}",
            self.name,
            self.version,
            self.commit,
            self.build_timestamp,
            self.battery_level,
            if self.battery_charging { "(Charging)" } else { "" }
        )
        .unwrap();

        TextBox::with_textbox_style(&info, bounds, character_style, textbox_style).draw(display)?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct TimeDetails {
    time: PrimitiveDateTime,
}

impl TimeDetails {
    pub const fn new(
        time: PrimitiveDateTime,
    ) -> Self {
        Self {
            time,
        }
    }

    pub fn draw<D: DrawTarget<Color = Rgb>>(&self, display: &mut D) -> Result<(), D::Error> {
        let mut buf: heapless::String<16> = heapless::String::new();
        write!(
            buf,
            "{:02}:{:02}",
            self.time.hour(),
            self.time.minute(),
        )
        .unwrap();

		let hm = Text::with_text_style(
            &buf,
            display.bounding_box().center(),
            watch_text_style(Rgb::CSS_DARK_CYAN),
            TextStyleBuilder::new()
                .alignment(embedded_graphics::text::Alignment::Center)
                .baseline(embedded_graphics::text::Baseline::Alphabetic)
                .build(),
        );

        let display_area = display.bounding_box();
        LinearLayout::vertical(Chain::new(hm))
            .with_spacing(spacing::FixedMargin(10))
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .draw(display)?;

		Ok(())
    }
}