[![Build Status](https://travis-ci.org/AssafVa/triez.svg?branch=master)](https://travis-ci.org/AssafVa/triez)

# Trie

implementation of a generic trie.

## implementation

implementation is that of a radix tree with fixed size nodes.
cannot handle nested inputs like: "home" and "homework"

size of nodes is `alphanet_size` parameter on trie init. 
In case a node has a single child there is an optimization of compressing all children until a split, such that 
compressed node size is at most max input length.

to implement on custom types one needs to implement the `Decomposable` trait, example implementation:

```rust
impl Decomposable<u8, std::vec::IntoIter<u8>> for u32 {
    fn decompose(self) -> std::vec::IntoIter<u8> {
        let bytes : Box<[u8]> = Box::new(self.to_be_bytes());
        bytes.into_vec().into_iter()
    }
}
```

## capabilities

currently shipping only minimum viable product so the only capabilities are `insert` and `contains`

## examples

```rust
let mut trie = Trie::new(
    |c: &char| (c.to_lowercase().next().unwrap() as usize) - ('a' as usize), // index function
    ('z' as usize) - ('a' as usize),                                         // alphabet size
);

assert_eq!(trie.contains(String::from("asd")), false);
trie.insert("asd".to_string());
assert_eq!(trie.contains(String::from("asd")), true);
```
