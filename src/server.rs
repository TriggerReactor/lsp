use tokio::io::{Stdin, Stdout};
use tower_lsp::{Client, LspService, LanguageServer, Server, lsp_types::{InitializeParams, InitializeResult, MessageType, InitializedParams}, jsonrpc};

struct Backend {
  client: Client,
}

impl Backend {
  /// Creates a new [`Server`] instance
  fn new(client: Client) -> Self {
    Self { client }
  }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
  async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
    Ok(InitializeResult::default())
  }

  async fn initialized(&self, _: InitializedParams) {
    self.client
      .log_message(MessageType::INFO, "TriggerReactor language server initialized!")
      .await;
  }

  async fn shutdown(&self) -> jsonrpc::Result<()> {
    Ok(())
  }
}

pub async fn run_server(stdin: Stdin, stdout: Stdout) {
  // TODO
  let (service, socket) = LspService::build(Backend::new).finish();
  Server::new(stdin, stdout, socket).serve(service).await;
}
