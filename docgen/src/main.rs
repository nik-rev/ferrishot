//! Generate completions, docs, etc
use std::{fs::File, io::Write, path::PathBuf, sync::LazyLock};

use clap::{Command, CommandFactory, ValueEnum};
use clap_complete::generate_to;
use clap_markdown::MarkdownOptions;
use ferrishot::Cli;

static ROOT: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".."));

fn generate_shell_completions(cmd: &mut Command) {
    let out_dir = ROOT.join("completions");

    std::fs::create_dir_all(&out_dir).unwrap();

    for shell in clap_complete::Shell::value_variants() {
        generate_to(*shell, cmd, "ferrishot", &out_dir).unwrap();
    }
    generate_to(clap_complete_nushell::Nushell, cmd, "ferrishot", &out_dir).unwrap();
    generate_to(carapace_spec_clap::Spec, cmd, "ferrishot", &out_dir).unwrap();
    generate_to(clap_complete_fig::Fig, cmd, "ferrishot", &out_dir).unwrap();

    clap_mangen::generate_to(Cli::command(), out_dir).unwrap();
}

fn main() {
    let mut cmd = Cli::command();

    generate_shell_completions(&mut cmd);

    // markdown help
    File::create(ROOT.join("CLI.md"))
        .unwrap()
        .write_all(
            clap_markdown::help_markdown_custom::<Cli>(&MarkdownOptions::new().show_footer(false))
                .as_bytes(),
        )
        .unwrap();
}
