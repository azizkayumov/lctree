from _lctree import LinkCutTree

tree = LinkCutTree()

a = tree.make_tree(9.0)
b = tree.make_tree(1.0)
c = tree.make_tree(8.0)
d = tree.make_tree(10.0)
e = tree.make_tree(2.0)
f = tree.make_tree(4.0)

tree.link(b, a)
tree.link(c, b)
tree.link(d, b)
tree.link(e, a)
tree.link(f, e)

assert tree.connected(c, f) == True # connected

(max_idx, max_weight) = tree.find_max(c, f)
assert max_idx == a
assert max_weight == 9.0

tree.cut(e, a)

assert tree.connected(c, f) == False # not connected anymore

print("All tests passed!")
