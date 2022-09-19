use clap::{Arg, Command};

use trg_analzyer::server::run_server;

#[tokio::main]
async fn main() {
  let _matches = Command::new("")
    .arg(Arg::new("stdio")
      .long("stdio")
      .required(true)
      .help("Use std io for TriggerReactor language server")
    )
    .get_matches();

  let stdio = tokio::io::stdin();
  let stdout = tokio::io::stdout();

  run_server(stdio, stdout).await
}
