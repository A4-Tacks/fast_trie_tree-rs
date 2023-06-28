use super::*;

const TEST_NUMS: [&[i32]; 59] = {
    [
        &[69, 30, 63, 30, 59, 75, 13, 79, 39],
        &[7, 13, 90, 64, 37, 8, 97, 43, 60],
        &[47, 90, 33, 56, 91, 58, 19, 26],
        &[11, 92, 30, 24, 64, 31, 50, 67],
        &[16, 44, 3, 0, 50, 23, 16, 26],
        &[18, 27, 74, 46, 15, 68, 45],
        &[89, 60, 35, 24, 14, 22, 83],
        &[80, 36, 79, 55, 77, 16, 55],
        &[15, 17, 51, 40, 76, 35, 98],
        &[88, 91, 91, 31, 65, 53, 44],
        &[20, 45, 19, 81, 54, 61, 49],
        &[95, 51, 95, 54, 41, 48, 27],
        &[90, 15, 27, 5, 42, 78, 65],
        &[5, 58, 26, 72, 29, 99, 28],
        &[12, 81, 29, 38, 13, 30, 7],
        &[47, 35, 49, 4, 54, 25, 89],
        &[81, 83, 20, 8, 9, 38, 79],
        &[26, 38, 29, 88, 25, 59],
        &[62, 29, 83, 91, 38, 68],
        &[53, 83, 18, 96, 22, 87],
        &[88, 83, 28, 66, 20, 32],
        &[22, 38, 4, 47, 36, 15],
        &[90, 43, 3, 66, 89, 65],
        &[17, 38, 70, 70, 24],
        &[70, 15, 51, 85, 4],
        &[51, 43, 3, 57, 16],
        &[93, 38, 75, 73, 9],
        &[87, 5, 35, 88, 64],
        &[22, 93, 78, 41],
        &[33, 95, 27, 66],
        &[99, 88, 72, 98],
        &[86, 10, 17, 63],
        &[1, 3, 4, 6],
        &[1, 3, 9, 5],
        &[96, 57, 40],
        &[41, 78, 24],
        &[69, 46, 73],
        &[74, 82, 80],
        &[41, 93, 56],
        &[68, 74, 30],
        &[73, 97, 14],
        &[0, 68, 72],
        &[92, 9, 87],
        &[5, 83, 22],
        &[8, 79, 82],
        &[2, 7, 8],
        &[29, 51],
        &[25, 92],
        &[79, 64],
        &[51, 33],
        &[30, 90],
        &[43],
        &[15],
        &[21],
        &[55],
        &[37],
        &[58],
        &[88],
        &[],
    ]
};

#[test]
fn count_test() {
    let mut tree = TrieTree::default();
    assert_eq!(tree.count(), 0);
    assert!(tree.insert("abc".chars()));
    assert_eq!(tree.count(), 1);
    assert!(tree.insert("ace".chars()));
    assert_eq!(tree.count(), 2);
    assert!(tree.remove("abc".chars()));
    assert_eq!(tree.count(), 1);
    assert!(! tree.remove("abc".chars()));
    assert_eq!(tree.count(), 1);
    assert!(tree.remove("ace".chars()));
    assert_eq!(tree.count(), 0);
    assert!(! tree.remove("ace".chars()));
    assert_eq!(tree.count(), 0);
}

#[test]
fn empty_test() {
    let mut tree = TrieTree::default();
    assert_eq!(tree.query_nostop("".chars()), Some(false));
    assert!(! tree.query("".chars()));
    assert_eq!(tree.count(), 0);
    tree.insert("".chars());
    assert_eq!(tree.count(), 1);
    assert!(tree.query("".chars()));
}

#[test]
fn insert_test() {
    let mut tree = TrieTree::new();
    let strs = ["a", "abc", "ab", "def", "f", ""];
    for (i, str) in strs.into_iter().enumerate() {
        assert_eq!(tree.count(), i);
        assert!(tree.insert(str.chars()));
        assert_eq!(tree.count(), i + 1);
    }
    for str in strs {
        assert!(tree.query(str.chars()));
        assert_eq!(tree.query_nostop(str.chars()), Some(true));
    }
    for str in strs {
        assert!(tree.remove(str.chars()));
        assert_eq!(tree.query(str.chars()), false);
        assert_ne!(tree.query_nostop(str.chars()), Some(true));
    }
    assert_eq!(tree.count(), 0);
}

#[test]
fn other_type_test() {
    let arr: &[&[i32]] = &TEST_NUMS;
    let mut tree = TrieTree::new();
    for (i, nums) in arr.into_iter().enumerate() {
        assert_eq!(tree.count(), i);
        assert!(tree.insert(nums.into_iter()));
        assert_eq!(tree.count(), i + 1);
    }
    for nums in arr {
        assert!(tree.query(nums.into_iter()));
        assert_eq!(tree.query_nostop(nums.into_iter()), Some(true));
    }
    assert_eq!(tree.count(), arr.len());
    for nums in arr {
        assert!(tree.remove(nums.into_iter()));
        assert_eq!(tree.query(nums.into_iter()), false);
        assert_ne!(tree.query_nostop(nums.into_iter()), Some(true));
    }
    assert_eq!(tree.count(), 0);
}

#[test]
fn iter_test() {
    let mut tree = TrieTree::new();
    for nums in TEST_NUMS {
        assert!(tree.insert(nums.into_iter()))
    }
    for nums in TEST_NUMS {
        assert!(tree.query(nums.into_iter()))
    }
    let mut vals: Vec<_> = tree.iter().collect();
    vals.sort();
    let mut origin_nums = Vec::from(TEST_NUMS);
    origin_nums.sort();
    macro_rules! cmp_eq {
        () => {
            for (a, b) in vals.into_iter().zip(origin_nums.clone()) {
                assert_eq!(&a.into_iter().map(|x| **x).collect::<Vec<_>>()[..], b);
            }
        };
    }
    cmp_eq!();
    for nums in TEST_NUMS.into_iter().take(5).chain(TEST_NUMS.into_iter().rev().take(4)) {
        assert!(tree.remove(nums.into_iter()));
        let pos = origin_nums.iter().position(|x| *x == &nums[..]).unwrap();
        origin_nums.swap_remove(pos);
    }
    origin_nums.sort();
    vals = tree.iter().collect();
    vals.sort();
    assert_eq!(tree.count(), vals.len());
    cmp_eq!();
}

#[test]
fn equals_test() {
    let tree1 = TrieTree::from_iter([[1, 2, 3], [1, 3, 5]]);
    let mut tree2 = TrieTree::from_iter([[1, 2, 3], [1, 3, 5]]);
    assert_eq!(tree1, tree2);
    tree2.insert([1, 2, 4]);
    assert_ne!(tree1, tree2);
    tree2.remove([1, 2, 3]);
    assert_ne!(tree1, tree2);
    tree2.remove([1, 2, 4]);
    assert_ne!(tree1, tree2);
    tree2.insert([1, 2, 3]);
    assert_eq!(tree1, tree2);

    // ========================================
    let [mut tree1, tree2] = [
        TrieTree::from_iter(TEST_NUMS),
        TrieTree::from_iter(TEST_NUMS.into_iter().rev())];
    assert_eq!(tree1, tree2);

    for item in TEST_NUMS {
        tree1.remove(item);
        assert_ne!(tree1, tree2);
        tree1.insert(item);
        assert_eq!(tree1, tree2);
    }
}

#[test]
fn example() {
    use crate::TrieTree;

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
}

#[test]
fn tree_from_node() {
    let tree: TrieTree<_> = TrieTree::from_iter(TEST_NUMS);
    let count = tree.count();
    let root: TrieNode<_> = tree.clone().into();
    let new_tree: TrieTree<_> = root.into();
    assert_eq!(new_tree.count(), count);
    assert_eq!(new_tree, tree);
}
