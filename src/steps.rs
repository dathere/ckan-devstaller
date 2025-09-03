use crate::styles::{highlighted_text, important_text};

pub fn step_intro() {
    println!("Welcome to the ckan-devstaller!");
    println!(
        "ckan-devstaller is provided by datHere - {}\n",
        highlighted_text("https://datHere.com"),
    );
    println!(
        "This installer should assist in setting up {} from a source installation along with ckan-compose. If you have any issues, please report them at https://support.dathere.com or https://github.com/dathere/ckan-devstaller/issues.",
        highlighted_text("CKAN 2.11.3")
    );
    println!(
        "{}",
        important_text(
            "This installer is only intended for a brand new installation of Ubuntu 22.04."
        )
    );
}
