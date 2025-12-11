use wot_core::types::{Relation, WoTGraph};

// TODO::: Is there any better number for mute and follow weights?

pub struct Follow;

impl Follow {
    pub fn relation() -> Relation {
        Relation::new("follow", 1.0).unwrap()
    }
}

pub struct Mute;

impl Mute {
    pub fn relation() -> Relation {
        Relation::new("mute", -1.0).unwrap()
    }
}

// TODO::: We may change String to nostr::Npub?
pub type NostrWoTGraph = WoTGraph<String>;
