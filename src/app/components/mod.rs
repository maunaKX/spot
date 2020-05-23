use crate::app::{AppAction};

pub mod playback;
pub use playback::{Playback, PlaybackModel};

pub mod playlist;
pub use playlist::{Playlist, PlaylistModel};

pub mod login;
pub use login::{Login};

pub mod player;
pub use player::{Player};

mod gtypes;

pub trait Component {
    fn handle(&self, action: &AppAction);
}
