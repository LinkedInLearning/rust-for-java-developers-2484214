#[derive(Debug)]
pub struct LinkedList<T> {
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, item: T) {
        todo!()
    }

    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    // TODO: optional

    // fn greatest(&self) -> Option<&T>
    // where
    //     T: Ord,
    // {
    //  ..
    // }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(4);
    list.push(5);
    list.push(3);
    list.pop();

    if let Some(node) = &list.head {
        assert_eq!(node.get(), &2);
    }

    // assert_eq!(list.greatest(), Some(&5));
}