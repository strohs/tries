import unittest
from trie import Trie


class TestTrie(unittest.TestCase):

    def test_new_trie_has_empty_root_node(self):
        trie = Trie()
        self.assertIsNone(trie.root.key)
        self.assertIsNone(trie.root.value)
        self.assertEqual(trie.root.terminal, False)
        self.assertEqual(len(trie.root.children), 0)

    def test_insert(self):
        trie = Trie()
        trie.insert("an")
        # check letter "a" was inserted
        self.assertEqual(len(trie.root.children), 1)
        self.assertEqual(trie.root.children.get("a").key, "a")
        # check letter "n" inserted as a child of "a" and it is a terminal node with a value of "an'
        self.assertEqual(len(trie.root.children['a'].children), 1)
        n_node = trie.root.children.get("a").children.get("n")
        self.assertEqual(n_node.key, "n")
        self.assertEqual(n_node.terminal, True)
        self.assertEqual(n_node.value, "an")

    def test_exists(self):
        trie = Trie()
        trie.insert("an")
        trie.insert("anna")
        trie.insert("annabelle")
        self.assertEqual(trie.exists("anna"), True)
        self.assertEqual(trie.exists("annabelle"), True)

    def test_exists_for_nonexistent_word(self):
        trie = Trie()
        trie.insert("an")
        trie.insert("anna")
        trie.insert("annabelle")
        self.assertEqual(trie.exists("foo"), False)

    def test_search(self):
        trie = Trie()
        trie.insert("an")
        trie.insert("anna")
        trie.insert("annabelle")
        trie.insert("tea")
        trie.insert("teapot")
        trie.insert("teavana")
        matches = trie.search("an")
        self.assertEqual(len(matches), 3)
        self.assertListEqual(matches, ["an", "anna", "annabelle"], "search results should be in ascending order")

    def test_search_returns_empty_list_for_non_existent(self):
        trie = Trie()
        trie.insert("an")
        trie.insert("anna")
        trie.insert("annabelle")
        matches = trie.search("foo")
        self.assertEqual(len(matches), 0)


if __name__ == '__main__':
    unittest.main()
