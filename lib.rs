use std::fmt;
use chrono::{DateTime, TimeZone, Utc};

// Define the TreeNode struct
pub struct TreeNode {
    timestamp: u64,
    heart_rate: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(timestamp: u64, heart_rate: u32) -> Self {
        TreeNode {
            timestamp,
            heart_rate,
            left: None,
            right: None,
        }
    }
}

// Define the HeartRateTree struct
pub struct HeartRateTree {
    pub root: Option<Box<TreeNode>>, //This field is public
}

impl HeartRateTree {
    pub fn new() -> Self {
        HeartRateTree { root: None }
    }

    pub fn insert(&mut self, timestamp: u64, heart_rate: u32) {
        let new_node = Box::new(TreeNode::new(timestamp, heart_rate));
        match self.root {
            Some(ref mut node) => Self::insert_node(node, new_node),
            None => self.root = Some(new_node),
        }
    }

    fn insert_node(current: &mut Box<TreeNode>, new_node: Box<TreeNode>) {
        if new_node.timestamp < current.timestamp {
            match current.left {
                Some(ref mut left) => Self::insert_node(left, new_node),
                None => current.left = Some(new_node),
            }
        } else {
            match current.right {
                Some(ref mut right) => Self::insert_node(right, new_node),
                None => current.right = Some(new_node),
            }
        }
    }

    pub fn average_last_minute(&self, current_time: u64) -> f32 {
        let start_time = current_time.saturating_sub(60);
        let (sum, count) = Self::sum_and_count(self.root.as_ref(), start_time, current_time);
        if count == 0 {
            0.0
        } else {
            sum as f32 / count as f32
        }
    }

    fn sum_and_count(node: Option<&Box<TreeNode>>, start: u64, end: u64) -> (u32, u32) {
        match node {
            Some(n) => {
                let mut sum = 0;
                let mut count = 0;
                if n.timestamp >= start && n.timestamp <= end {
                    sum += n.heart_rate;
                    count += 1;
                }
                let (left_sum, left_count) = Self::sum_and_count(n.left.as_ref(), start, end);
                let (right_sum, right_count) = Self::sum_and_count(n.right.as_ref(), start, end);
                sum += left_sum + right_sum;
                count += left_count + right_count;
                (sum, count)
            }
            None => (0, 0),
        }
    }

    // In-order traversal to collect nodes (Needed for tests, do not delete!)
    pub fn inorder_traversal(&self, node: &Option<Box<TreeNode>>, result: &mut Vec<(u64, u32)>) {
        if let Some(ref n) = node {
            self.inorder_traversal(&n.left, result);
            result.push((n.timestamp, n.heart_rate));
            self.inorder_traversal(&n.right, result);
        }
    }

    pub fn get_root(&self) -> &Option<Box<TreeNode>> {
        &self.root
    }

    fn display_node(&self, node: &Option<Box<TreeNode>>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref n) = node {
            self.display_node(&n.left, f)?;
            let datetime = Utc.timestamp(n.timestamp as i64, 0);
            writeln!(f, "{} - {}", datetime.to_rfc3339(), n.heart_rate)?;
            self.display_node(&n.right, f)?;
        }
        Ok(())
    }
}

// Implementing Display trait for TreeNode
impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = Utc.timestamp(self.timestamp as i64, 0);
        write!(f, "{} - {}", datetime.to_rfc3339(), self.heart_rate)
    }
}

// Implementing Display trait for HeartRateTree
impl fmt::Display for HeartRateTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_node(&self.root, f)
    }
}
