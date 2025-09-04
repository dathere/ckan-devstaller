# ckan-devstaller

`ckan-devstaller` attempts to install CKAN 2.11.3 from source using [ckan-compose](https://github.com/tino097/ckan-compose), intended for development use in a new Ubuntu 22.04 instance. The following are also installed and enabled by default:

- [DataStore extension](https://docs.ckan.org/en/2.11/maintaining/datastore.html)
- [ckanext-scheming extension](https://github.com/ckan/ckanext-scheming)
- [DataPusher+ extension](https://github.com/dathere/datapusher-plus)

[DRUF mode](https://github.com/dathere/datapusher-plus?tab=readme-ov-file#druf-dataset-resource-upload-first-workflow) is available but disabled by default. The [`datatablesview-plus` extension](https://github.com/dathere/ckanext-datatables-plus) is planned to be included in a future release.

## Quick start

> [!CAUTION]
> Make sure `ckan-devstaller` is run in a **new** Ubuntu 22.04 instance. Do NOT run `ckan-devstaller` in an existing instance that is important for your usage.

> [!WARNING]
> If you are using Ubuntu 22.04 on VirtualBox, you may need to add your user to the sudoers file before running the ckan-devstaller install script. Open a terminal in your virtual machine (VM), run `su -` and log in as the root user with the password you used to set up the VM, then type `sudo adduser <username> sudo` where `<username>` is your username then restart your VM and run the ckan-devstaller installer script.

> [!NOTE]  
> The `/etc/ckan/default/ckan.ini` config file will have its comments removed for now. There are plans to fix this in a future release of `ckan-devstaller`.

> [!NOTE]  
> Currently `ckan-devstaller` supports x86 architecture. ARM support is planned.

You have two common options to choose from for installation. Paste one of the following scripts into your new Ubuntu 22.04 instance's terminal.

### Install with non-interactive mode (default config)

```bash
wget -O - https://github.com/dathere/ckan-devstaller/releases/download/0.2.0/install.bash | bash -s default
```

### Install with interactive mode

```bash
wget -O - https://github.com/dathere/ckan-devstaller/releases/download/0.2.0/install.bash | bash
```

## Demos

### Interactive customizable installation

![ckan-devstaller-interactive-mode-demo](https://github.com/user-attachments/assets/cc12471c-5b20-4571-85d6-8a4351931419)

### Installation (sped up)
![ckan-devstaller-demo](https://github.com/user-attachments/assets/9fc388ab-e044-4453-ae49-7d7f31065fe3)
