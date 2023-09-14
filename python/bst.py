# This is a sample Python script.
from typing import List


# Press Shift+F10 to execute it or replace it with your code.
# Press Double Shift to search everywhere for classes, files, tool windows, actions, and settings.


class TestClass:
    def __init__(self):
        pass


class Node:
    def __init__(self, val=None):
        self.val = val
        self.left_node = None
        self.right_node = None


class BinarySearch:
    def __init__(self):
        self.tree = None
        pass

    def insert(self, val):
        self.tree = self._insert(val, self.tree)

    def _insert(self, val, node: Node):
        if node is None:
            node = Node(val)
            return node
        if val < node.val:
            node.left_node = self._insert(val, node.left_node)
        else:
            node.right_node = self._insert(val, node.right_node)

        return node

    def search(self, val):
        return self._search(val, self.tree)

    def _search(self, val, node: Node):
        if node is None:
            return None

        if node.val == val:
            return node

        if val < node.val:
            return self._search(val, node.left_node)
        else:
            return self._search(val, node.right_node)

    def traverse(self):
        q: List[Node] = [self.tree]
        while q:
            n = q.pop()

            if n.right_node is not None:
                q.append(n.right_node)
            if n.left_node is not None:
                q.append(n.left_node)

    def _inorder_display(self,node:Node):
        if node is None:
            return
        self._inorder_display(node.left_node)
        print(node.val)
        self._inorder_display(node.right_node)

    def display(self):
        self._inorder_display(self.tree)



def bst():
    examples = [3, 10, 2]
    tree = BinarySearch()
    tree.insert(50)
    tree.insert(3)
    tree.insert(100)

    tree.display()

    node = tree.search(3)
    print(f"Found {node.val}")


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    bst()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
