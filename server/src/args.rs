use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
#[clap(about, version, author)]
pub struct Args {
    // use reverse proxy for web develop server
    #[clap(long, parse(try_from_str = true_or_false), default_value_t = false)]
    pub proxy: bool,

    // path to config.ini
    #[clap(long, default_value_t = String::from("config.ini"))]
    pub config: String,
}

fn true_or_false(s: &str) -> Result<bool, &'static str> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("expected `true` or `false`"),
    }
}
