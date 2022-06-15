# Tries

Implementiaions of a standard (unoptimized) [trie](https://en.wikipedia.org/wiki/Trie) data structure in rust and python.

The trie(s) implemented here allow the following operations:
- `insert(string)` inserts a word into the trie
- `search(string) : list(string)` returns all words in the trie that begin with the given string or are equal to the given string 
- `exists(string): bool` returns true if the given string is in this trie
- `delete(string)` removes the given string from the try

All operations on the trie are `O(n)` runtime