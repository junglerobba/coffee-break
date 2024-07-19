use clap::Command;
use clap_complete::{generate, Generator};

pub(crate) fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    let name = cmd
        .get_bin_name()
        .unwrap_or_else(|| cmd.get_name())
        .to_string();
    generate(gen, cmd, name, &mut std::io::stdout());
}
