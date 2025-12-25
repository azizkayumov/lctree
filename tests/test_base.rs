use lctree::{FindMax, LinkCutTree};

#[test]
pub fn basic_usage() {
    // We form a link-cut tree for the following forest:
    // (the numbers in parentheses are the weights of the nodes):
    //            a(9)
    //           /    \
    //         b(1)    e(2)
    //        /   \      \
    //      c(8)  d(10)   f(4)
    let mut lctree: LinkCutTree<FindMax> = LinkCutTree::new();
    let a = lctree.make_tree(9.);
    let b = lctree.make_tree(1.);
    let c = lctree.make_tree(8.);
    let d = lctree.make_tree(10.);
    let e = lctree.make_tree(2.);
    let f = lctree.make_tree(4.);

    lctree.link(b, a);
    lctree.link(c, b);
    lctree.link(d, b);
    lctree.link(e, a);
    lctree.link(f, e);

    // Checking connectivity:
    assert!(lctree.connected(c, f)); // connected

    // Path aggregation:
    // We find the node with max weight on the path between c to f,
    // where a has the maximum weight of 9.0:
    let heaviest_node = lctree.path(c, f);
    assert_eq!(heaviest_node.idx, a);
    assert_eq!(heaviest_node.weight, 9.0);

    // We cut node e from its parent a:
    lctree.cut(e, a);

    // The forest should now look like this:
    //            a(9)
    //           /
    //         b(1)      e(2)
    //        /   \        \
    //      c(8)  d(10)    f(4)

    // We check connectivity again:
    assert!(!lctree.connected(c, f)); // not connected anymore
}
