#[derive(Default)]
pub struct Tracker {
    distance: Track,
    tracks: Vec<Track>,
}

impl Tracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_full_track(&self) -> Track {
        let mut full_track = self.distance;

        for track in &self.tracks {
            full_track.line_index += track.line_index;
            full_track.char_index += track.char_index;
        }
        full_track
    }

    pub fn get_track(&self) -> Track {
        *self.get_last_track()
    }

    /// Merge last two tracks
    ///
    /// # Return
    ///
    /// - True if suceeded to merge
    /// - false if not
    pub fn merge_track(&mut self) -> bool {
        if self.tracks.is_empty() {
            return false;
        }

        let last_track = self.tracks.pop().unwrap();

        // There is still offet remaining
        if self.tracks.is_empty() {
            // Only basic distance is left
            self.distance.merge(last_track);
            true
        } else {
            self.get_last_track_mut().merge(last_track);
            true
        }
    }

    pub fn forward_line(&mut self) {
        let track = self.get_last_track_mut();
        track.line_index += 1;
        track.char_index = 0;
    }

    pub fn forward_char(&mut self) {
        let track = self.get_last_track_mut();
        track.char_index += 1;
    }

    fn get_last_track_mut(&mut self) -> &mut Track {
        if self.tracks.is_empty() {
            &mut self.distance
        } else {
            self.tracks.last_mut().unwrap()
        }
    }

    fn get_last_track(&self) -> &Track {
        if self.tracks.is_empty() {
            &self.distance
        } else {
            self.tracks.last().unwrap()
        }
    }

    pub fn set_milestone(&mut self) {
        let offset = Track::new();
        self.tracks.push(offset);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Track {
    pub line_index: usize,
    pub char_index: usize,
}

impl Track {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, track: Track) {
        self.line_index += track.line_index;
        self.char_index += track.char_index;
    }
}
