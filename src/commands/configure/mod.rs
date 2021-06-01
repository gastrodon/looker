use serenity::framework::standard::macros::group;

mod channels;
mod default;
mod verify;

pub use channels::CHANNELS_COMMAND;
pub use default::CONFIGURE_COMMAND;
pub use verify::{UNVERIFY_COMMAND, VERIFY_COMMAND};

#[group]
#[prefix = "config"]
#[commands(channels, verify, unverify)]
#[default_command(configure)]
pub struct Configure;
