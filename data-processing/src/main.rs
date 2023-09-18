trait DataPipeline<T> {
    fn insert(&mut self, item: T);

    fn insert_many(&mut self, items: impl Iterator<Item = T>);

    fn num_valid(&self) -> u32;

    fn filter(&mut self, predicate: fn(&T) -> bool);
}

struct Validator<T>;

impl<T> Validator<T> {
    fn new() -> Self {
        todo!()
    }
}

impl<T> DataPipeline<T> for Validator<T> {
    fn insert(&mut self, item: T) {
        todo!()
    }

    fn insert_many(&mut self, items: impl Iterator<Item = T>) {
        todo!()
    }

    fn num_valid(&self) -> u32 {
        todo!()
    }

    fn filter(&mut self, predicate: fn(&T) -> bool) {
        todo!()
    }
}

fn main() {
    let mut vals = Validator::<i32>::new();
    vals.insert(1);
    vals.insert(10);
    vals.insert(100);
    vals.filter(|&x| x < 50);

    assert_eq!(vals.num_valid(), 2);
}
