from typing import Optional


class Node:
    def __init__(self, val):
        self.next = None
        self.val = val


class LinkedList:
    def __init__(self):
        self.head: Optional[Node] = None
        self.tail: Optional[Node] = None

    def insert(self, val):
        new_node = Node(val)
        if self.head is None:
            self.head = new_node
            return

        curr_node = self.head
        while True:
            if curr_node.next is None:
                curr_node.next = new_node
                break
            curr_node = curr_node.next

    def delete(self, val):
        curr_node: Node = self.head
        prev_node: Node = None
        while curr_node:
            if curr_node.val == val:
                print("Deleting")
                if prev_node:
                    prev_node.next = curr_node.next
                else:
                    # curr_node is head
                    self.head = curr_node.next
                del curr_node
                break

            prev_node = curr_node
            curr_node = curr_node.next

    def print(self):
        curr_node = self.head
        while curr_node:
            print(curr_node.val)
            curr_node = curr_node.next


def main():
    print("Start")
    ll = LinkedList()
    ll.insert(1)
    ll.insert(10)
    ll.insert(2)

    ll.print()

    ll.delete(1)
    ll.print()


if __name__ == '__main__':
    main()
