mod server;
mod completion;
mod hover;
mod goto;
mod rename;
mod diagnostics;
mod formatting;
mod workspace;

pub use server::LspServer;

pub fn run() {
    let server = LspServer::new();
    server.start();
}
