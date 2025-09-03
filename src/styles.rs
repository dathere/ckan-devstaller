use owo_colors::{OwoColorize, Stream::Stdout};

pub fn highlighted_text(text: &str) -> impl std::fmt::Display {
    format!(
        "{}",
        text.if_supports_color(Stdout, |t| t.on_blue().white())
    )
}

pub fn important_text(text: &str) -> impl std::fmt::Display {
    format!(
        "{}",
        text.if_supports_color(Stdout, |t| t.on_bright_red().white())
    )
}

pub fn step_text(text: &str) -> impl std::fmt::Display {
    format!(
        "{}",
        text.if_supports_color(Stdout, |t| t.on_magenta().white())
    )
}

pub fn success_text(text: &str) -> impl std::fmt::Display {
    format!(
        "{}",
        text.if_supports_color(Stdout, |t| t.on_green().white())
    )
}
