struct MyVec<T> {
    data: Box<[T]>,
    size: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            data: Box::new([]),
            size: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        MyVec {
            data: Box::new([Default::default(); capacity]),
            size: 0,
        }
    }

    fn push(&mut self, item: T) {
        if self.size == self.data.len() {
            self.resize(self.size + 1);
        }
        self.data[self.size] = item;
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            Some(std::mem::replace(&mut self.data[self.size], Default::default()))
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.size {
            self.size -= 1;
            let removed_item = std::mem::replace(&mut self.data[index], Default::default());
            for i in index..self.size {
                self.data[i] = std::mem::replace(&mut self.data[i + 1], Default::default());
            }
            Some(removed_item)
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            Some(&self.data[index])
        } else {
            None
        }
    }

    fn resize(&mut self, new_size: usize) {
        let mut new_data: Box<[T]> = Box::new([Default::default(); new_size]);
        let elements_to_copy = std::cmp::min(self.size, new_size);
        new_data[..elements_to_copy].copy_from_slice(&self.data[..elements_to_copy]);
        self.data = new_data;
        self.size = elements_to_copy;
    }
}

