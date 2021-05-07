use serenity::framework::standard::macros::group;

mod about;
mod unverified;

use about::ABOUT_COMMAND;
use unverified::UNVERIFIED_COMMAND;

#[group]
#[commands(about, unverified)]
pub struct General;
