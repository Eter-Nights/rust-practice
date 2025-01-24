use std::ops::Range;

pub struct GapBuffer<T> {
    storage: Vec<T>,
    gap: Range<usize>,
}

impl<T> GapBuffer<T> {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            gap: 0..0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    pub fn len(&self) -> usize {
        self.capacity() - self.gap.len()
    }

    pub fn position(&self) -> usize {
        self.gap.start
    }

    unsafe fn space(&self, index: usize) -> *const T {
        self.storage.as_ptr().offset(index as isize)
    }

    unsafe fn space_mut(&mut self, index: usize) -> *mut T {
        self.storage.as_mut_ptr().offset(index as isize)
    }

    fn index_to_raw(&self, index: usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            index + self.gap.len()
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let raw = self.index_to_raw(index);
        if raw < self.capacity() {
            unsafe { Some(&*self.space(raw)) }
        } else {
            None
        }
    }

    pub fn set_position(&mut self, pos: usize) {
        if pos > self.len() {
            panic!("Index {} out of bounds {}", pos, self.len());
        }

        unsafe {
            let gap = self.gap.clone();
            if pos > gap.start {
                let distance = pos - gap.start;
                std::ptr::copy(self.space(gap.end), self.space_mut(gap.start), distance);
            } else if pos < gap.start {
                let distance = gap.start - pos;
                std::ptr::copy(
                    self.space(pos),
                    self.space_mut(gap.end - distance),
                    distance,
                );
            }
            self.gap = pos..pos + gap.len();
        }
    }

    pub fn insert(&mut self, val: T) {
        if self.gap.len() == 0 {
            self.enlarge_gap()
        }

        unsafe {
            let index = self.gap.start;
            std::ptr::write(self.space_mut(index), val);
        }
        self.gap.start += 1;
    }

    pub fn insert_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.insert(item);
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.gap.end == self.capacity() {
            return None;
        }

        let element = unsafe { std::ptr::read(self.space(self.gap.end)) };
        self.gap.end += 1;
        Some(element)
    }

    fn enlarge_gap(&mut self) {
        let mut new_capacity = self.capacity() * 2;
        if new_capacity == 0 {
            new_capacity = 4;
        }

        let mut new = Vec::with_capacity(new_capacity);
        let after_gap = self.capacity() - self.gap.end;
        let new_gap = self.gap.start..new.capacity() - after_gap;

        unsafe {
            std::ptr::copy_nonoverlapping(self.space(0), new.as_mut_ptr(), self.gap.start);
            let new_gap_end = new.as_mut_ptr().offset(new_gap.end as isize);
            std::ptr::copy_nonoverlapping(self.space(self.gap.end), new_gap_end, after_gap);
        }

        self.storage = new;
        self.gap = new_gap;
    }
}

impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.gap.start {
                std::ptr::drop_in_place(self.space_mut(i));
            }
            for i in self.gap.end..self.capacity() {
                std::ptr::drop_in_place(self.space_mut(i));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut buf = GapBuffer::new();
        buf.insert_iter("Lord of the Rings".chars());
        buf.set_position(12);
    }
}
