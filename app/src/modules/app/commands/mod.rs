mod jingle;
mod language;
mod ping;
mod radio;
mod voice;

pub mod commands {
	pub use super::jingle::JINGLE_COMMAND as jingle;
	pub use super::language::LANGUAGE_COMMAND as language;
	pub use super::ping::PING_COMMAND as ping;
	pub use super::radio::RADIO_COMMAND as radio;
	pub use super::voice::join::JOIN_COMMAND as join;
	pub use super::voice::leave::LEAVE_COMMAND as leave;
	pub use super::voice::mute::MUTE_COMMAND as mute;
}
