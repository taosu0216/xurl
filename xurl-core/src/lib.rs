pub mod error;
pub mod model;
pub mod provider;
pub mod render;
pub mod service;
pub mod uri;

pub use error::{Result, XurlError};
pub use model::{
    MessageRole, PiEntryListView, ProviderKind, ResolutionMeta, ResolvedThread, SubagentDetailView,
    SubagentListView, SubagentView, ThreadMessage,
};
pub use provider::ProviderRoots;
pub use service::{
    render_pi_entry_list_markdown, render_subagent_view_markdown, render_thread_markdown,
    resolve_pi_entry_list_view, resolve_subagent_view, resolve_thread,
};
pub use uri::ThreadUri;
