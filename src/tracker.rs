pub struct Tracker<T> {
    distance: Track<T>,
    tracks: Vec<Track<T>>,
}

impl<T> Tracker<T>
where
    T: Clone + Copy,
{
    pub fn new(start: T) -> Self {
        Self {
            distance: Track::new(start),
            tracks: vec![],
        }
    }

    pub fn get_full_track(&self) -> Track<T> {
        let mut full_track = self.distance;

        for track in &self.tracks {
            full_track.line_index += track.line_index;
            full_track.char_index += track.char_index;
        }
        full_track
    }

    pub fn get_track(&self) -> &Track<T> {
        if self.tracks.is_empty() {
            &self.distance
        } else {
            self.tracks.last().unwrap()
        }
    }

    /// connect last two tracks
    ///
    /// # Return
    ///
    /// - True if suceeded to merge
    /// - false if not
    pub fn connect_track(&mut self) -> bool {
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

    fn get_last_track_mut(&mut self) -> &mut Track<T> {
        if self.tracks.is_empty() {
            &mut self.distance
        } else {
            self.tracks.last_mut().unwrap()
        }
    }

    pub fn set_milestone(&mut self, milestone: T) {
        let track = Track::new(milestone);
        self.tracks.push(track);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Track<T> {
    pub line_index: usize,
    pub char_index: usize,
    pub milestone: T,
}

impl<T> Track<T> {
    pub fn new(milestone: T) -> Self {
        Self {
            line_index: 0,
            char_index: 0,
            milestone,
        }
    }

    pub fn merge(&mut self, track: Track<T>) {
        self.line_index += track.line_index;
        self.char_index += track.char_index;
    }
}
