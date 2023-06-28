#![doc = include_str!("../README.md")] // use readme

use std::{
    borrow::Borrow,
    hash::Hash, fmt::Debug,
};

mod trie_node;

use trie_node::TrieNode;
pub use trie_node::iter::Iter;

#[cfg(test)]
mod tests;

/// A TrieTree that supports the following functions:
/// ===
/// * Insert values
/// * Remove values
/// * Traversal values
/// * Prefix lookup values
/// * Debug Struct
pub struct TrieTree<T>
{
    root: TrieNode<T>,
    count: usize,
}

impl<T: Hash + Eq> Eq for TrieTree<T> {}

impl<T> PartialEq for TrieTree<T>
where T: Hash + Eq
{
    fn eq(&self, other: &Self) -> bool {
        self.count() == other.count()
            && self.root == other.root
    }
}

impl<T, I> FromIterator<I> for TrieTree<T>
where T: Hash + Eq,
      I: IntoIterator<Item = T>
{
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let tree: TrieTree<_> = [[1, 2, 3], [1, 3, 5]].into_iter().collect();
    /// assert!(tree.query([1, 2, 3]));
    /// assert!(tree.query([1, 3, 5]));
    /// ```
    fn from_iter<IntoIter: IntoIterator<Item = I>>(iter: IntoIter) -> Self {
        let mut tree = Self::new();
        tree.extend(iter);
        tree
    }
}


impl<T, I> Extend<I> for TrieTree<T>
where T: Hash + Eq,
      I: IntoIterator<Item = T>
{
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// tree.extend([&[1, 2, 3][..], &[1, 2][..]]);
    /// assert!(tree.query(&[1, 2, 3]));
    /// assert!(tree.query(&[1, 2]));
    /// assert!(! tree.query(&[1]));
    /// ```
    fn extend<IntoIter: IntoIterator<Item = I>>(&mut self, iter: IntoIter) {
        for item in iter {
            self.insert(item);
        }
    }
}


impl<T> AsRef<Self> for TrieTree<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> TrieTree<T> {
    /// return values count
    pub fn count(&self) -> usize {
        self.count
    }

    /// set count value
    /// * **Please do not use it! Unless you know what you're doing**
    unsafe fn set_count(&mut self, value: usize) {
        self.count = value
    }

}

impl<T> Clone for TrieTree<T>
where T: Clone
{
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            count: self.count()
        }
    }
}

impl<T: Debug> Debug for TrieTree<T>
where T: Hash + Eq
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TrieTree")
            .field("count", &self.count)
            .field("root", &self.root)
            .finish()
    }
}

impl<T> Default for TrieTree<T>
where T: Hash + Eq,
{
    fn default() -> Self {
        Self {
            root: TrieNode::default(),
            count: 0,
        }
    }
}

impl<T> TrieTree<T>
where T: Hash + Eq
{
    /// new Self
    /// - use Self::default()
    pub fn new() -> Self {
        Self::default()
    }

    /// clear all values
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// assert_eq!(tree.count(), 0);
    /// assert!(tree.insert("abc".chars()));
    /// assert_eq!(tree.count(), 1);
    /// assert!(tree.query("abc".chars()));
    /// tree.clear();
    /// assert_eq!(tree.count(), 0);
    /// assert!(! tree.query("abc".chars()));
    /// assert!(tree.insert("abc".chars()));
    /// ```
    pub fn clear(&mut self) {
        unsafe {
            self.set_root(TrieNode::default());
            self.set_count(0);
        }
    }

    /// set root value
    /// * **Please do not use it! Unless you know what you're doing**
    unsafe fn set_root(&mut self, root: TrieNode<T>) {
        self.root = root
    }

    /// insert values.
    /// - return whether the insertion values was successful
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// assert_eq!(tree.count(), 0);
    /// assert!(tree.insert("abc".chars()));
    /// assert_eq!(tree.count(), 1);
    /// assert!(! tree.insert("abc".chars()));
    /// assert_eq!(tree.count(), 1);
    /// # assert!(! tree.insert("abc".chars()));
    /// # assert_eq!(tree.count(), 1);
    /// ```
    pub fn insert(&mut self, iter: impl IntoIterator<Item = T>) -> bool {
        let res = self.root.insert(iter.into_iter());
        if res {
            unsafe { self.set_count(self.count() + 1) }
        }
        res
    }

    /// Whether the query values is in the tree
    /// Result list examples
    /// ---
    /// | query | data | result |
    /// | ----- | ---- | ------ |
    /// | abcd  | abcd | true   |
    /// | abc   | abcd | false  |
    /// | bcd   | abcd | false  |
    /// | abcde | abcd | false  |
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// assert!(tree.insert("abcd".chars()));
    /// assert!(tree.query("abcd".chars()));
    /// assert!(! tree.query("abc".chars()));
    /// assert!(! tree.query("bcd".chars()));
    /// assert!(! tree.query("abcde".chars()));
    /// ```
    pub fn query<Q>(&self, iter: impl IntoIterator<Item = Q>) -> bool
    where Q: Borrow<T>
    {
        self.root.query(iter.into_iter())
    }

    /// Whether the query values head is in the tree
    /// Result list examples
    /// ---
    /// | query | data |   result    |
    /// | ----- | ---- | ----------- |
    /// |       | abcd | Some(false) |
    /// | abc   | abcd | Some(false) |
    /// | abcd  | abcd | Some(true)  |
    /// | bcd   | abcd |    None     |
    /// | abcde | abcd |    None     |
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// assert_eq!(tree.query_nostop("".chars()),       Some(false));
    /// assert!(tree.insert("abcd".chars()));
    /// assert_eq!(tree.query_nostop("abc".chars()),    Some(false));
    /// assert_eq!(tree.query_nostop("abcd".chars()),   Some(true));
    /// assert_eq!(tree.query_nostop("bcd".chars()),    None);
    /// assert_eq!(tree.query_nostop("abcde".chars()),  None);
    /// ```
    pub fn query_nostop<Q>(&self, iter: impl IntoIterator<Item = Q>) -> Option<bool>
    where Q: Borrow<T>
    {
        self.root.query_nostop(iter.into_iter())
    }

    /// Get the Iterator matching the prefix<br/>
    /// If there is no matching value, return `None`
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// assert!(tree.insert("abc".chars()));
    /// assert!(tree.insert("ace".chars()));
    /// assert!(tree.insert("bee".chars()));
    /// let mut res: Vec<_> = tree.query_iter("a".chars()).unwrap().collect();
    /// res.sort();
    /// assert_eq!(res, vec![vec![&'b', &'c'], vec![&'c', &'e']]);
    /// ```
    pub fn query_iter<Q>(&self, iter: impl IntoIterator<Item = Q>) -> Option<Iter<T>>
    where Q: Borrow<T>
    {
        self.root.query_iter(iter.into_iter())
    }

    /// remove values and clean up overhanging branches
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// assert_eq!(tree.count(), 0);
    /// assert!(tree.insert("abc".chars()));
    /// assert_eq!(tree.count(), 1);
    /// assert!(tree.query("abc".chars()));
    /// assert!(tree.remove("abc".chars()));
    /// assert!(! tree.remove("abc".chars()));
    /// assert!(! tree.query("abc".chars()));
    /// assert_eq!(tree.count(), 0);
    /// ```
    pub fn remove<Q>(&mut self, iter: impl IntoIterator<Item = Q>) -> bool
    where Q: Borrow<T>
    {
        let res = self.root.remove_branch(iter.into_iter());
        if res {
            unsafe { self.set_count(self.count() - 1) }
        }
        res
    }

    /// remove values
    /// * It won't do the cleaning work for hanging branches!
    /// * **Please do not use it! Unless you know what you're doing**
    #[allow(unused)]
    unsafe fn remove_no_clean<Q>(
        &mut self,
        iter: impl IntoIterator<Item = Q>
    ) -> bool
    where Q: Borrow<T>
    {
        let res = self.root.remove(iter.into_iter());
        if res {
            self.set_count(self.count() - 1)
        }
        res
    }

    /// get iterator
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let tree = TrieTree::from_iter(["abc".chars(), "bcd".chars()]);
    /// let mut items: Vec<Vec<&char>> = tree.iter().collect();
    /// items.sort();
    /// assert_eq!(items[0], [&'a', &'b', &'c']);
    /// assert_eq!(items[1], [&'b', &'c', &'d']);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        self.root.iter()
    }

    /// Shrink each `HashMap` in the tree
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::from_iter(["abc".chars(), "bcd".chars()]);
    /// tree.shrink_to_fit();
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.root.op_nodes_first_root(&mut |node| {
            node.childs_mut().shrink_to_fit()
        })
    }
}
