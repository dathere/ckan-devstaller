mod questions;
mod steps;
mod styles;

use crate::{
    questions::{question_ckan_version, question_ssh, question_sysadmin},
    steps::{
        step_install_ahoy, step_install_and_run_ckan_compose,
        step_install_ckanext_scheming_extension, step_install_curl,
        step_install_datapusher_plus_extension, step_install_datastore_extension,
        step_install_docker, step_install_openssh, step_package_updates,
    },
    styles::{important_text, step_text, success_text},
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use human_panic::{metadata, setup_panic};
use inquire::Confirm;
use std::{path::PathBuf, str::FromStr};
use xshell::cmd;
use xshell_venv::{Shell, VirtualEnv};

/// CLI to help install a CKAN instance for development within minutes. Learn more at: https://ckan-devstaller.dathere.com
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Skip interactive steps
    #[arg(short, long)]
    skip_interactive: bool,
    /// Skip running CKAN at the end of installation
    #[arg(short, long)]
    skip_run: bool,
    #[arg(short, long)]
    /// CKAN version to install defined by semantic versioning from official releases from https://github.com/ckan/ckan
    ckan_version: Option<String>,
    /// List of CKAN extensions to install, separated by spaces
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    extensions: Option<Vec<String>>,
    /// List of custom features, separated by spaces
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    features: Option<Vec<String>>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Attempt to uninstall CKAN and related ckan-devstaller installation files
    Uninstall {},
}

#[derive(Clone)]
struct Sysadmin {
    username: String,
    password: String,
    email: String,
}

struct Config {
    ssh: bool,
    ckan_version: String,
    sysadmin: Sysadmin,
    extension_datastore: bool,
    extension_ckanext_scheming: bool,
    extension_datapusher_plus: bool,
    druf_mode: bool,
}

fn main() -> Result<()> {
    setup_panic!(metadata!()
        .homepage("https://dathere.com")
        .support("- Create a support ticket at https://support.dathere.com or report an issue at https://github.com/dathere/ckan-devstaller"));

    // Set up default config
    let args = Args::parse();
    let sh = Shell::new()?;
    let username = cmd!(sh, "whoami").read()?;

    if matches!(&args.command, Some(Commands::Uninstall {})) {
        let uninstall_confirmation = Confirm::new(
            "Are you sure you want to uninstall CKAN and related files from ckan-devstaller?",
        )
        .with_help_message(
            r#"The following commands are ran when attempting the uninstall:
sudo rm -rf /usr/lib/ckan
sudo rm -rf /etc/ckan
cd ~/
rm -rf qsv*
rm -rf README ckan-compose ahoy dpp_default_config.ini get-docker.sh permissions.sql"#,
        )
        .prompt()?;
        if uninstall_confirmation {
            cmd!(sh, "sudo rm -rf /usr/lib/ckan").run()?;
            cmd!(sh, "sudo rm -rf /etc/ckan").run()?;
            sh.change_dir(format!("/home/{username}"));
            cmd!(sh, "rm -rf qsv*").run()?;
            cmd!(sh, "rm -rf README ckan-compose ahoy dpp_default_config.ini get-docker.sh permissions.sql").run()?;
        } else {
            println!("Cancelling command.");
        }
        return Ok(());
    }

    let default_sysadmin = Sysadmin {
        username: username.clone(),
        password: "password".to_string(),
        email: format!("{username}@localhost"),
    };
    let config = Config {
        ssh: args
            .features
            .is_some_and(|features| features.contains(&"enable-ssh".to_string())),
        ckan_version: if args.ckan_version.is_some() {
            args.ckan_version.unwrap()
        } else {
            "2.11.4".to_string()
        },
        sysadmin: default_sysadmin.clone(),
        extension_datastore: args
            .extensions
            .clone()
            .is_some_and(|extensions| extensions.contains(&"DataStore".to_string())),
        extension_ckanext_scheming: args
            .extensions
            .clone()
            .is_some_and(|extensions| extensions.contains(&"ckanext-scheming".to_string())),
        extension_datapusher_plus: args
            .extensions
            .is_some_and(|extensions| extensions.contains(&"DataPusher+".to_string())),
        druf_mode: false,
    };

    steps::step_intro();

    let mut default_config_text =
        String::from("The current configuration for ckan-devstaller does the following:");
    if config.ssh {
        default_config_text.push_str("\n- Install openssh-server to enable SSH access");
    }
    default_config_text.push_str("\n- Install ckan-compose (https://github.com/tino097/ckan-compose/tree/ckan-devstaller) which sets up the CKAN backend (PostgreSQL, SOLR, Redis)");
    default_config_text.push_str(format!("\n- Install CKAN v{}", config.ckan_version).as_str());
    if config.extension_datastore {
        default_config_text.push_str("\n- Install the DataStore extension");
    }
    if config.extension_ckanext_scheming {
        default_config_text.push_str("\n- Install the ckanext-scheming extension");
    }
    if config.extension_datapusher_plus {
        default_config_text.push_str("\n- Install the DataPusher+ extension");
        default_config_text.push_str("\n- Disable DRUF mode for DataPusher+");
    }
    println!("{default_config_text}");
    let answer_customize = if args.skip_interactive {
        false
    } else {
        Confirm::new("Would you like to customize the configuration for your CKAN installation?")
            .prompt()?
    };
    let config = if answer_customize {
        let answer_ssh = question_ssh()?;
        let answer_ckan_version = question_ckan_version()?;
        let answer_sysadmin = question_sysadmin(username.clone())?;
        // let answer_extension_datastore = Confirm::new("Would you like to install the DataStore extension?")
        //     .with_default(true)
        //     .prompt()?;
        // let answer_extension_ckanext_scheming = Confirm::new("Would you like to install the ckanext-scheming extension?")
        //     .with_default(true)
        //     .prompt()?;
        let answer_extension_datapusher_plus =
            Confirm::new("Would you like to install the DataPusher+ extension?")
                .with_default(true)
                .prompt()?;
        let answer_druf_mode = if answer_extension_datapusher_plus {
            Confirm::new("Would you like to enable DRUF mode for DataPusher+?")
                .with_default(false)
                .prompt()?
        } else {
            false
        };
        Config {
            ssh: answer_ssh,
            ckan_version: answer_ckan_version,
            sysadmin: answer_sysadmin,
            extension_datastore: true,
            extension_ckanext_scheming: true,
            extension_datapusher_plus: answer_extension_datapusher_plus,
            druf_mode: answer_druf_mode,
        }
    } else {
        config
    };

    let begin_installation = if args.skip_interactive {
        true
    } else {
        Confirm::new("Would you like to begin the installation?").prompt()?
    };

    if begin_installation {
        println!("\n{}", important_text("Starting installation..."));
        // Run sudo apt update and sudo apt upgrade
        step_package_updates("1.".to_string(), &sh)?;

        // Install curl
        step_install_curl("2.".to_string(), &sh)?;
        // If user wants SSH capability, install openssh-server
        if config.ssh {
            step_install_openssh("2.".to_string(), &sh)?;
        }

        // Install docker CLI if user does not have it installed
        step_install_docker("3.".to_string(), &sh, username.clone())?;

        step_install_ahoy("4.".to_string(), &sh, username.clone())?;

        step_install_and_run_ckan_compose("5.".to_string(), &sh, username.clone())?;

        println!(
            "\n{} Installing CKAN {}...",
            step_text("6."),
            config.ckan_version
        );
        cmd!(sh, "sudo apt install python3-dev libpq-dev python3-pip python3-venv git-core redis-server -y").run()?;
        cmd!(sh, "sudo mkdir -p /usr/lib/ckan/default").run()?;
        cmd!(sh, "sudo chown {username} /usr/lib/ckan/default").run()?;
        let venv_path = PathBuf::from_str("/usr/lib/ckan/default")?;
        let venv = VirtualEnv::with_path(&sh, &venv_path)?;
        venv.pip_upgrade("pip")?;
        venv.pip_install(
            format!(
                "git+https://github.com/ckan/ckan.git@ckan-{}#egg=ckan[requirements]",
                config.ckan_version
            )
            .as_str(),
        )?;
        cmd!(sh, "sudo mkdir -p /etc/ckan/default").run()?;
        cmd!(sh, "sudo chown -R {username} /etc/ckan/").run()?;
        cmd!(
            sh,
            "git clone https://github.com/ckan/ckan.git /usr/lib/ckan/default/src/ckan"
        )
        .run()?;
        sh.change_dir("/usr/lib/ckan/default/src/ckan");
        cmd!(sh, "ckan generate config /etc/ckan/default/ckan.ini").run()?;
        cmd!(
            sh,
            "ln -s /usr/lib/ckan/default/src/ckan/who.ini /etc/ckan/default/who.ini"
        )
        .run()?;
        sh.change_dir("/usr/lib/ckan/default/src/ckan");
        venv.pip_install("flask-debugtoolbar==0.14.1")?;
        sh.change_dir("/var/lib");
        cmd!(sh, "sudo mkdir -p ckan/default").run()?;
        cmd!(sh, "sudo chown {username}.{username} ckan/default").run()?;
        cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini db init").run()?;
        let sysadmin_username = config.sysadmin.username;
        let sysadmin_password = config.sysadmin.password;
        let sysadmin_email = config.sysadmin.email;
        cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini user add {sysadmin_username} password={sysadmin_password} email={sysadmin_email}").run()?;
        cmd!(
            sh,
            "ckan -c /etc/ckan/default/ckan.ini sysadmin add {sysadmin_username}"
        )
        .run()?;
        println!(
            "{}",
            success_text(format!("6. Installed CKAN {}.", config.ckan_version).as_str())
        );

        // Install extensions
        if config.extension_datastore {
            step_install_datastore_extension("7.".to_string(), &sh, username.clone())?;
        }
        if config.extension_ckanext_scheming {
            step_install_ckanext_scheming_extension("8.".to_string(), &sh, username.clone())?;
        }
        if config.extension_datapusher_plus {
            step_install_datapusher_plus_extension(
                "9.".to_string(),
                &sh,
                sysadmin_username,
                username.clone(),
            )?;
        }

        println!("\n{}", success_text("Running CKAN instance..."));
        if !args.skip_run {
            cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini run").run()?;
        }
    } else {
        println!("Cancelling installation.");
    }

    Ok(())
}
