from typing import Optional
from collections import deque


class Node:
    """
    Node represnents a "Node" of a Trie.
    It will be either a intermediate Node or a terminal Node.
    Intermediate Node's store a single character, they are the middle characters of a word,
    while terminal Nodes will represent the last character of a word. If a Node is marked as
    terminal, then its value property will contain the actual word that was stored
    """
    def __init__(self, key=None):
        self.children: dict[str, Node] = dict()
        # the prefix character stored in this node
        self.key: Optional[str] = key
        # the word stored in the node if this is a terminal node
        self.value: Optional[str] = None
        # is this a terminal (leaf) node
        self.terminal: bool = False

    def __eq__(self, o: object) -> bool:
        """
        two Nodes are equal if their keys are equal
        """
        if not isinstance(o, Node):
            return NotImplemented
        return self.key == o.key

    def __ne__(self, o: object) -> bool:
        if not isinstance(o, Node):
            return NotImplemented
        return self.key != o.key

    def __hash__(self) -> int:
        """
        a nodes key will be used as its hash value
        """
        return hash(self.key)

    def __str__(self):
        return f"key:{self.key} term:{self.terminal} value:{self.value} child_len:{len(self.children)}"


class Trie:

    def __init__(self):
        self.root = Node()

    def insert(self, s: str):
        """
        inserts s into this Trie
        :param s: the string to insert
        :return:
        """
        curr = self.root
        for ch in s:
            child_node = curr.children.get(ch)
            if child_node:
                curr = child_node
            else:
                curr.children[ch] = Node(ch)
                curr = curr.children[ch]
        # should now be at a terminal node
        curr.terminal = True
        curr.value = s

    def exists(self, s: str) -> bool:
        """
        checks if the s exists in this Trie
        :param s:
        :return: True if s exists in this Trie, else False
        """
        curr = self.root
        for ch in s:
            curr = curr.children.get(ch)
            if not curr:
                return False
        return curr.terminal

    def search(self, s: str) -> list[str]:
        if len(s) == 0:
            return list()
        curr = self.root
        for ch in s:
            curr = curr.children.get(ch)
            if not curr:
                return list()
        # should be at the end of the prefix match, now perform a Depth First Search
        # to find all matching nodes
        matches = []
        queue = [curr]
        while len(queue) > 0:
            node = queue.pop()
            for (key, child_node) in node.children.items():
                queue.append(child_node)

            if node.terminal:
                matches.append(node.value)
        # return matching strings in ascending order
        matches.sort()
        return matches

    def delete(self, s: str) -> bool:
        curr = self.root
        for ch in s:
            curr = curr.children.get(ch)
            if not curr:
                return False

        if curr.terminal and curr.value == s:
            curr.terminal = False
            curr.value = None
            return True
        else:
            return False

    def __str__(self):
        """
        prints the node.key(s) of this Trie in row order
        :return:
        """
        root = self.root
        queue = deque()
        queue.append(root)
        res = ""

        while len(queue) > 0:
            for _ in range(len(queue)):
                if queue[0]:
                    node = queue.popleft()
                    for (child_key, child_node) in node.children.items():
                        term = "*" if child_node.terminal else ""
                        res += child_key + "(" + term + ") "
                        # print("{}({}) ".format(child_key, term), end="")
                        if len(child_node.children) > 0:
                            queue.append(child_node)
            if len(queue) > 0:
                res += "\n"

        return res
