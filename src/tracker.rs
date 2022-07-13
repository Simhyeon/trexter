/// Main entry for tracking progression
#[derive(Debug)]
pub struct Tracker<T> {
    distance: Track<T>,
    tracks: Vec<Track<T>>,
}

impl<T> Tracker<T> {
    pub fn new(start: T) -> Self {
        Self {
            distance: Track::new(start),
            tracks: vec![],
        }
    }

    pub fn get_distance(&self) -> &Track<T> {
        &self.distance
    }

    pub fn get_track_counts(&self) -> usize {
        self.tracks.len()
    }

    pub fn get_full_track(&self) -> Track<&T> {
        let mut full_track = Track::new(&self.distance.milestone);
        full_track.line_index = self.distance.line_index;
        full_track.char_index = self.distance.char_index;

        for track in &self.tracks {
            full_track.line_index += track.line_index;
            full_track.char_index = track.char_index;
        }
        full_track
    }

    pub fn get_sub_track(&self, reverse_index: usize) -> Track<&T> {
        let mut full_track = Track::new(&self.distance.milestone);
        full_track.line_index = self.distance.line_index;
        full_track.char_index = self.distance.char_index;

        let max = self.tracks.len();
        let end = max - (reverse_index).min(max);

        for track in &self.tracks[0..end] {
            full_track.line_index += track.line_index;
            full_track.char_index = track.char_index;
        }
        full_track
    }

    pub fn get_track(&self) -> Track<&T> {
        if self.tracks.is_empty() {
            Track::new(&self.distance.milestone)
        } else {
            Track::new(&self.tracks.last().unwrap().milestone)
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
            self.distance.merge(&last_track);
            true
        } else {
            self.get_last_track_mut().merge(&last_track);
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

    pub fn new_track(&mut self, milestone: T) {
        let mut track = Track::new(milestone);
        track.line_index = 0;
        self.tracks.push(track);
    }
}

/// Tracking unit
#[derive(Debug)]
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

    pub fn merge(&mut self, track: &Track<T>) {
        self.line_index += track.line_index;
        if self.line_index == track.line_index {
            self.char_index += track.char_index;
        } else {
            self.char_index = track.char_index;
        }
    }
}
