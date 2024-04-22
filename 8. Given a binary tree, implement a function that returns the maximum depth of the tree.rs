fn max_depth(root: Option<Box<Node>>) -> i32 {
    match root {
        None => 0
        Some(node) => {
            let x = 1 + max_depth(node.left);

            let y = 1 + max_depth(node.right);

            x.max(y)
        }
    }
}