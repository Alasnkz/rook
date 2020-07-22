pub struct Ring<T: Copy> {
    buffer: Vec<T>,
    capacity: usize,
    playhead: usize,
}

impl<T: Copy> Ring<T> {
    pub fn new(capacity: usize) -> Ring<T> {
        Ring {
            buffer: Vec::with_capacity(capacity),
            capacity: capacity,
            playhead: 0,
        }
    }

    pub fn insert(&mut self, item: T) {
        if self.buffer.len() == self.capacity {
            self.buffer[self.playhead] = item;
        } else {
            self.buffer.push(item);
        }
        self.playhead += 1;
        if self.playhead == self.capacity {
            self.playhead = 0;
        }
    }

    pub fn unroll(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        let playhead = self.playhead;

        self.buffer[playhead..]
            .into_iter()
            .for_each(|i: &T| result.push(i.clone()));
        self.buffer[..playhead]
            .into_iter()
            .for_each(|i: &T| result.push(i.clone()));

        result
    }
}

#[test]
fn test_ring_buffer() {
    let mut r = Ring::new(3);
    assert_eq!(r.playhead, 0);
    r.insert(1);
    assert_eq!(r.playhead, 1);
    r.insert(2);
    assert_eq!(r.playhead, 2);
    r.insert(3);
    assert_eq!(r.playhead, 0);
    r.insert(1);
    assert_eq!(r.playhead, 1);
    r.insert(2);
    assert_eq!(r.playhead, 2);
    r.insert(3);
    assert_eq!(r.playhead, 0);
}
