use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct PriorityQueue<T> {
    queues: Vec<VecDeque<T>>,
}

impl<T> PriorityQueue<T> {
    pub fn new(max_priority: u8) -> Self {
        PriorityQueue {
            queues: (0..=max_priority).map(|_| VecDeque::new()).collect(),
        }
    }

    pub fn push(&mut self, priority: u8, value: T) {
        self.queues[priority as usize].push_back(value);
    }

    pub fn pop(&mut self) -> Option<(u8, T)> {
        for (i, queue) in self.queues.iter_mut().enumerate().rev() {
            if let Some(value) = queue.pop_front() {
                return Some((i as u8, value));
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.queues.iter().map(|q| q.len()).sum()
    }
}
