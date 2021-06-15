use serenity::framework::standard::macros::group;

mod about;
mod unverified;
mod verify;

use about::ABOUT_COMMAND;
use unverified::UNVERIFIED_COMMAND;
use verify::{ACCEPT_COMMAND, DENY_COMMAND};

#[group]
#[commands(about, unverified, accept, deny)]
pub struct General;
