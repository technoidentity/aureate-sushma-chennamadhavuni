
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn print_list(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = node.next.as_ref();
        }

        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_front(10);
    list.push_front(20);

    list.print_list();
}
