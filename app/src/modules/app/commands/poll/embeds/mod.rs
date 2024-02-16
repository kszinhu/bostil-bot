mod setup;
mod vote;

#[allow(unused_imports)]
pub mod embeds {
    pub use super::setup::SETUP_EMBED;
    pub use super::vote::VOTE_EMBED;
}
