use std::process::ExitCode;

use clap::Parser;
use xurl_core::{
    ProviderKind, ProviderRoots, ThreadUri, XurlError, render_pi_entry_list_markdown,
    render_subagent_view_markdown, render_thread_markdown, resolve_pi_entry_list_view,
    resolve_subagent_view, resolve_thread,
};

#[derive(Debug, Parser)]
#[command(name = "xurl", version, about = "Resolve and read code-agent threads")]
struct Cli {
    /// Thread URI like agents://codex/<session_id>, agents://claude/<session_id>, agents://pi/<session_id>/<entry_id>, or legacy forms like codex://<session_id>
    uri: String,

    /// List subagents for a main thread URI
    /// For Pi, list session entries for agents://pi/<session_id> (legacy pi://<session_id> works too)
    #[arg(long)]
    list: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> xurl_core::Result<()> {
    let roots = ProviderRoots::from_env_or_home()?;
    let uri = ThreadUri::parse(&cli.uri)?;
    let supports_subagent = matches!(
        uri.provider,
        xurl_core::ProviderKind::Codex | xurl_core::ProviderKind::Claude
    );

    if cli.list && uri.provider == ProviderKind::Pi {
        let view = resolve_pi_entry_list_view(&uri, &roots)?;
        let markdown = render_pi_entry_list_markdown(&view);
        print!("{markdown}");
        return Ok(());
    }

    if cli.list || (supports_subagent && uri.agent_id.is_some()) {
        if cli.list && uri.agent_id.is_some() {
            return Err(XurlError::InvalidMode(
                "--list cannot be used with child thread URIs like agents://<provider>/<main_thread_id>/<agent_id>".to_string(),
            ));
        }

        let view = resolve_subagent_view(&uri, &roots, cli.list)?;

        let markdown = render_subagent_view_markdown(&view);
        print!("{markdown}");
        return Ok(());
    }

    let resolved = resolve_thread(&uri, &roots)?;
    let markdown = render_thread_markdown(&uri, &resolved)?;
    print!("{markdown}");

    Ok(())
}
