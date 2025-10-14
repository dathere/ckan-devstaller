use crate::styles::{highlighted_text, important_text, step_text, success_text};
use anyhow::Result;
use xshell::{Shell, cmd};

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
        "\nYou may also learn more about ckan-devstaller at https://ckan-devstaller.dathere.com."
    );
    println!(
        "\n{}\n",
        important_text(
            "This installer is only intended for a brand new installation of Ubuntu 22.04."
        )
    );
}

pub fn step_package_updates(step_prefix: String, sh: &Shell) -> Result<()> {
    println!(
        "\n{} Running {} and {}...",
        step_text(step_prefix.as_str()),
        highlighted_text("sudo apt update -y"),
        highlighted_text("sudo apt upgrade -y")
    );
    println!(
        "{}",
        important_text("You may need to provide your sudo password.")
    );
    cmd!(sh, "sudo apt update -y").run()?;
    // Ignoring xrdp error with .ignore_status() for now
    cmd!(sh, "sudo apt upgrade -y").ignore_status().run()?;
    println!(
        "{}",
        success_text(
            format!("{step_prefix} Successfully ran update and upgrade commands.").as_str()
        )
    );
    Ok(())
}

pub fn step_install_curl(step_prefix: String, sh: &Shell) -> Result<()> {
    println!(
        "\n{} Installing {}...",
        step_text("2."),
        highlighted_text("curl")
    );
    cmd!(sh, "sudo apt install curl -y").run()?;
    println!(
        "{}",
        success_text(format!("{step_prefix} Successfully installed curl.").as_str())
    );
    Ok(())
}

pub fn step_install_openssh(step_prefix: String, sh: &Shell) -> Result<()> {
    println!(
        "\n{} Installing openssh-server...",
        step_text(step_prefix.as_str())
    );
    cmd!(sh, "sudo apt install openssh-server -y").run()?;
    println!(
        "{}",
        success_text(format!("{step_prefix} Successfully installed openssh-server.").as_str())
    );
    Ok(())
}

pub fn step_install_docker(step_prefix: String, sh: &Shell, username: String) -> Result<()> {
    let dpkg_l_output = cmd!(sh, "dpkg -l").read()?;
    let has_docker = cmd!(sh, "grep docker")
        .stdin(dpkg_l_output.clone())
        .ignore_status()
        .output()?
        .status
        .success();
    if !has_docker {
        println!("{} Installing Docker...", step_text(step_prefix.as_str()),);
        cmd!(
            sh,
            "curl -fsSL https://get.docker.com -o /home/{username}/get-docker.sh"
        )
        .run()?;
        cmd!(sh, "sudo sh /home/{username}/get-docker.sh").run()?;
        println!(
            "{}",
            success_text(format!("{step_prefix} Successfully installed Docker.").as_str())
        );
    }
    Ok(())
}

pub fn step_install_ahoy(step_prefix: String, sh: &Shell, username: String) -> Result<()> {
    println!("\n{} Installing Ahoy...", step_text(step_prefix.as_str()),);
    sh.change_dir(format!("/home/{username}"));
    cmd!(sh, "sudo curl -LO https://github.com/ahoy-cli/ahoy/releases/download/v2.5.0/ahoy-bin-linux-amd64").run()?;
    cmd!(sh, "mv ./ahoy-bin-linux-amd64 ./ahoy").run()?;
    cmd!(sh, "sudo chmod +x ./ahoy").run()?;
    println!(
        "{}",
        success_text(format!("{step_prefix} Successfully installed Ahoy.").as_str())
    );
    Ok(())
}

pub fn step_install_and_run_ckan_compose(
    step_prefix: String,
    sh: &Shell,
    username: String,
) -> Result<()> {
    println!(
        "\n{} Downloading, installing, and starting ckan-compose...",
        step_text(step_prefix.as_str()),
    );
    if !std::fs::exists(format!("/home/{username}/ckan-compose"))? {
        cmd!(sh, "git clone https://github.com/tino097/ckan-compose.git").run()?;
    }
    sh.change_dir(format!("/home/{username}/ckan-compose"));
    cmd!(sh, "git switch ckan-devstaller").run()?;
    let env_data = "PROJECT_NAME=ckan-devstaller-project
DATASTORE_READONLY_PASSWORD=pass
POSTGRES_PASSWORD=pass";
    std::fs::write(format!("/home/{username}/ckan-compose/.env"), env_data)?;
    cmd!(sh, "sudo ../ahoy up").run()?;
    println!(
        "{}",
        success_text(format!("{step_prefix} Successfully ran ckan-compose.").as_str())
    );
    Ok(())
}
