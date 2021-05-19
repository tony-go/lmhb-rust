pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        let node = Node {
            elem: element,
            next: self.head.take(),
        };

        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|v| {
            self.head = v.next;

            v.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[test]
fn test_list() {
    let mut list = List::new();
    assert_eq!(list.pop(), None);

    list.push(23);
    list.push(34);
    list.push(67);
    assert_eq!(
        list.pop(),
        Some(67)
    );
    assert_eq!(
        list.pop(),
        Some(34)
    );
    assert_eq!(
        list.pop(),
        Some(23)
    );
    assert_eq!(
        list.pop(),
        None
    );
}

fn main() {
    let mut list = List::new();
    list.push(23);
    list.push(43);
    list.push(67);
    list.pop();
}
