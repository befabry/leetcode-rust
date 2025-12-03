// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>, // You own list1 now
    list2: Option<Box<ListNode>>, // You own list2 now
) -> Option<Box<ListNode>> {
    let mut merged = vec![];

    // Take ownership of list1 - no & needed
    let mut list1_owned = list1;
    while let Some(node) = list1_owned {
        merged.push(node.val);
        list1_owned = node.next; // Move to next node (taking ownership)
    }

    // Same for list2
    let mut list2_owned = list2;
    while let Some(node) = list2_owned {
        merged.push(node.val);
        list2_owned = node.next;
    }

    merged.sort();

    let mut next = None;
    for &val in merged.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = next;
        next = Some(Box::new(node));
    }

    next
}
