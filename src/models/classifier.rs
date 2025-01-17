pub struct DecisionTree {
    _min_samples: u32,
    _max_depth: u32,
}

pub struct TreeNode {
    _feature: u32,
    _threshold: f64,
    _left: TreeNodes,
    _right: TreeNodes,
    _gain: f64,
    _value: u32,
}

pub struct NullNode {}

pub enum TreeNodes {
    TreeNode,
    NullNode,
}
