use terp::cli;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    cli::parse_cli().run();
}
