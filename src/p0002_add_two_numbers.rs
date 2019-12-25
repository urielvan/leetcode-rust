use utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut res;
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut pre: i32 = 0;

        loop {
            let mut val: i32 = pre;

            if p1.is_some() {
                val += p1.unwrap().val;
                p1 = p1.unwrap().next.as_ref();
            }

            if p2.is_some() {
                val += p2.unwrap().val;
                p2 = p2.unwrap().next.as_ref();
            }

            pre = val / 10;
            val %= 10;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
            tail = &mut tail.as_mut().unwrap().next;

            if p1.is_none() && p2.is_none() && pre == 0 {
                break;
            }
        }

        res.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use utils::linked_list::to_list;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );
    }
}
