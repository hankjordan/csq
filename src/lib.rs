use std::{hash::Hash, collections::{VecDeque, HashSet}};

use parking_lot::RwLock;

pub struct RawCSQ<T> {
    queue: VecDeque<T>,
    set: HashSet<T>,
}

impl<T> RawCSQ<T> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            set: HashSet::new(),
        }
    }
}

pub struct ConcurrentSetQueue<T> {
    inner: RwLock<RawCSQ<T>>
}

impl<T> std::fmt::Debug for ConcurrentSetQueue<T> 
where T: std::fmt::Debug + Eq + Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConcurrentSetQueue").finish()
    }
}


impl<T> Default for ConcurrentSetQueue<T> 
where T: Eq + Hash {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ConcurrentSetQueue<T> 
where T: Eq + Hash {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(RawCSQ::new())
        }
    }

    pub fn pop(&self) -> Option<T> {
        let mut w = self.inner.write();

        if let Some(value) = w.queue.pop_front() {
            w.set.remove(&value);
            return Some(value);
        }

        None
    }

    pub fn drain(&self, amount: usize) -> impl Iterator<Item = T> + '_ {
        (0..amount).map_while(|_| {
            if let Some(value) = self.pop() {
                return Some(value);
            }

            None
        })
    }
}

impl<T> ConcurrentSetQueue<T> 
where T: Eq + Hash + Clone {
    pub fn push(&self, value: T) {
        let mut w = self.inner.write();

        if !w.set.contains(&value) {
            w.set.insert(value.clone());
            w.queue.push_back(value);
        }
    }    
}