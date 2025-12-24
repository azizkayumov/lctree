use lctree::{FindSum, LinkCutTree};

#[test]
pub fn path_aggregation() {
    // We form a link-cut tree from the following rooted tree
    // (the numbers in parentheses are the weights of the nodes):
    //           a(9)
    //           /  \
    //         b(1)  e(2)
    //        /   \    \
    //      c(8)  d(10)  f(4)

    // Use FindMax or FindSum, depending on your usage:
    let mut lctree: LinkCutTree<FindSum> = lctree::LinkCutTree::new();
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

    // We find the sum of the weights on the path between c to f,
    let result = lctree.path(c, f);
    assert_eq!(result.sum, 8. + 1. + 9. + 2. + 4.);
}
