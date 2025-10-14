mod questions;
mod steps;
mod styles;

use crate::{
    questions::{question_ckan_version, question_ssh, question_sysadmin},
    steps::{
        step_install_ahoy, step_install_and_run_ckan_compose, step_install_curl,
        step_install_docker, step_install_openssh, step_package_updates,
    },
    styles::{important_text, step_text, success_text},
};
use anyhow::Result;
use clap::Parser;
use human_panic::{metadata, setup_panic};
use inquire::Confirm;
use serde_json::json;
use std::{path::PathBuf, str::FromStr};
use xshell::cmd;
use xshell_venv::{Shell, VirtualEnv};

/// CLI to help install a CKAN instance for development within minutes. Learn more at: https://ckan-devstaller.dathere.com
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Skip interactive steps and install CKAN with datHere's default config
    #[arg(short, long)]
    default: bool,
    /// Preset configuration.
    #[arg(short, long)]
    preset: Option<String>,
    #[arg(short, long)]
    /// CKAN version to install defined by semantic versioning from official releases from https://github.com/ckan/ckan, or a custom git repository.
    ckan_version: Option<String>,
    /// List of CKAN extensions to install, separated by either commas or spaces.
    #[arg(short, long)]
    extensions: Option<Vec<String>>,
    /// List of custom features, separated by either commas or spaces.
    #[arg(short, long)]
    features: Option<Vec<String>>,
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
    let default_sysadmin = Sysadmin {
        username: username.clone(),
        password: "password".to_string(),
        email: format!("{username}@localhost"),
    };
    let config = Config {
        ssh: args.features.is_some_and(|features| features.contains(&"enable-ssh".to_string())),
        ckan_version: if args.ckan_version.is_some() { args.ckan_version.unwrap() } else { "2.11.3".to_string() },
        sysadmin: default_sysadmin.clone(),
        extension_datastore: args.extensions.clone().is_some_and(|extensions| extensions.contains(&"DataStore".to_string())),
        extension_ckanext_scheming: args.extensions.clone().is_some_and(|extensions| extensions.contains(&"ckanext-scheming".to_string())),
        extension_datapusher_plus: args.extensions.is_some_and(|extensions| extensions.contains(&"DataPusher+".to_string())),
        druf_mode: false,
    };

    steps::step_intro();

    let mut default_config_text = String::from("The current configuration for ckan-devstaller does the following:");
    if config.ssh {
        default_config_text.push_str("\n- Install openssh-server to enable SSH access");
    }
    default_config_text.push_str("\n- Install ckan-compose (https://github.com/tino097/ckan-compose) which sets up the CKAN backend (PostgreSQL, SOLR, Redis)");
    default_config_text.push_str(format!("\n- Install CKAN v{}", config.ckan_version).as_str());
    if config.extension_datastore {
        default_config_text.push_str("\n- Install the DataStore extension");
    }
    if config.extension_ckanext_scheming {
        default_config_text.push_str("\n- Install the ckanext-scheming extension");
    }
    if config.extension_datapusher_plus {
        default_config_text.push_str("\n- Install the DataPusher+ extension");
    }
    default_config_text.push_str("\n- Disable DRUF mode for DataPusher+");
    println!("{default_config_text}");
    let answer_customize = if args.default {
        false
    } else {
        Confirm::new(
            "Would you like to customize the configuration for your CKAN installation?",
        )
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

    let begin_installation = if args.default {
        true
    } else {
        Confirm::new("Would you like to begin the installation?").prompt()?
    };

    if begin_installation {
        println!("{}", important_text("Starting installation..."));
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
            success_text(format!("✅ 6. Installed CKAN {}.", config.ckan_version).as_str())
        );

        if config.extension_datapusher_plus {
            println!(
                "\n{} Enabling DataStore plugin, adding config URLs in /etc/ckan/default/ckan.ini and updating permissions...",
                step_text("7."),
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
                    "✅ 7. Enabled DataStore plugin, set DataStore URLs in /etc/ckan/default/ckan.ini, and updated permissions."
                )
            );

            println!(
                "{}",
                step_text("\n{} Installing ckanext-scheming and DataPusher+ extensions..."),
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
            cmd!(sh, "ckan config-tool /etc/ckan/default/ckan.ini -s app:main scheming.dataset_fallback=false").run()?;
            // app_main_section.insert("ckan.plugins", ckan_plugins);
            // app_main_section.insert("scheming.presets", "ckanext.scheming:presets.json");
            // app_main_section.insert("scheming.dataset_fallback", "false");
            // conf.write_to_file("/etc/ckan/default/ckan.ini")?;
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
            cmd!(
            sh,
            "ckan config-tool /etc/ckan/default/ckan.ini -s app:main ckan.plugins={ckan_plugins}"
        )
        .run()?;
            cmd!(sh, "ckan config-tool /etc/ckan/default/ckan.ini -s app:main scheming.dataset_schemas=ckanext.datapusher_plus:dataset-druf.yaml").run()?;
            // app_main_section.insert("ckan.plugins", ckan_plugins);
            // app_main_section.insert(
            //     "scheming.dataset_schemas",
            //     "ckanext.datapusher_plus:dataset-druf.yaml",
            // );
            // conf.write_to_file("/etc/ckan/default/ckan.ini")?;
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
ckanext.datapusher_plus.enable_druf = false
ckanext.datapusher_plus.enable_form_redirect = true
"#;
            std::fs::write("dpp_default_config.ini", dpp_default_config)?;
            cmd!(
                sh,
                "ckan config-tool /etc/ckan/default/ckan.ini -f dpp_default_config.ini"
            )
            .run()?;
            let resource_formats_str = std::fs::read_to_string(
                "/usr/lib/ckan/default/src/ckan/config/resource_formats.json",
            )?;
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
                success_text("✅ 8. Installed ckanext-scheming and DataPusher+ extensions.")
            );
        }

        println!("\n{}", success_text("✅ Running CKAN instance..."));
        cmd!(sh, "ckan -c /etc/ckan/default/ckan.ini run").run()?;
    }

    Ok(())
}
