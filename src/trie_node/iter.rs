use super::TrieNode;
use std::{
    collections::hash_map::Iter as HashMapIter,
    fmt::Debug,
};

/// Iterator used to iterate over `TrieNode`
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    /// 存储每个节点以及其是否为终节点
    /// 当获取该值后将是否为终节点设置为否
    nodes: Vec<(bool, HashMapIter<'a, T, TrieNode<T>>)>,
    /// 存储每个值
    datas: Vec<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    /// do `self.next_op(|x| x)`
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// tree.insert([1, 2].into_iter());
    /// assert_eq!(tree.iter().next_ref().unwrap(), &[&1, &2]);
    /// ```
    #[inline]
    pub fn next_ref(&mut self) -> Option<&Vec<&'a T>> {
        self.next_op(|x| x)
    }

    /// Using a function to convert internal temporary references and return next value
    ///
    /// Implemented in the `next` method is `self.next_op(|arr| arr.clone())`
    /// # Examples
    /// ```
    /// # use fast_trie_tree::TrieTree;
    /// let mut tree = TrieTree::new();
    /// tree.insert(0..=2);
    /// assert_eq!(
    ///     tree.iter()
    ///     .next_op(|x| x.into_iter()
    ///         .map(|n| *n + 2)
    ///         .collect::<Vec<_>>()
    ///     ).unwrap(),
    ///     vec![2, 3, 4]
    /// );
    /// ```
    pub fn next_op<'b, R, F>(&'b mut self, f: F) -> Option<R>
    where F: FnOnce(&'b Vec<&'a T>) -> R
    {
        // 节点以及该节点的值迭代器
        let (is_stop, kvs)
            = self.nodes.last_mut()?;
        Some(if *is_stop {
            *is_stop = false;
            f(&self.datas)
        } else if let Some((data, node)) = kvs.next() {
            // 在末端迭代到下一个分支
            self.datas.push(data);
            self.nodes.push((node.stop(), node.childs().iter()));
            self.next_op(f)?
        } else {
            // 末端迭代到了底部
            // 删除该端, 它已没有更多子节点也不是一个终止节点了
            self.datas.pop()?;
            self.nodes.pop()?;
            self.next_op(f)?
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T>
{
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_op(|arr| arr.clone())
    }
}

impl<'a, T> From<&'a TrieNode<T>> for Iter<'a, T> {
    fn from(node: &'a TrieNode<T>) -> Self {
        #![allow(clippy::vec_init_then_push)]
        // 刻意的去使用`Vec::new()`而不是`vec![]`宏
        // 来避免一些可能的效率影响
        let mut nodes = Vec::new();
        nodes.push((node.stop(), node.childs().iter()));
        debug_assert_eq!(nodes.len(), 1);
        Self {
            nodes,
            datas: Vec::new()
        }
    }
}


//// 实现不了 `<(ToT)>`
///// Container for `TrieNode` and `Iter`
//pub struct IntoIter<'a, T> {
//    tree: TrieTree<T>,
//    iter: Option<Iter<'a, T>>,
//    _pin: PhantomPinned,
//}
//impl<'a, T> From<TrieTree<T>> for IntoIter<'a, T>
//where T: Hash + Eq
//{
//    fn from(tree: TrieTree<T>) -> Self {
//        let mut res = IntoIter {
//            tree,
//            iter: None,
//            _pin: PhantomPinned
//        };
//        let iter = res.tree.iter();
//        res.iter = Some(iter);
//        res
//    }
//}
//
//impl<'a, T> IntoIter<'a, T> {
//    pub fn into_tree(self) -> TrieTree<T> {
//        self.tree
//    }
//}
//
//impl<'a, T> IntoIterator for IntoIter<'a, T> {
//    type Item = Vec<&'a T>;
//    type IntoIter = Iter<'a, T>;
//
//    fn into_iter(self) -> Self::IntoIter {
//        self.iter
//    }
//}
//
//impl<'a, T> Deref for IntoIter<'a, T> {
//    type Target = Iter<'a, T>;
//
//    fn deref(&self) -> &Self::Target {
//        &self.iter
//    }
//}
//
//impl<'a, T> DerefMut for IntoIter<'a, T> {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.iter
//    }
//}
//
//impl<'a, T> Debug for IntoIter<'a, T>
//where T: Debug + Hash + Eq,
//{
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.debug_struct("IntoIter")
//            .field("tree", &self.tree)
//            .field("iter", &self.iter)
//            .finish()
//    }
//}
//
//impl<T> From<IntoIter<'_, T>> for TrieTree<T> {
//    fn from(into_iter: IntoIter<'_, T>) -> Self {
//        into_iter.tree
//    }
//}
