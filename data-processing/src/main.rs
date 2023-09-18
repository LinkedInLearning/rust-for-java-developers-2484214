trait DataPipeline {
    fn insert(&mut self, item: T);
    
	fn insert_many(&mut self, items: impl Iterator<Item = T>);
    
	fn num_valid(&self) -> u32;
    
	fn filter(&mut self, predicate: fn(&T) -> bool);
}

struct Validator<T>;

fn main() {
    let mut vals = Validator::<i32> {};
    vals.insert(1);
    vals.insert(10);
    vals.insert(100);
    vals.filter(|&x| x < 50);

    assert_eq!(vals.num_valid(), 2);
}
