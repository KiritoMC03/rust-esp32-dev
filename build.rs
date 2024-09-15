use std::{fs::File, io::Write};
use uuid::Uuid;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() {
    uuid_on_build().expect("Can't create uuid file");
    cfg_toml_check().expect("Can't check cfg.toml file");
    embuild::espidf::sysenv::output();
}

fn uuid_on_build() -> anyhow::Result<()> {
    if let Ok(_already_exists) = File::open("uuid.toml") {
        return Ok(());
    }

    let mut uuid_file = File::create("uuid.toml")?;
    uuid_file.write_all("[get-uuid]\n".as_bytes())?;
    let uuid_val = Uuid::new_v4().to_string();
    uuid_file.write_fmt(format_args!("uuid = \"{}\"\n", uuid_val))?;

    let package_root = env!("CARGO_MANIFEST_DIR");
    let uuid_rs = format!("{}/_uuid.rs", package_root);
    let mut uuid_file = File::create(uuid_rs)?;
    uuid_file.write_fmt(format_args!(
        "const UUID: &'static str = \"{}\";\n",
        uuid_val
    ))?;

    Ok(())
}

fn cfg_toml_check() -> anyhow::Result<()> {
    // Check if the `cfg.toml` file exists and has been filled out.
    if !std::path::Path::new("cfg.toml").exists() {
        panic!("You need to create a `cfg.toml` file with your Wi-Fi credentials! Use `cfg.toml.example` as a template.");
    }

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;
    if app_config.wifi_ssid == "FBI Surveillance Van" || app_config.wifi_psk == "hunter2" {
        panic!("You need to set the Wi-Fi credentials in `cfg.toml`!");
    }
    
    Ok(())
}