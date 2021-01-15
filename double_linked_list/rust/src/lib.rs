#[derive(serde::Serialize)]
struct Node {
    next: Option<Box<Node>>,
    previous: Option<Box<Node>>,
    value: i32,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            next: None,
            previous: None,
            value,
        }
    }
}

#[derive(serde::Serialize)]
struct DoubleLinkedList {
    length: usize,
    head: Option<Box<Node>>,
}

impl DoubleLinkedList {
    fn new() -> DoubleLinkedList {
        DoubleLinkedList {
            length: 0,
            head: None,
        }
    }

    fn add(&mut self, node: Node) {
        self.length += 1;

        if let Some(head) = &self.head {
            head.previous = Some(Box::new(node));
            node.next = Some(*head);
            self.head = Some(Box::new(node));
        } else {
            self.head = Some(Box::new(node));
        }
    }
}

#[cfg(test)]
mod tests;
