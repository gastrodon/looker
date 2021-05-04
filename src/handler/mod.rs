mod events;
mod verify;

pub const EMOJI_CHECK: &str = "\u{2705}";
pub const EMOJI_CROSS: &str = "\u{274C}";

pub use events::Handler;
pub use verify::handle_verify;
