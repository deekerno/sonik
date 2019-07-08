use rand::thread_rng;
use std::collections::VecDeque;

use crate::database::record::Track;

// Real requirement for shuffle
trait LenAndSwap {
    fn len(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
}

// An exact copy of rand::Rng::shuffle, with the signature modified to
// accept any type that implements LenAndSwap
fn shuffle<T, R>(values: &mut T, mut rng: R)
where
    T: LenAndSwap,
    R: rand::Rng,
{
    let mut i = values.len();
    while i >= 2 {
        // invariant: elements with index >= i have been locked in place.
        i -= 1;
        // lock element i in place.
        values.swap(i, rng.gen_range(0, i + 1));
    }
}

// VecDeque trivially fulfills the LenAndSwap requirement, but
// we have to spell it out.
impl<T> LenAndSwap for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.swap(i, j)
    }
}

pub struct SonikQueue {
    pub tracks: VecDeque<Track>,
    pub total_time: u32,
}

impl SonikQueue {
    pub fn new() -> SonikQueue {
        SonikQueue {
            tracks: VecDeque::<Track>::new(),
            total_time: 0,
        }
    }

    pub fn add(&mut self, track: Track) {
        self.total_time += &track.duration;
        self.tracks.push_back(track);
    }

    pub fn add_to_front(&mut self, track: Track) {
        self.total_time += &track.duration;
        self.tracks.push_front(track);
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.total_time = 0;
    }

    pub fn shuffle(&mut self) {
        shuffle(&mut self.tracks, thread_rng());
    }

    pub fn is_empty(&mut self) -> bool {
        self.tracks.is_empty()
    }

    pub fn take(&mut self) -> Track {
        let track = self.tracks.pop_front().unwrap();
        self.total_time -= &track.duration;

        track
    }
}
