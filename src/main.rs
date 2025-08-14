use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use inquire::Confirm;
use owo_colors::{OwoColorize, Stream::Stdout};
use xshell::cmd;
use xshell_venv::{Shell, VirtualEnv};

fn main() -> Result<()> {
    println!("Welcome to the ckan-devstaller!");
    println!(
        "ckan-devstaller is provided by datHere - {}\n",
        "https://datHere.com".if_supports_color(Stdout, |text| text.on_blue().white()),
    );
    println!(
        "This installer should assist in setting up {} from a source installation along with ckan-compose (https://github.com/tino097/ckan-compose).",
        "CKAN 2.11.3".if_supports_color(Stdout, |text| text.on_blue().white())
    );
    println!(
        "This installer is also only intended for a brand new installation of {}.\n",
        "Ubuntu 22.04".if_supports_color(Stdout, |text| text.on_blue().white())
    );
    let ans = Confirm::new("Would you like to begin the installation?")
        .with_default(false)
        .prompt()?;

    if ans {
        let sh = Shell::new()?;
        println!(
            "\n{} Running {} and {}...",
            "1.".if_supports_color(Stdout, |text| text.on_magenta().white()),
            "sudo apt update -y".if_supports_color(Stdout, |text| text.on_blue().white()),
            "sudo apt upgrade -y".if_supports_color(Stdout, |text| text.on_blue().white())
        );
        println!(
            "{}",
            "You may need to provide your sudo password."
                .if_supports_color(Stdout, |text| text.on_bright_red().white())
        );
        cmd!(sh, "sudo apt update -y").run()?;
        cmd!(sh, "sudo apt upgrade -y").run()?;
        println!(
            "{}",
            "✅ 1. Successfully ran update and upgrade commands."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );

        println!(
            "\n{} Enabling SSH...",
            "2.".if_supports_color(Stdout, |text| text.on_magenta().white()),
        );
        cmd!(sh, "sudo apt install openssh-server -y").run()?;
        println!(
            "{}",
            "✅ 2. Successfully enabled SSH."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );
        let username = cmd!(sh, "whoami").read()?;

        let dpkg_l_output = cmd!(sh, "dpkg -l").read()?;
        let has_docker = cmd!(sh, "grep docker")
            .stdin(dpkg_l_output.clone())
            .ignore_status()
            .output()?
            .status
            .success();
        if !has_docker {
            println!(
                "{} Installing Docker...",
                "3.".if_supports_color(Stdout, |text| text.on_magenta().white()),
            );
            cmd!(
                sh,
                "curl -fsSL https://get.docker.com -o /home/{username}/get-docker.sh"
            )
            .run()?;
            cmd!(sh, "sudo sh /home/{username}/get-docker.sh").run()?;
            println!(
                "{}",
                "✅ 3. Successfully installed Docker."
                    .if_supports_color(Stdout, |text| text.on_green().white())
            );
        }

        let has_docker_compose = cmd!(sh, "grep docker-compose")
            .stdin(dpkg_l_output)
            .ignore_status()
            .output()?
            .status
            .success();
        if !has_docker_compose {
            cmd!(sh, "sudo apt install docker-compose -y").run()?;
        }

        println!(
            "\n{} Installing Ahoy...",
            "4.".if_supports_color(Stdout, |text| text.on_magenta().white()),
        );
        cmd!(sh, "sudo curl -LO https://github.com/ahoy-cli/ahoy/releases/download/v2.5.0/ahoy-bin-linux-amd64").run()?;
        cmd!(sh, "mv ./ahoy-bin-linux-amd64 ./ahoy").run()?;
        cmd!(sh, "sudo chmod +x ./ahoy").run()?;
        println!(
            "{}",
            "✅ 4. Successfully installed Ahoy."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );

        println!(
            "\n{} Downloading, installing, and starting ckan-compose...",
            "5.".if_supports_color(Stdout, |text| text.on_magenta().white()),
        );
        println!("{}", "You may need to provide an arbitrary name then press ENTER to set defaults for the rest.".if_supports_color(Stdout, |text| text.on_bright_red().white()));
        sh.change_dir(format!("/home/{username}"));
        if !std::fs::exists(format!("/home/{username}/ckan-compose"))? {
            cmd!(sh, "git clone https://github.com/tino097/ckan-compose.git").run()?;
        }
        sh.change_dir(format!("/home/{username}/ckan-compose"));
        cmd!(sh, "git switch solr-9-impl").run()?;
        let env_data = "PROJECT_NAME=ckan-devstaller-project
DATASTORE_READONLY_PASSWORD=pass
POSTGRES_PASSWORD=pass";
        std::fs::write(format!("/home/{username}/ckan-compose/.env"), env_data)?;
        cmd!(sh, "sudo ../ahoy up").run()?;
        println!(
            "{}",
            "✅ 5. Successfully ran ckan-compose."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );

        println!(
            "\n{} Installing CKAN 2.11.3...",
            "6.".if_supports_color(Stdout, |text| text.on_magenta().white()),
        );
        cmd!(sh, "sudo apt install python3-dev libpq-dev python3-pip python3-venv git-core redis-server -y").run()?;
        cmd!(sh, "sudo mkdir -p /usr/lib/ckan/default").run()?;
        cmd!(sh, "sudo chown {username} /usr/lib/ckan/default").run()?;
        let venv_path = PathBuf::from_str("/usr/lib/ckan/default")?;
        let venv = VirtualEnv::with_path(&sh, &venv_path)?;
        venv.pip_upgrade("pip")?;
        venv.pip_install(
            "git+https://github.com/ckan/ckan.git@ckan-2.11.3#egg=ckan[requirements]",
        )?;
        cmd!(sh, "sudo mkdir -p /etc/ckan/default").run()?;
        cmd!(sh, "sudo chown -R {username} /etc/ckan/").run()?;
        cmd!(
            sh,
            "git clone https://github.com/ckan/ckan.git /usr/lib/ckan/default/src"
        )
        .run()?;
        sh.change_dir("/usr/lib/ckan/default/src");
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
        cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini user add {username} password=password email={username}@localhost").run()?;
        cmd!(
            sh,
            "ckan -c /etc/ckan/default/ckan.ini sysadmin add {username}"
        )
        .run()?;
        cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini run").run()?;
        println!(
            "{}",
            "✅ 6. Installed CKAN 2.11.3 and started running instance."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );
    }

    Ok(())
}

struct Config {
    ssh: bool,
}

fn get_config_from_prompts() -> Result<Config> {
    let ssh = Confirm::new("Would you like to enable SSH? (optional)")
        .with_default(false)
        .with_help_message(
            format!(
                "This step would install {}",
                "openssh-server".if_supports_color(Stdout, |text| text.on_blue().white())
            )
            .as_str(),
        )
        .prompt()?;
    Ok(Config { ssh })
}
