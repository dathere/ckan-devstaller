# ckan-devstaller

`ckan-devstaller` attempts to install CKAN 2.11.3 from source using [ckan-compose](https://github.com/tino097/ckan-compose), intended for development use in a new Ubuntu 22.04 instance. The following are also installed and enabled by default:

- [DataStore extension](https://docs.ckan.org/en/2.11/maintaining/datastore.html)
- [ckanext-scheming extension](https://github.com/ckan/ckanext-scheming)
- [DataPusher+ extension](https://github.com/dathere/datapusher-plus)
- [DRUF mode](https://github.com/dathere/datapusher-plus?tab=readme-ov-file#druf-dataset-resource-upload-first-workflow)

The [`datatablesview-plus` extension](https://github.com/dathere/ckanext-datatables-plus) is planned to be included in a future release.

> ![INFO]
> We plan on including customizability for enabling/disabling features in a future release.

## Quick start

> [!CAUTION]
> Make sure `ckan-devstaller` is run in a **new** Ubuntu 22.04 instance. Do NOT run `ckan-devstaller` in an existing instance that is important for your usage.

> [!NOTE]  
> The `/etc/ckan/default/ckan.ini` config file will have its comments removed for now. There are plans to fix this in a future release of `ckan-devstaller`.

Paste this into your new Ubuntu 22.04 instance's terminal:

```bash
curl -fsSL https://github.com/dathere/ckan-devstaller/releases/download/0.1.0/install.bash | bash
```

<img width="1271" height="183" alt="{8479CBE1-788E-48B3-AE9C-F3A51724520C}" src="https://github.com/user-attachments/assets/86373a89-895b-403c-a699-0cf3865ee100" />

## Demo (sped up)

![ckan-devstaller-demo](https://github.com/user-attachments/assets/9fc388ab-e044-4453-ae49-7d7f31065fe3)
