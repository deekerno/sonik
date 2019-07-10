// Taken from https://github.com/eugene-bulkin/rust-bk-tree/
// as I needed to be able to serialize the tree.

use std::borrow::Borrow;
use std::cmp::min;
use std::collections::{hash_map, HashMap};
use std::fmt::{self, Debug, Formatter};
use std::iter::Extend;

use serde_derive::{Deserialize, Serialize};

/// A trait for a *metric* (distance function).
///
/// Implementations should follow the metric axioms:
///
/// * **Zero**: `distance(a, b) == 0` if and only if `a == b`
/// * **Symmetry**: `distance(a, b) == distance(b, a)`
/// * **Triangle inequality**: `distance(a, c) <= distance(a, b) + distance(b, c)`
///
/// If any of these rules are broken, then the BK-tree may give unexpected
/// results.
pub trait Metric<K: ?Sized> {
    fn distance(&self, a: &K, b: &K) -> u64;
}

/// A node within the [BK-tree](https://en.wikipedia.org/wiki/BK-tree).
#[derive(Serialize, Deserialize)]
struct BKNode<K> {
    /// The key determining the node.
    key: K,
    /// A hash-map of children, indexed by their distance from this node based
    /// on the metric being used by the tree.
    children: HashMap<u64, BKNode<K>>,
}

impl<K> BKNode<K>
{
    /// Constructs a new `BKNode<K>`.
    pub fn new(key: K) -> BKNode<K>
    {
        BKNode {
            key: key,
            children: HashMap::new(),
        }
    }

    /// Add a child to the node.
    ///
    /// Given the distance from this node's key, add the given key as a child
    /// node. *Warning:* this does not test the invariant that the distance as
    /// measured by the tree between this node's key and the provided key
    /// actually matches the distance passed in.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use bk_tree::BKNode;
    ///
    /// let mut foo = BKNode::new("foo");
    /// foo.add_child(1, "fop");
    /// ```
    pub fn add_child(&mut self, distance: u64, key: K) {
        self.children.insert(distance, BKNode::new(key));
    }
}

impl<K> Debug for BKNode<K> where K: Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_map().entry(&self.key, &self.children).finish()
    }
}

/// A representation of a [BK-tree](https://en.wikipedia.org/wiki/BK-tree).
#[derive(Debug, Serialize, Deserialize)]
pub struct BKTree<K, M = Levenshtein>
{
    /// The root node. May be empty if nothing has been put in the tree yet.
    root: Option<BKNode<K>>,
    /// The metric being used to determine the distance between nodes on the
    /// tree.
    metric: M,
}

impl<K, M> BKTree<K, M>
    where M: Metric<K>
{
    /// Constructs a new `BKTree<K>` using the provided metric.
    ///
    /// Note that we make no assumptions about the metric function provided.
    /// *Ideally* it is actually a
    /// [valid metric](https://en.wikipedia.org/wiki/Metric_(mathematics)),
    /// but you may choose to use one that is not technically a valid metric.
    /// If you do not use a valid metric, however, you may find that the tree
    /// behaves confusingly for some values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bk_tree::{BKTree, metrics};
    ///
    /// let tree: BKTree<&str> = BKTree::new(Levenshtein);
    /// ```
    pub fn new(metric: M) -> BKTree<K, M>
    {
        BKTree {
            root: None,
            metric: metric,
        }
    }

    /// Adds a key to the tree.
    ///
    /// If the tree is empty, this simply sets the root to
    /// `Some(BKNode::new(key))`. Otherwise, we iterate downwards through the
    /// tree until we see a node that does not have a child with the same
    /// distance. If we encounter a node that is exactly the same distance from
    /// the root node, then the new key is the same as that node's key and so we
    /// do nothing. **Note**: This means that if your metric allows for unequal
    /// keys to return 0, you will see improper behavior!
    ///
    /// # Examples
    ///
    /// ```
    /// use bk_tree::{BKTree, metrics};
    ///
    /// let mut tree: BKTree<&str> = BKTree::new(Levenshtein);
    ///
    /// tree.add("foo");
    /// tree.add("bar");
    /// ```
    pub fn add(&mut self, key: K) {
        match self.root {
            Some(ref mut root) => {
                let mut cur_node = root;
                let mut cur_dist = self.metric.distance(&cur_node.key, &key);
                while cur_node.children.contains_key(&cur_dist) && cur_dist > 0 {
                    // We have to do some moving around here to safely get the
                    // child corresponding to the current distance away without
                    // accidentally trying to mutate the wrong thing.
                    //
                    let current = cur_node;
                    let next_node = current.children.get_mut(&cur_dist).unwrap();

                    cur_node = next_node;
                    cur_dist = self.metric.distance(&cur_node.key, &key);
                }
                cur_node.add_child(cur_dist, key);
            }
            None => {
                self.root = Some(BKNode::new(key));
            }
        }
    }

    /// Searches for a key in the BK-tree given a certain tolerance.
    ///
    /// This traverses the tree searching for all keys with distance within
    /// `tolerance` of of the key provided. The tolerance may be zero, in which
    /// case this searches for exact matches. The results are returned as an
    /// iterator of `(distance, key)` pairs.
    ///
    /// *Note:* There is no guarantee on the order of elements yielded by the
    /// iterator. The elements returned may or may not be sorted in terms of
    /// distance from the provided key.
    ///
    /// # Examples
    /// ```
    /// use bk_tree::{BKTree, metrics};
    ///
    /// let mut tree: BKTree<&str> = BKTree::new(metrics::Levenshtein);
    ///
    /// tree.add("foo");
    /// tree.add("fop");
    /// tree.add("bar");
    ///
    /// assert_eq!(tree.find("foo", 0).collect::<Vec<_>>(), vec![(0, &"foo")]);
    /// assert_eq!(tree.find("foo", 1).collect::<Vec<_>>(), vec![(0, &"foo"), (1, &"fop")]);
    /// assert!(tree.find("foz", 0).next().is_none());
    /// ```
    pub fn find<'a, 'q, Q: ?Sized>(&'a self, key: &'q Q, tolerance: u64) -> Find<'a, 'q, K, Q, M>
        where K: Borrow<Q>, M: Metric<Q>
    {
        Find {
            root: self.root.as_ref(),
            stack: Vec::new(),
            tolerance: tolerance,
            metric: &self.metric,
            key: key,
        }
    }

    /// Searches for an exact match in the tree.
    ///
    /// This is equivalent to calling `find` with a tolerance of 0, then picking
    /// out the first result.
    ///
    /// # Examples
    /// ```
    /// use bk_tree::{BKTree, metrics};
    ///
    /// let mut tree: BKTree<&str> = BKTree::new(metrics::Levenshtein);
    ///
    /// tree.add("foo");
    /// tree.add("fop");
    /// tree.add("bar");
    ///
    /// assert_eq!(tree.find_exact("foz"), None);
    /// assert_eq!(tree.find_exact("foo"), Some(&"foo"));
    /// ```
    pub fn find_exact<Q: ?Sized>(&self, key: &Q) -> Option<&K>
        where K: Borrow<Q>, M: Metric<Q>
    {
        self.find(key, 0).next().map(|(_, found_key)| found_key)
    }
}

impl<K, M: Metric<K>> Extend<K> for BKTree<K, M> {
    /// Adds multiple keys to the tree.
    ///
    /// Given an iterator with items of type `K`, this method simply adds every
    /// item to the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use bk_tree::{BKTree, metrics};
    ///
    /// let mut tree: BKTree<&str> = BKTree::new(metrics::Levenshtein);
    ///
    /// tree.extend(vec!["foo", "bar"]);
    /// ```
    fn extend<I: IntoIterator<Item = K>>(&mut self, keys: I) {
        for key in keys {
            self.add(key);
        }
    }
}

impl<K: AsRef<str>> Default for BKTree<K> {
    fn default() -> BKTree<K> {
        BKTree::new(Levenshtein)
    }
}

/// Iterator for the results of `BKTree::find`.
pub struct Find<'a, 'q, K: 'a, Q: 'q + ?Sized, M: 'a>
{
    /// Root node.
    root: Option<&'a BKNode<K>>,
    /// Iterator stack. Because of the inversion of control in play, we must
    /// implement the traversal using an explicit stack.
    stack: Vec<StackItem<'a, K>>,
    tolerance: u64,
    metric: &'a M,
    key: &'q Q,
}

/// An element of the iteration stack.
struct StackItem<'a, K: 'a> {
    cur_dist: u64,
    children_iter: hash_map::Iter<'a, u64, BKNode<K>>,
}

/// Delayed action type. Because of Rust's borrowing rules, we can't inspect
/// and modify the stack at the same time. We instead record the modification
/// and apply it at the end of the procedure.
enum StackAction<'a, K: 'a>
{
    Push(&'a BKNode<K>),
    Pop,
}

impl<'a, 'q, K, Q: ?Sized, M> Iterator for Find<'a, 'q, K, Q, M>
    where K: Borrow<Q>, M: Metric<Q>
{
    type Item = (u64, &'a K);

    fn next(&mut self) -> Option<(u64, &'a K)> {
        // Special case the root node
        if let Some(root) = self.root.take() {
            let cur_dist = self.metric.distance(self.key, root.key.borrow() as &Q);
            self.stack.push(StackItem {
                cur_dist: cur_dist,
                children_iter: root.children.iter(),
            });
            if cur_dist <= self.tolerance {
                return Some((cur_dist, &root.key));
            }
        }

        loop {
            let action = match self.stack.last_mut() {
                Some(stack_top) => {
                    // Find the first child node within an appropriate distance
                    let min_dist = stack_top.cur_dist.saturating_sub(self.tolerance);
                    let max_dist = stack_top.cur_dist.saturating_add(self.tolerance);
                    let mut action = StackAction::Pop;
                    for (dist, child_node) in &mut stack_top.children_iter {
                        if min_dist <= *dist && *dist <= max_dist {
                            action = StackAction::Push(child_node);
                            break;
                        }
                    }
                    action
                },
                None => return None,
            };

            match action {
                StackAction::Push(child_node) => {
                    // Push this child node onto the stack (to inspect later)
                    let cur_dist = self.metric.distance(self.key, child_node.key.borrow() as &Q);
                    self.stack.push(StackItem {
                        cur_dist: cur_dist,
                        children_iter: child_node.children.iter(),
                    });
                    // If this node is also close enough to the key, yield it
                    if cur_dist <= self.tolerance {
                        return Some((cur_dist, &child_node.key));
                    }
                },
                StackAction::Pop => {
                    self.stack.pop();
                },
            }
        }
    }
}

/// This calculates the Levenshtein distance between two strings.
///
/// The [distance metric itself][1] is calculated using the [Wagner-Fischer][2]
/// dynamic programming algorithm.
///
/// # Examples
///
/// ```
/// use bk_tree::Metric;
/// use bk_tree::metrics::Levenshtein;
///
/// assert_eq!(Levenshtein.distance("bar", "baz"), 1);
/// assert_eq!(Levenshtein.distance("kitten", "sitting"), 3);
/// ```
///
/// [1]: https://en.wikipedia.org/wiki/Levenshtein_distance
/// [2]: https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm
#[derive(Debug, Serialize, Deserialize)]
pub struct Levenshtein;

impl<K: AsRef<str> + ?Sized> Metric<K> for Levenshtein
{
    fn distance(&self, a: &K, b: &K) -> u64 {
        let str_a: &str = a.as_ref();
        let str_b: &str = b.as_ref();

        let len_a = str_a.chars().count();
        let len_b = str_b.chars().count();
        if len_a == 0 {
            return len_b as u64;
        }
        if len_b == 0 {
            return len_a as u64;
        }

        // This is a case-insensitive algorithm
        let a_lower = str_a.to_lowercase();
        let b_lower = str_b.to_lowercase();

        // Initialize the array
        let mut d: Vec<Vec<usize>> = Vec::new();
        for j in 0..(len_b + 1) {
            let mut cur_vec = Vec::new();
            for i in 0..(len_a + 1) {
                if j == 0 {
                    cur_vec.push(i);
                } else if i == 0 {
                    cur_vec.push(j);
                } else {
                    cur_vec.push(0);
                }
            }
            d.push(cur_vec);
        }

        for (j, chr_b) in b_lower.chars().enumerate() {
            for (i, chr_a) in a_lower.chars().enumerate() {
                if chr_a == chr_b {
                    // If they're the same, then don't modify the value
                    d[j + 1][i + 1] = d[j][i];
                } else {
                    // Otherwise, pick the lowest cost option for an error
                    let deletion = d[j + 1][i] + 1;
                    let insertion = d[j][i + 1] + 1;
                    let substitution = d[j][i] + 1;
                    d[j + 1][i + 1] = min(min(deletion, insertion), substitution);
                }
            }
        }

        d[len_b][len_a] as u64
    }
}
