#![allow(unused)]

struct Counter {
    count: u32,
}

impl Counter {
    fn new(init: u32) -> Counter {
        Counter { count: init }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 1 {
            None
        } else if self.count % 2 == 0 {
            self.count /= 2;
            Some(self.count)
        } else {
            self.count = 3 * self.count + 1;
            Some(self.count)
        }
    }
}

fn main() {
    let mut count = Counter::new(15);
    let mut vec = Vec::new();

    for i in count {
        vec.push(i.pow(2));
    }

    println!("{:?}", vec);
}
