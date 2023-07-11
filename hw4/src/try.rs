struct MyVec {
    data: [i32; 10]
}

impl MyVec {
    fn new() -> Self {
        MyVec {
            data: [0; 10]
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        MyVec {
            data: [0; usize]
        }
    }

    fn push(&mut self, item: i32) {
        assert!(self.size < 10, "Vector capacity exceeded");
        self.data[self.size] = item;
        self.size += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.data[self.size])
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<i32> {
        if index < self.size {
            let removed_item = self.data[index];
            for i in index..(self.size - 1) {
                self.data[i] = self.data[i + 1];
            }
            self.size -= 1;
            Some(removed_item)
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<i32> {
        if index < self.size {
            Some(self.data[index])
        } else {
            None
        }
    }

    fn resize(&mut self, new_size: usize) {
        assert!(new_size <= 10, "Resize exceeds maximum size of 10");
        if new_size < self.size {
            self.size = new_size;
        }
    }
}

fn main() {
    let mut my_vec: MyVec = MyVec::new();
    my_vec.push(10);
    my_vec.push(20);
    my_vec.push(30);

    println!("MyVec: {:?}", my_vec);
    println!("Pop: {:?}", my_vec.pop());
    println!("Remove index 0: {:?}", my_vec.remove(0));
    println!("Get index 1: {:?}", my_vec.get(1));
    my_vec.resize(2);

    println!("MyVec after resizing: {:?}", my_vec);
}
