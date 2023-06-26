pub(crate) mod debug;
pub(crate) mod iter;

use std::{collections::HashMap, borrow::Borrow, hash::Hash};

use self::iter::Iter;



pub(crate) struct TrieNode<T> {
    /// 是否为一个停止节点
    stop: bool,
    /// 子节点们
    childs: HashMap<T, Self>
}

impl<T: Hash + Eq> Eq for TrieNode<T> {}

impl<T> PartialEq for TrieNode<T>
where T: Hash + Eq
{
    fn eq(&self, other: &Self) -> bool {
        self.stop() == other.stop()
            && self.childs() == other.childs()
    }
}


impl<T> Clone for TrieNode<T>
where T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            stop: self.stop(),
            childs: self.childs().clone(),
        }
    }
}

impl<T> Default for TrieNode<T> {
    fn default() -> Self {
        Self {
            stop: false,
            childs: HashMap::with_capacity(0),
        }
    }
}

impl<T> TrieNode<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置为终节点
    fn set_stop(&mut self) {
        self.stop = true
    }

    /// 设置为非终节点
    fn unset_stop(&mut self) {
        self.stop = false
    }

    /// 返回是否是一个终节点
    pub fn stop(&self) -> bool {
        self.stop
    }

    /// link to `self.stop()`
    pub fn is_stop(&self) -> bool {
        self.stop()
    }

    /// 返回子节点是否为空
    pub fn is_empty(&self) -> bool {
        self.childs.is_empty()
    }

    /// 返回是否是一个可删除的节点
    /// 判定为非终节点且没有子节点
    pub fn can_remove(&self) -> bool {
        !self.stop() && self.is_empty()
    }

    #[allow(unused)]
    pub fn childs(&self) -> &HashMap<T, Self> {
        &self.childs
    }

    /// 获取该节点的迭代器
    pub fn iter(&self) -> Iter<T> {
        self.into()
    }
}

impl<T> TrieNode<T>
where T: Hash + Eq
{
    /// 根据键获取子节点
    pub fn get_child(&self, query: impl Borrow<T>) -> Option<&Self> {
        self.childs.get(query.borrow())
    }

    /// 根据键获取可变子节点
    fn get_child_mut(&mut self, query: impl Borrow<T>) -> Option<&mut Self> {
        self.childs.get_mut(query.borrow())
    }

    /// 获取子节点, 如果没有该节点则将其插入
    fn get_or_insert_child(&mut self, data: T) -> &mut Self {
        self.childs.entry(data).or_default()
    }

    /// 从子节点中删除一个节点
    pub fn remove_child(&mut self, query: impl Borrow<T>) -> Option<Self> {
        self.childs.remove(query.borrow())
    }

    /// 删除一个分支
    /// 返回是否删除成功
    /// 当达到给定查询最后一个元素时返回是否删除成功
    /// 然后这个结果会一直被传递到最外层
    /// 在递回时会执行清理函数, 不管是否删除成功
    fn removed_do<Q>(
        &mut self,
        mut iter: impl Iterator<Item = Q>,
        cleaner: &impl Fn(&mut Self, Q),
        ) -> bool
    where Q: Borrow<T>
    {
        if let Some(query) = iter.next() {
            // 查询过程
            if let Some(node) = self.get_child_mut(query.borrow()) {
                // 继续查询
                let is_removed = node.removed_do(iter, cleaner);
                cleaner(self, query);
                is_removed
            } else {
                // 中途查询到空节点, 未能成功查询到
                false
            }
        } else {
            // 查询到尾部节点
            let stop = self.stop();
            if stop {
                // 是一个终止节点, 成功删除
                self.unset_stop();
            } else {
                // 不是一个终止节点, 删除失败, 没有可以删的东西
            }
            stop
        }
    }

    /// 从树中移除一个值串
    /// 并不会进行悬垂清理
    /// 可能造成获取状态时指向了一个悬垂分支导致永不匹配
    pub unsafe fn remove<Q>(&mut self, iter: impl Iterator<Item = Q>) -> bool
    where Q: Borrow<T>
    {
        self.removed_do(iter, &|_, _| ())
    }

    /// 从树中移除一个值串
    /// 如果有悬垂节点则将其删除
    pub fn remove_branch<Q>(&mut self, iter: impl Iterator<Item = Q>) -> bool
    where Q: Borrow<T>
    {
        self.removed_do(iter, &|self_, query| {
            if self_.get_child(query.borrow()).unwrap().can_remove() {
                let res = self_.remove_child(query);
                debug_assert!(matches!(res, Some(..)))
            }
        })
    }

    /// 查询值串是否在树中
    pub fn query<Q>(&self, iter: impl Iterator<Item = Q>) -> bool
    where Q: Borrow<T>
    {
        debug_assert!(! bool::default());
        self.query_node(iter)
            .map(|node| node.is_stop())
            .unwrap_or_default()
    }

    /// 查询值串是否在树中
    /// 需要匹配部分相同, 但是未匹配部分可以通配
    /// 比如
    /// ---
    /// | query | data |   result    |
    /// | ----- | ---- | ----------- |
    /// |  abc  | abcd | Some(false) |
    /// | abcd  | abcd | Some(true)  |
    /// |  bcd  | abcd | None        |
    /// | abcde | abcd | None        |
    pub fn query_nostop<Q>(&self, iter: impl Iterator<Item = Q>) -> Option<bool>
    where Q: Borrow<T>
    {
        self.query_node(iter).map(|node| node.is_stop())
    }

    /// 查询末尾处节点
    /// 如果中途查询值串终止则返回`None`
    pub fn query_node<Q>(&self, iter: impl Iterator<Item = Q>) -> Option<&Self>
    where Q: Borrow<T>
    {
        let mut root = self;
        for query in iter {
            root = root.get_child(query)?
        }
        Some(root)
    }

    /// 查询值串并给出后序元素的迭代器
    pub fn query_iter<Q>(&self, iter: impl Iterator<Item = Q>) -> Option<Iter<T>>
    where Q: Borrow<T>
    {
        self.query_node(iter)
            .map(|node| node.iter())
    }

    /// 插入一串值
    /// 返回是否插入成功
    /// 当值串已存在则插入失败
    pub fn insert(&mut self, iter: impl Iterator<Item = T>) -> bool {
        let mut root = self;
        for data in iter {
            root = root.get_or_insert_child(data)
        }
        if root.stop() {
            // 已经是一个终节点, 插入失败
            false
        } else {
            // 不是一个终节点, 设置为一个终节点, 返回插入成功
            root.set_stop();
            true
        }
    }
}
