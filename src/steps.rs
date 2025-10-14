use crate::styles::{highlighted_text, important_text, step_text, success_text};
use anyhow::Result;
use serde_json::json;
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

pub fn step_install_datastore_extension(
    step_prefix: String,
    sh: &Shell,
    username: String,
) -> Result<()> {
    println!(
        "\n{} Enabling DataStore plugin, adding config URLs in /etc/ckan/default/ckan.ini and updating permissions...",
        step_text(step_prefix.as_str()),
    );
    let mut conf = ini::Ini::load_from_file("/etc/ckan/default/ckan.ini")?;
    let app_main_section = conf.section_mut(Some("app:main")).unwrap();
    let mut ckan_plugins = app_main_section.get("ckan.plugins").unwrap().to_string();
    ckan_plugins.push_str(" datastore");
    app_main_section.insert("ckan.plugins", ckan_plugins);
    app_main_section.insert(
        "ckan.datastore.write_url",
        "postgresql://ckan_default:pass@localhost/datastore_default",
    );
    app_main_section.insert(
        "ckan.datastore.read_url",
        "postgresql://datastore_default:pass@localhost/datastore_default",
    );
    app_main_section.insert("ckan.datastore.sqlsearch.enabled", "true");
    conf.write_to_file("/etc/ckan/default/ckan.ini")?;
    let postgres_container_id = cmd!(
        sh,
        "sudo docker ps -aqf name=^ckan-devstaller-project-postgres$"
    )
    .read()?;
    let set_permissions_output = cmd!(
        sh,
        "ckan -c /etc/ckan/default/ckan.ini datastore set-permissions"
    )
    .read()?;
    std::fs::write("permissions.sql", set_permissions_output)?;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        if std::fs::exists("permissions.sql")? {
            break;
        }
    }
    sh.change_dir(format!("/home/{username}"));
    cmd!(
        sh,
        "sudo docker cp permissions.sql {postgres_container_id}:/permissions.sql"
    )
    .run()?;
    cmd!(sh, "sudo docker exec {postgres_container_id} psql -U ckan_default --set ON_ERROR_STOP=1 -f permissions.sql").run()?;
    println!(
        "{}",
        success_text(
            format!("{step_prefix} Enabled DataStore plugin, set DataStore URLs in /etc/ckan/default/ckan.ini, and updated permissions.").as_str()
        )
    );
    Ok(())
}

pub fn step_install_ckanext_scheming_extension(
    step_prefix: String,
    sh: &Shell,
    username: String,
) -> Result<()> {
    println!(
        "{}",
        step_text("\n{} Installing the ckanext-scheming extension..."),
    );
    cmd!(
        sh,
        "pip install -e git+https://github.com/ckan/ckanext-scheming.git#egg=ckanext-scheming"
    )
    .run()?;
    let mut conf = ini::Ini::load_from_file("/etc/ckan/default/ckan.ini")?;
    let app_main_section = conf.section_mut(Some("app:main")).unwrap();
    let mut ckan_plugins = app_main_section.get("ckan.plugins").unwrap().to_string();
    ckan_plugins.push_str(" scheming_datasets");
    cmd!(
        sh,
        "ckan config-tool /etc/ckan/default/ckan.ini -s app:main ckan.plugins={ckan_plugins}"
    )
    .run()?;
    cmd!(sh, "ckan config-tool /etc/ckan/default/ckan.ini -s app:main scheming.presets=ckanext.scheming:presets.json").run()?;
    cmd!(
        sh,
        "ckan config-tool /etc/ckan/default/ckan.ini -s app:main scheming.dataset_fallback=false"
    )
    .run()?;
    // app_main_section.insert("ckan.plugins", ckan_plugins);
    // app_main_section.insert("scheming.presets", "ckanext.scheming:presets.json");
    // app_main_section.insert("scheming.dataset_fallback", "false");
    // conf.write_to_file("/etc/ckan/default/ckan.ini")?;
    Ok(())
}

pub fn step_install_datapusher_plus_extension(
    step_prefix: String,
    sh: &Shell,
    sysadmin_username: String,
    username: String,
) -> Result<()> {
    // Install DataPusher+
    println!(
        "{}",
        step_text(format!("\n{step_prefix} Installing DataPusher+ extension...").as_str())
    );
    cmd!(sh, "sudo apt install python3-virtualenv python3-dev python3-pip python3-wheel build-essential libxslt1-dev libxml2-dev zlib1g-dev git libffi-dev libpq-dev uchardet -y").run()?;
    sh.change_dir("/usr/lib/ckan/default/src");
    cmd!(
        sh,
        "pip install -e git+https://github.com/dathere/datapusher-plus.git@main#egg=datapusher-plus"
    )
    .run()?;
    sh.change_dir("/usr/lib/ckan/default/src/datapusher-plus");
    cmd!(sh, "pip install -r requirements.txt").run()?;
    sh.change_dir(format!("/home/{username}"));
    cmd!(sh, "wget https://github.com/dathere/qsv/releases/download/4.0.0/qsv-4.0.0-x86_64-unknown-linux-gnu.zip").run()?;
    cmd!(sh, "sudo apt install unzip -y").run()?;
    cmd!(sh, "unzip qsv-4.0.0-x86_64-unknown-linux-gnu.zip").run()?;
    cmd!(sh, "sudo rm -rf qsv-4.0.0-x86_64-unknown-linux-gnu.zip").run()?;
    cmd!(sh, "sudo mv ./qsvdp_glibc-2.31 /usr/local/bin/qsvdp").run()?;
    let mut conf = ini::Ini::load_from_file("/etc/ckan/default/ckan.ini")?;
    let app_main_section = conf.section_mut(Some("app:main")).unwrap();
    let mut ckan_plugins = app_main_section.get("ckan.plugins").unwrap().to_string();
    ckan_plugins.push_str(" datapusher_plus");
    app_main_section.insert("ckan.plugins", ckan_plugins);
    app_main_section.insert(
        "scheming.dataset_schemas",
        "ckanext.datapusher_plus:dataset-druf.yaml",
    );
    app_main_section.insert("ckanext.datapusher_plus.use_proxy", "false");
    app_main_section.insert("ckanext.datapusher_plus.download_proxy", "");
    app_main_section.insert("ckanext.datapusher_plus.ssl_verify", "false");
    app_main_section.insert("ckanext.datapusher_plus.upload_log_level", "INFO");
    app_main_section.insert(
        "ckanext.datapusher_plus.formats",
        "csv tsv tab ssv xls xlsx xlsxb xlsm ods geojson shp qgis zip",
    );
    app_main_section.insert("ckanext.datapusher_plus.pii_screening", "false");
    app_main_section.insert("ckanext.datapusher_plus.pii_found_abort", "false");
    app_main_section.insert("ckanext.datapusher_plus.pii_regex_resource_id_or_alias", "");
    app_main_section.insert("ckanext.datapusher_plus.pii_show_candidates", "false");
    app_main_section.insert("ckanext.datapusher_plus.pii_quick_screen", "false");
    app_main_section.insert("ckanext.datapusher_plus.qsv_bin", "/usr/local/bin/qsvdp");
    app_main_section.insert("ckanext.datapusher_plus.preview_rows", "100");
    app_main_section.insert("ckanext.datapusher_plus.download_timeout", "300");
    app_main_section.insert(
        "ckanext.datapusher_plus.max_content_length",
        "1256000000000",
    );
    app_main_section.insert("ckanext.datapusher_plus.chunk_size", "16384");
    app_main_section.insert("ckanext.datapusher_plus.default_excel_sheet", "0");
    app_main_section.insert("ckanext.datapusher_plus.sort_and_dupe_check", "true");
    app_main_section.insert("ckanext.datapusher_plus.dedup", "false");
    app_main_section.insert("ckanext.datapusher_plus.unsafe_prefix", "unsafe_");
    app_main_section.insert("ckanext.datapusher_plus.reserved_colnames", "_id");
    app_main_section.insert("ckanext.datapusher_plus.prefer_dmy", "false");
    app_main_section.insert("ckanext.datapusher_plus.ignore_file_hash", "true");
    app_main_section.insert("ckanext.datapusher_plus.auto_index_threshold", "3");
    app_main_section.insert("ckanext.datapusher_plus.auto_index_dates", "true");
    app_main_section.insert("ckanext.datapusher_plus.auto_unique_index", "true");
    app_main_section.insert("ckanext.datapusher_plus.summary_stats_options", "");
    app_main_section.insert(
        "ckanext.datapusher_plus.add_summary_stats_resource",
        "false",
    );
    app_main_section.insert(
        "ckanext.datapusher_plus.summary_stats_with_preview",
        "false",
    );
    app_main_section.insert(
        "ckanext.datapusher_plus.qsv_stats_string_max_length",
        "32767",
    );
    app_main_section.insert(
        "ckanext.datapusher_plus.qsv_dates_whitelist",
        "date,time,due,open,close,created",
    );
    app_main_section.insert("ckanext.datapusher_plus.qsv_freq_limit", "10");
    app_main_section.insert("ckanext.datapusher_plus.auto_alias", "true");
    app_main_section.insert("ckanext.datapusher_plus.auto_alias_unique", "false");
    app_main_section.insert("ckanext.datapusher_plus.copy_readbuffer_size", "1048576");
    app_main_section.insert("ckanext.datapusher_plus.type_mapping", r#"{"String": "text", "Integer": "numeric","Float": "numeric","DateTime": "timestamp","Date": "date","NULL": "text"}"#);
    app_main_section.insert("ckanext.datapusher_plus.auto_spatial_simplication", "true");
    app_main_section.insert(
        "ckanext.datapusher_plus.spatial_simplication_relative_tolerance",
        "0.1",
    );
    app_main_section.insert("ckanext.datapusher_plus.latitude_fields", "latitude,lat");
    app_main_section.insert(
        "ckanext.datapusher_plus.longitude_fields",
        "longitude,long,lon",
    );
    app_main_section.insert(
        "ckanext.datapusher_plus.jinja2_bytecode_cache_dir",
        "/tmp/jinja2_butecode_cache",
    );
    app_main_section.insert("ckanext.datapusher_plus.auto_unzip_one_file", "true");
    app_main_section.insert(
        "ckanext.datapusher_plus.api_token",
        "<CKAN service account token for CKAN user with sysadmin privileges>",
    );
    app_main_section.insert(
        "ckanext.datapusher_plus.describeGPT_api_key",
        "<Token for OpenAI API compatible service>",
    );
    app_main_section.insert("ckanext.datapusher_plus.file_bin", "/usr/bin/file");
    app_main_section.insert("ckanext.datapusher_plus.enable_druf", "false");
    app_main_section.insert("ckanext.datapusher_plus.enable_form_redirect", "true");
    conf.write_to_file("/etc/ckan/default/ckan.ini")?;
    let resource_formats_str = std::fs::read_to_string(
        "/usr/lib/ckan/default/src/ckan/ckan/config/resource_formats.json",
    )?;
    let mut resource_formats_val: serde_json::Value = serde_json::from_str(&resource_formats_str)?;
    let all_resource_formats = resource_formats_val
        .get_mut(0)
        .unwrap()
        .as_array_mut()
        .unwrap();
    all_resource_formats.push(json!([
        "TAB",
        "Tab Separated Values File",
        "text/tab-separated-values",
        []
    ]));
    std::fs::write(
        "/usr/lib/ckan/default/src/ckan/ckan/config/resource_formats.json",
        serde_json::to_string(&resource_formats_val)?,
    )?;
    cmd!(sh, "sudo locale-gen en_US.UTF-8").run()?;
    cmd!(sh, "sudo update-locale").run()?;
    let token_command_output = cmd!(
        sh,
        "ckan -c /etc/ckan/default/ckan.ini user token add {sysadmin_username} dpplus"
    )
    .read()?;
    let tail_output = cmd!(sh, "tail -n 1").stdin(token_command_output).read()?;
    let dpp_api_token = cmd!(sh, "tr -d '\t'").stdin(tail_output).read()?;
    cmd!(sh, "ckan config-tool /etc/ckan/default/ckan.ini ckanext.datapusher_plus.api_token={dpp_api_token}").env("LC_ALL", "en_US.UTF-8").run()?;
    cmd!(
        sh,
        "ckan -c /etc/ckan/default/ckan.ini db upgrade -p datapusher_plus"
    )
    .run()?;
    println!(
        "{}",
        success_text(format!("{step_prefix} Installed DataPusher+ extension.").as_str())
    );
    Ok(())
}
