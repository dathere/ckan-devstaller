use anyhow::Result;
use inquire::Confirm;
use owo_colors::{OwoColorize, Stream::Stdout};
use serde_json::json;
use std::{path::PathBuf, str::FromStr};
use xshell::cmd;
use xshell_venv::{Shell, VirtualEnv};

fn main() -> Result<()> {
    println!("Welcome to the ckan-devstaller!");
    println!(
        "ckan-devstaller is provided by datHere - {}\n",
        "https://datHere.com".if_supports_color(Stdout, |text| text.on_blue().white()),
    );
    println!(
        "This installer should assist in setting up {} from a source installation along with ckan-compose (https://github.com/tino097/ckan-compose). If you have any issues, please report them at https://github.com/dathere/ckan-devstaller/issues.",
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
        // Ignoring xrdp error with .ignore_status() for now
        cmd!(sh, "sudo apt upgrade -y").ignore_status().run()?;
        println!(
            "{}",
            "✅ 1. Successfully ran update and upgrade commands."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );

        println!(
            "\n{} Installing curl and enabling SSH...",
            "2.".if_supports_color(Stdout, |text| text.on_magenta().white()),
        );
        cmd!(sh, "sudo apt install curl openssh-server -y").run()?;
        println!(
            "{}",
            "✅ 2. Successfully installed curl and enabled SSH."
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
        println!(
            "{}",
            "✅ 6. Installed CKAN 2.11.3 and started running instance."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );

        println!(
            "\n{} Enabling DataStore plugin, adding config URLs in /etc/ckan/default/ckan.ini and updating permissions...",
            "7.".if_supports_color(Stdout, |text| text.on_magenta().white()),
        );
        // TODO: use the ckan config-tool command instead of rust-ini
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
            "✅ 7. Enabled DataStore plugin, set DataStore URLs in /etc/ckan/default/ckan.ini, and updated permissions."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );

        println!(
            "\n{} Installing ckanext-scheming and DataPusher+ extensions...",
            "8.".if_supports_color(Stdout, |text| text.on_magenta().white()),
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
        app_main_section.insert("ckan.plugins", ckan_plugins);
        app_main_section.insert("scheming.presets", "ckanext.scheming:presets.json");
        app_main_section.insert("scheming.dataset_fallback", "false");
        conf.write_to_file("/etc/ckan/default/ckan.ini")?;
        // Install DataPusher+
        cmd!(sh, "sudo apt install python3-virtualenv python3-dev python3-pip python3-wheel build-essential libxslt1-dev libxml2-dev zlib1g-dev git libffi-dev libpq-dev uchardet -y").run()?;
        sh.change_dir("/usr/lib/ckan/default/src");
        cmd!(sh, "pip install -e git+https://github.com/dathere/datapusher-plus.git@main#egg=datapusher-plus").run()?;
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
        conf.write_to_file("/etc/ckan/default/ckan.ini")?;
        let dpp_default_config = r#"
ckanext.datapusher_plus.use_proxy = false
ckanext.datapusher_plus.download_proxy = 
ckanext.datapusher_plus.ssl_verify = false
# supports INFO, DEBUG, TRACE - use DEBUG or TRACE when debugging scheming Formulas
ckanext.datapusher_plus.upload_log_level = INFO
ckanext.datapusher_plus.formats = csv tsv tab ssv xls xlsx xlsxb xlsm ods geojson shp qgis zip
ckanext.datapusher_plus.pii_screening = false
ckanext.datapusher_plus.pii_found_abort = false
ckanext.datapusher_plus.pii_regex_resource_id_or_alias =
ckanext.datapusher_plus.pii_show_candidates = false
ckanext.datapusher_plus.pii_quick_screen = false
ckanext.datapusher_plus.qsv_bin = /usr/local/bin/qsvdp
ckanext.datapusher_plus.preview_rows = 100
ckanext.datapusher_plus.download_timeout = 300
ckanext.datapusher_plus.max_content_length = 1256000000000
ckanext.datapusher_plus.chunk_size = 16384
ckanext.datapusher_plus.default_excel_sheet = 0
ckanext.datapusher_plus.sort_and_dupe_check = true
ckanext.datapusher_plus.dedup = false
ckanext.datapusher_plus.unsafe_prefix = unsafe_
ckanext.datapusher_plus.reserved_colnames = _id
ckanext.datapusher_plus.prefer_dmy = false
ckanext.datapusher_plus.ignore_file_hash = true
ckanext.datapusher_plus.auto_index_threshold = 3
ckanext.datapusher_plus.auto_index_dates = true
ckanext.datapusher_plus.auto_unique_index = true
ckanext.datapusher_plus.summary_stats_options =
ckanext.datapusher_plus.add_summary_stats_resource = false
ckanext.datapusher_plus.summary_stats_with_preview = false
ckanext.datapusher_plus.qsv_stats_string_max_length = 32767
ckanext.datapusher_plus.qsv_dates_whitelist = date,time,due,open,close,created
ckanext.datapusher_plus.qsv_freq_limit = 10
ckanext.datapusher_plus.auto_alias = true
ckanext.datapusher_plus.auto_alias_unique = false
ckanext.datapusher_plus.copy_readbuffer_size = 1048576
ckanext.datapusher_plus.type_mapping = {"String": "text", "Integer": "numeric","Float": "numeric","DateTime": "timestamp","Date": "date","NULL": "text"}
ckanext.datapusher_plus.auto_spatial_simplication = true
ckanext.datapusher_plus.spatial_simplication_relative_tolerance = 0.1
ckanext.datapusher_plus.latitude_fields = latitude,lat
ckanext.datapusher_plus.longitude_fields = longitude,long,lon
ckanext.datapusher_plus.jinja2_bytecode_cache_dir = /tmp/jinja2_butecode_cache
ckanext.datapusher_plus.auto_unzip_one_file = true
ckanext.datapusher_plus.api_token = <CKAN service account token for CKAN user with sysadmin privileges>
ckanext.datapusher_plus.describeGPT_api_key = <Token for OpenAI API compatible service>
ckanext.datapusher_plus.file_bin = /usr/bin/file
ckanext.datapusher_plus.enable_druf = true
ckanext.datapusher_plus.enable_form_redirect = true
"#;

        std::fs::write("dpp_default_config.ini", dpp_default_config)?;
        cmd!(
            sh,
            "ckan config-tool /etc/ckan/default/ckan.ini -f dpp_default_config.ini"
        )
        .run()?;
        let resource_formats_str =
            std::fs::read_to_string("/usr/lib/ckan/default/src/ckan/config/resource_formats.json")?;
        let mut resource_formats_val: serde_json::Value =
            serde_json::from_str(&resource_formats_str)?;
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
            "/usr/lib/ckan/default/src/ckan/config/resource_formats.json",
            serde_json::to_string(&resource_formats_val)?,
        )?;
        // let token_command_output = cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini user token add {username} dpplus | tail -n 1 | tr -d '\t'").read()?;
        // let dpp_api_token = duct::cmd!("tr", "-d", "'\t").stdin_bytes(duct::cmd!("tail", "-n", "1").stdin_bytes(token_command_output)).read()?;
        cmd!(sh, "sudo locale-gen en_US.UTF-8").run()?;
        cmd!(sh, "sudo update-locale").run()?;
        let token_command_output = cmd!(
            sh,
            "ckan -c /etc/ckan/default/ckan.ini user token add {username} dpplus"
        )
        .read()?;
        let tail_output = cmd!(sh, "tail -n 1").stdin(token_command_output).read()?;
        let dpp_api_token = cmd!(sh, "tr -d '\t'").stdin(tail_output).read()?;
        cmd!(sh, "ckan config-tool /etc/ckan/default/ckan.ini ckanext.datapusher_plus.api_token={dpp_api_token}").run()?;
        cmd!(
            sh,
            "ckan -c /etc/ckan/default/ckan.ini db upgrade -p datapusher_plus"
        )
        .run()?;
        println!(
            "{}",
            "✅ 8. Installed ckanext-scheming and DataPusher+ extensions."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );

        println!(
            "\n{}",
            "✅ 9. Running CKAN instance..."
                .if_supports_color(Stdout, |text| text.on_green().white())
        );
        cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini run").run()?;
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
