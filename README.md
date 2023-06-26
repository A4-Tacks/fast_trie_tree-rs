# Fast Trie Tree
* no use `Rc` | `RefCell`...
* Structures such as `Rc` and `RefCell` are not used,
  so this will be slightly faster than `TrieTree` implementations that use those structures
* Use `Iterator<T: Hash + Eq>` to generalize, not only for a data type such as `char`, `u8`


## Examples
```rust
# use fast_trie_tree::TrieTree;

let mut ctree: TrieTree<char> = TrieTree::new();
assert!(ctree.insert("abc".chars()));
assert_eq!(format!("{:#?}", ctree), concat!(
        "TrieTree {\n",
        "    count: 1,\n",
        "    root: (/): {\n",
        "        ('a'): {\n",
        "            ('b'): {\n",
        "                ['c']: {}\n",
        "            }\n",
        "        }\n",
        "    },\n",
        "}"
));
assert_eq!(ctree.query_iter("a".chars())
           .unwrap().collect::<Vec<_>>(), [[&'b', &'c']]);
assert!(ctree.insert("ace".chars()));
assert!(ctree.query("ace".chars()));
assert!(ctree.query("abc".chars()));
assert!(! ctree.query("ab".chars()));

let byte_tree = TrieTree::from_iter(
    [
        [0u8, 2, 3, 8],
        [4, 5, 7, 9],
        [4, 2, 6, 1],
    ]);
assert_eq!(byte_tree.count(), 3);
assert!(byte_tree.query([0, 2, 3, 8]));
assert!(byte_tree.query([4, 5, 7, 9]));
assert!(byte_tree.query([4, 2, 6, 1]));
let mut res = byte_tree.query_iter([4]).unwrap().collect::<Vec<_>>();
res.sort();
assert_eq!(res, [[&2, &6, &1], [&5, &7, &9]]);
assert_eq!(byte_tree.count(), 3);
```


# Info
* crate: <https://crates.io/crates/fast_trie_tree>
* repo: <https://github.com/A4-Tacks/fast_trie_tree-rs>
