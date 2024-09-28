use crate::commands::*;
use colored::Colorize;

const TITLE: &str = "KUMA command line interface";
const INIT_SHORT_HELP: &str =
    "Initialize config. It is recommented to run it before using kuma-control";

pub fn print_help_and_exit() {
    help();
    std::process::exit(1);
}

pub fn print_help_init_and_exit() {
    help_init();
    std::process::exit(1);
}

pub fn print_help_tenants_and_exit() {
    help_tenants();
    std::process::exit(1);
}

pub fn print_help_services_and_exit() {
    help_services();
    std::process::exit(1);
}

pub fn print_help_assets_and_exit() {
    help_assets();
    std::process::exit(1);
}

fn print_title() {
    println!("{}", TITLE);
    println!();
}

pub fn help() {
    print_title();

    println!(
        "{} {} <COMMAND>",
        title("Usage:"),
        std::env::args().next().unwrap()
    );
    println!();

    println!("{}", title("Commands:"));
    println!("  {}\t\t{}", INIT, INIT_SHORT_HELP,);
    println!("  {}\tAssets commands", ASSETS);
    println!("  {}\tEvents commands", EVENTS);
    println!("  {}\tResources commands", RESOURCES);
    println!("  {}\tServices commands", SERVICES);
    println!("  {}\tTenants commands", TENANTS);
    println!("  help\t\tPrint this message or the help of the given subcommand(s)");
    println!();

    println!("{}", title("Options:"));
    println!("  -h, --help  Print help");
}

pub fn help_init() {
    print_title();

    println!("{}\t{}", INIT, INIT_SHORT_HELP);
}

pub fn help_tenants() {
    print_title();

    println_subcommand_usage(TENANTS);

    println!("  {}\t\tPrint {} list", LIST, TENANTS);
}

pub fn help_services() {
    print_title();

    println_subcommand_usage(SERVICES);

    println!("  {}\t\tPrint {} list", LIST, SERVICES);
}

pub fn help_assets() {
    print_title();

    println_subcommand_usage(ASSETS);

    println!("  {}\t\tPrint {} list", LIST, ASSETS);
    println!(
        "  {}\tImport assets list. It reads from stdin json format or use paramaters from command line.\n\t\t\
            For example, next command creates one asset {}\n\t\t\
            Possible fields: name=\"example-name\", fqdn=fqdn1,fqdn2,fqdn3, ipAddresses=1.1.1.1,2.2.2.2,3.3.3.3, owner=somebody",
        IMPORT,
        "kc assets import name=example-name fqdn=example.com"
            .bold()
            .bright_white(),
    );
}

fn println_subcommand_usage(name: &str) {
    println!("{} {} <subcommands>", title("Usage:"), name);
    println!();
}

fn title(s: &str) -> colored::ColoredString {
    s.bold().underline()
}
