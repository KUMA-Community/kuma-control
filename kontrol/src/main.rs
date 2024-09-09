mod commands;
mod help;

use commands::*;
use std::io::IsTerminal;

fn main() -> Result<(), anyhow::Error> {
    if std::env::args().count() <= 2 {
        help::print_help_and_exit();
    }

    let command = std::env::args().nth(1).unwrap();
    let subcommand = std::env::args().nth(2).unwrap();

    if command == HELP {
        match subcommand.as_str() {
            INIT => {
                help::print_help_init_and_exit();
            }
            ASSETS => {
                help::print_help_assets_and_exit();
            }
            EVENTS => {}
            RESOURCES => {}
            SERVICES => {}
            TENANTS => {}
            _ => {}
        }

        anyhow::bail!("unknown argument '{}'", subcommand);
    }

    let client = http::Client::from_config()?;
    let subcommand = std::env::args().nth(2).unwrap();
    match command.as_str() {
        INIT => {
            return config::Config::interactive_init();
        }
        ASSETS => {
            let asset_client = assets::Client::new(client.clone());
            match subcommand.as_str() {
                LIST => {
                    return asset_client.list();
                }
                IMPORT => {
                    let input = std::io::stdin();

                    let from_input = !input.is_terminal();

                    let tenant_id: Option<String> = if let Some(tenant_name) =
                        std::env::args().nth(3)
                    {
                        if !tenant_name.starts_with("tenant=") {
                            if !from_input {
                                anyhow::bail!(
                                    "first argument after import must be tenant, for example, assets import tenant=Main ..."
                                );
                            }
                            None
                        } else {
                            let tenant_name = tenant_name.strip_prefix("tenant=").unwrap();
                            let tenant_id =
                                tenants::Client::new(client).find_by_name(tenant_name)?;
                            if let Some(tenant_id) = tenant_id {
                                Some(tenant_id)
                            } else {
                                anyhow::bail!("tenant with name '{}' is not found", tenant_name);
                            }
                        }
                    } else {
                        None
                    };

                    let input = std::io::stdin();

                    if input.is_terminal() {
                        if tenant_id.is_none() {
                            anyhow::bail!(assets::NOT_SPECIFIED_TENANT_MSG);
                        }
                        let fields: Vec<String> = std::env::args().skip(4).collect();
                        return asset_client.import_from_fields(&tenant_id.unwrap(), &fields[..]);
                    }

                    if std::env::args().nth(5).is_some() {
                        let useless_args = std::env::args().skip(4).collect::<Vec<String>>();
                        anyhow::bail!("useless arguments specified: {}", useless_args.join(" "));
                    }

                    return asset_client.import_from_input(tenant_id, input);
                }
                _ => {}
            }
        }
        EVENTS => {}
        RESOURCES => {}
        SERVICES => {
            let client = services::Client::new(client);
            if subcommand == LIST {
                return client.list();
            };
        }
        TENANTS => {
            let client = tenants::Client::new(client);
            if subcommand == LIST {
                return client.list();
            };
        }
        _ => {}
    };

    help::print_help_and_exit();
    Ok(())
}
