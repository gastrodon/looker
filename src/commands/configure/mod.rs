use serenity::framework::standard::macros::group;

mod channels;
mod default;

pub use channels::CHANNELS_COMMAND;
pub use default::CONFIGURE_COMMAND;

#[group]
#[prefix = "config"]
#[commands(channels)]
#[default_command(configure)]
pub struct Configure;
