from lctree_rs import FindMaxTree

# Build a forest consisting of 6 nodes with the following weights:
# (the numbers in parentheses are the weights of the nodes):
tree = FindMaxTree()
a = tree.make_tree(9.0)
b = tree.make_tree(1.0)
c = tree.make_tree(8.0)
d = tree.make_tree(10.0)
e = tree.make_tree(2.0)
f = tree.make_tree(4.0)

# Link the nodes to form the following tree:
#           a(9)
#           /  \
#         b(1)  e(2)
#        /   \    \
#      c(8)  d(10)  f(4)
tree.link(b, a)
tree.link(c, b)
tree.link(d, b)
tree.link(e, a)
tree.link(f, e)

# Check connectivity:
assert tree.connected(c, f)

# Find the node with the maximum weight on the path from c to f:
(max_idx, max_weight) = tree.find_max(c, f)
assert max_idx == a
assert max_weight == 9.0

# Cut the edge between e and a:
tree.cut(e, a)

# Now c and f should not be connected anymore:
assert not tree.connected(c, f)

print("OK!")
