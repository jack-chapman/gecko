#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::missing_errors_doc
)]
mod document;
mod editor;
mod row;
mod terminal;

use editor::Editor;

fn main() {
    Editor::default().run();
}
