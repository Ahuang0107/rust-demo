// Definition for singly-linked list.

// Constraints:
//
// 1. The number of nodes in each linked list is in the range [1, 100].
// 2. 0 <= Node.val <= 9
// 3. It is guaranteed that the list represents a number that does not have leading zeros.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn from_slice(nums: &[i32]) -> Option<Box<ListNode>> {
        // 使用迭代器逆序遍历数组，构造链表
        nums.iter()
            .rev()
            .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
    }
    pub fn to_vec(&self) -> Vec<i32> {
        let mut result = vec![];
        self.collect_val(&mut result);
        result
    }
    pub fn collect_val(&self, collection: &mut Vec<i32>) {
        collection.push(self.val);
        if let Some(next) = &self.next {
            next.collect_val(collection);
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let rev_l1 = l1.unwrap().to_vec().into_iter().rev().collect::<Vec<i32>>();
        let rev_l2 = l2.unwrap().to_vec().into_iter().rev().collect::<Vec<i32>>();
        let mut index = 0;
        let mut rev_result = vec![];
        let mut carry = 0;
        while rev_l1.len() > index && rev_l2.len() > index {
            let a = rev_l1[index];
            let b = rev_l2[index];
            let sum = a + b + carry;
            carry = sum / 10;
            rev_result.push(sum % 10);
            index += 1;
        }
        while rev_l1.len() > index {
            let sum = rev_l1[index] + carry;
            carry = sum / 10;
            rev_result.push(sum % 10);
            index += 1;
        }
        while rev_l2.len() > index {
            let sum = rev_l2[index] + carry;
            carry = sum / 10;
            rev_result.push(sum % 10);
            index += 1;
        }
        if carry > 0 {
            rev_result.push(carry);
        }
        let result = rev_result.into_iter().rev().collect::<Vec<i32>>();
        ListNode::from_slice(result.as_slice())
    }
}

#[macro_export]
macro_rules! test_add_two_numbers {
    ($input1:expr, $input2:expr, $expected:expr) => {
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_slice($input1), ListNode::from_slice($input2))
                .unwrap()
                .to_vec(),
            $expected,
            "get error output with input({:?},{:?})",
            $input1,
            $input2
        );
    };
}

fn main() {
    test_add_two_numbers!(&[7, 2, 4, 3], &[5, 6, 4], vec![7, 8, 0, 7]);
    test_add_two_numbers!(&[2, 4, 3], &[5, 6, 4], vec![8, 0, 7]);
    test_add_two_numbers!(&[0], &[0], vec![0]);
    test_add_two_numbers!(&[5], &[5], vec![1, 0]);
}
