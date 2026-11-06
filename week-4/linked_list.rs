struct Node {
    data: i32,
    next: Option<usize>,
}

struct LinkedList {
    nodes: Vec<Node>,
    head: Option<usize>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            nodes: Vec::new(),
            head: None,
        }
    }
    fn display(&self){
        let mut current = self.head;
        while let Some(index) = current {
            print!("{} -> ", self.nodes[index].data);
            current = self.nodes[index].next;
        }
        println!("None");
    }

    fn insert(&mut self, mut node: Node) {
        let temp = self.head;
        let index = self.nodes.len();

        node.next = temp;
        self.nodes.push(node);
        self.head = Some(index);
    }
    fn delete(&mut self, data: i32) {
        let mut current = self.head;
        let mut prev: Option<usize> = None;

        while let Some(index) = current {
            if self.nodes[index].data == data {
                if let Some(prev_index) = prev {
                    self.nodes[prev_index].next = self.nodes[index].next;
                } else {
                    self.head = self.nodes[index].next;
                }
                return;
            }
            prev = current;
            current = self.nodes[index].next;
        }


    }
    
}

fn main() {
    let mut list = LinkedList::new();

    list.insert(Node { data: 10, next: None });
    list.insert(Node { data: 20, next: None });
    list.insert(Node { data: 30, next: None });

    list.delete(20);

    list.display();
}