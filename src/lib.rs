use parking_lot::Mutex;
use crossbeam_queue::SegQueue;
use dashmap::DashSet;


pub struct ConSetQueue<T> {
    lock: Mutex<()>,
    set: DashSet<Arc<T>>,
    queue: SegQueue<Arc<T>>,
}

impl<T> std::fmt::Debug for ConSetQueue<T>
where T: std::fmt::Debug + Eq + Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConSetQueue").field("set", &self.set).field("queue", &self.queue).finish()
    }
}

impl<T> Default for ConSetQueue<T>
where T: Eq + Hash {
    fn default() -> Self {
        Self {
            lock: Mutex::default(),
            set: DashSet::default(),
            queue: SegQueue::default(),
        }
    }
}

impl<T> ConSetQueue<T> 
where T: Eq + Hash {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn push(&self, value: T) {
        let _guard = self.lock.lock().unwrap();

        if !self.set.contains(&value) {
            let ptr = Arc::new(value);

            self.queue.push(ptr.clone());
            self.set.insert(ptr);
        }
    }

    pub fn pop(&self) -> Option<T> {
        let _guard = self.lock.lock().unwrap();

        if let Some(ptr) = self.queue.pop() {
            self.set.remove(&ptr);

            match Arc::try_unwrap(ptr) {
                Ok(value) => { return Some(value) }
                Err(_) => { panic!() }
            }
        }

        return None;
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
