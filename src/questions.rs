use crate::{Sysadmin, styles::highlighted_text};
use anyhow::Result;
use inquire::{Confirm, Select, Text};

pub fn question_ssh() -> Result<bool> {
    Ok(Confirm::new("Would you like to enable SSH? (optional)")
        .with_default(false)
        .with_help_message(
            format!(
                "This step would install {}",
                highlighted_text("openssh-server")
            )
            .as_str(),
        )
        .prompt()?)
}

pub fn question_ckan_version() -> Result<String> {
    let ckan_version_options: Vec<&str> = vec!["2.11.3", "2.10.8", "Other"];
    let answer_ckan_version = Select::new(
        "What CKAN version would you like to install? (optional)",
        ckan_version_options,
    )
    .with_help_message("We recommend using the latest compatible version of CKAN. Please do not choose 'Other' option unless for testing purposes as the CKAN version may not be supported and may cause a broken installation.")
    .prompt()?;
    if answer_ckan_version == "Other" {
        Ok(
            Text::new("What CKAN version would you like to install? (optional)")
                .with_default("2.11.3")
                .prompt()?,
        )
    } else {
        Ok(answer_ckan_version.to_string())
    }
}

pub fn question_sysadmin(username: String) -> Result<Sysadmin> {
    let configure_sysadmin = Confirm::new("Would you like to configure the sysadmin account for your CKAN instance?")
        .with_help_message(format!("The following values are set as defaults for the sysadmin account:\n\n- Username: {username}\n- Password: password\n- Email: {username}@localhost\n").as_str())
        .prompt()?;
    if configure_sysadmin {
        let username = Text::new("What should your sysadmin username be set to?")
            .with_default(username.clone().as_str())
            .prompt()?;
        let password = Text::new("What should your sysadmin password be set to?")
            .with_default("password")
            .with_help_message("The password must be at least 8 characters long")
            .prompt()?;
        let email = Text::new("What should your sysadmin email be set to?")
            .with_default(format!("{username}@localhost").as_str())
            .prompt()?;
        Ok(Sysadmin {
            username,
            password,
            email,
        })
    } else {
        Ok(Sysadmin {
            username: username.clone(),
            password: "password".to_string(),
            email: format!("{username}@localhost"),
        })
    }
}
