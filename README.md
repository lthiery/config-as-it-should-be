# Config as it should be

This demonstrates how I think configuration should be done with Rust apps.

## Module definitions ##
Every module should define a serde struct that represents its configuration variables. This is represented by `Gateway`
in `config.rs`:
```rust
#[derive(Debug, Deserialize)]
pub struct Gateway {
    pub ip_address: String,
    pub port: u16,
}
```
The important thing is that the modules (and certainly libraries) should not be concerned with HOW the user is eventually
choosing to load in the configuration.

## Application definition ##
The application's configuration will become a collection of these structs. In this case, the `Config` struct. 
```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    pub gateway: Gateway
}
```
This one is very simple, but you can imagine that it becomes quite large in a real application.

## Loading the configuration ##
The application decides how to load the config definition. The `config` crate provides a convenient way to do this 
where you can layer environmental variables to override file's settings which may have been deserialized from a TOML file.
```rust
let config_builder = Config::builder()
        .add_source(File::with_name(&args.config_path))
        .add_source(Environment::with_prefix("APP").separator("__"))
        .build()
        .unwrap();
```
The `__` separator here is important because if any setting fields have a space, it will be represented with `_`. In
this case, we would have never been able to set `APP_GATEWAY_IP_ADDRESS`.

## Mixing with CLI args ##
Do not override any settings with CLI args; that's what the environment variables are for! You _should_ use clap to
indicate the location of the settings file and generally "commands" instead of configuration should live there.

## Deployment ##
You may be shipping this application and running it with systemd. I highly recommend _not_ listing all the environmental
variables in the systemd service file. Instead, you can use ansible to write into the TOML file on deployment, eg:
```toml
[gateway]
ip_address = {{ ip_address }}
port = {{ port }}
```

```ansible
- name: Render TOML config
  ansible.builtin.template:
    src: templates/config.toml.j2
    dest: /etc/app/config.toml
  vars:
    ip_address: "localhost"
    port: 8080
```

This makes it easy to inspect the configuration file and restart the daemon without worrying about `systemctl daemon-reload`.