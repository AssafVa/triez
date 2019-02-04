mod radix_tree;
mod implementations;

pub use implementations::*;

/// A generic tree based collection storing decomposed items
///
/// A generic tree based fixed width per node tree in which inserted elements are decomposed into
/// their parts and stored such that shared prefixes are reused. Optimization used for nodes with
/// single child such that nodes until a future split are condensed into a single node.
///
/// AKA "prefix tree", "Radix tree"
///
/// # Examples
///
/// ```
/// let mut trie = Trie::new(
///     |c: &char| (c.to_lowercase().next().unwrap() as usize) - ('a' as usize),
///     ('z' as usize) - ('a' as usize),
/// );
/// assert_eq!(trie.contains(&String::from("asd"))), false);
/// trie.insert(String::from("asd")));
/// assert_eq!(trie.contains(&String::from("asd"))), false);
/// ```
pub type Trie<T, FIndex> = radix_tree::Trie<T, FIndex>;

/// Trait that splits T into component parts
///
/// this trait needs to be implemented in order for T to be placed into a trie
pub trait Decomposable<TParts, TIterator: Iterator<Item=TParts>> {
    fn decompose(self) -> TIterator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_simple() {
        let mut trie = Trie::new(
            |c: &char| (c.to_lowercase().next().unwrap() as usize) - ('a' as usize), // index function
            ('z' as usize) - ('a' as usize),                                      // alphabet size
        );

        assert_eq!(trie.contains(String::from("asd")), false);
        assert_eq!(trie.contains(String::from("dsa")), false);
        trie.insert(String::from("asd"));
        assert_eq!(trie.contains(String::from("dsa")), false);
        assert_eq!(trie.contains(String::from("asd")), true);
        trie.insert(String::from("asd"));
        assert_eq!(trie.contains(String::from("asd")), true);
        assert_eq!(trie.contains(String::from("dsa")), false);
        trie.insert(String::from("dsa"));
        assert_eq!(trie.contains(String::from("asd")), true);
        assert_eq!(trie.contains(String::from("dsa")), true);
    }

    #[test]
    fn test_trie_simple_numeric() {
        let mut trie = Trie::new(
            |c: &u8| (*c as usize),
            u8::max_value() as usize,
        );

        trie.insert(456 as u16);
    }
}
