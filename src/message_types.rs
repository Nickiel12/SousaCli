use serde::{Deserialize, Serialize};

/// A struct that defines all the music tags supported by Sousa
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemTag {
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
}

impl Default for ItemTag {
    fn default() -> Self {
        ItemTag {
            path: String::new(),
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            album_artist: String::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PartialTag {
    pub path: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
}

impl Default for PartialTag {
    fn default() -> Self {
        PartialTag {
            path: None,
            title: None,
            artist: None,
            album: None,
            album_artist: None,
        }
    }
}

impl PartialTag {
    pub fn has_path(self: &Self) -> bool {
        self.path.is_some()
    }

    pub fn has_title(self: &Self) -> bool {
        self.title.is_some()
    }

    pub fn has_artist(self: &Self) -> bool {
        self.artist.is_some()
    }

    pub fn has_album(self: &Self) -> bool {
        self.album.is_some()
    }

    pub fn has_album_artist(self: &Self) -> bool {
        self.album_artist.is_some()
    }

    pub fn is_empty(self: &Self) -> bool {
        return self.path.is_none()
            && self.title.is_none()
            && self.artist.is_none()
            && self.album.is_none()
            && self.album_artist.is_none();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerResponse {
    pub message: String,
    pub search_results: Vec<ItemTag>,
}

#[derive(Serialize, Deserialize)]
pub enum SkipDirection {
    Forward,
    Backward,
}

#[derive(Serialize, Deserialize)]
pub enum UIRequest {
    Play,
    Pause,
    Skip(SkipDirection),
    Search(PartialTag),
    SwitchTo(PartialTag),
    GetStatus,
}

pub fn itemtag_to_partial(itemtag: &ItemTag) -> PartialTag {
    let mut partial_tag = PartialTag {
        ..PartialTag::default()
    };

    if itemtag.path.is_empty() {
        partial_tag.path = None;
    } else {
        partial_tag.path = Some(itemtag.path.clone());
    }

    if itemtag.title.is_empty() {
        partial_tag.title = None;
    } else {
        partial_tag.title = Some(itemtag.title.clone());
    }

    if itemtag.album.is_empty() {
        partial_tag.album = None;
    } else {
        partial_tag.album = Some(itemtag.album.clone());
    }

    if itemtag.artist.is_empty() {
        partial_tag.artist = None;
    } else {
        partial_tag.artist = Some(itemtag.artist.clone());
    }

    if itemtag.album_artist.is_empty() {
        partial_tag.album_artist = None;
    } else {
        partial_tag.album_artist = Some(itemtag.album_artist.clone());
    }

    return partial_tag;
}
