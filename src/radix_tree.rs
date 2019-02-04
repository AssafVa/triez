/// A generic tree based collection storing decomposed items
///
/// A generic tree based fixed width per node tree in which inserted elements are decomposed into
/// their parts and stored such that shared prefixes are reused. Optimization used for nodes with
/// single child such that nodes until a future split are condensed into a single node.
///
/// AKA "prefix tree", "trie"
///
/// # Examples
///
/// ```
/// let mut trie = Trie::new(
///     |c: &char| (c.to_lowercase().next().unwrap() as usize) - ('a' as usize),
///     ('z' as usize) - ('a' as usize),
/// );
/// assert_eq!(trie.contains(&"asd".to_string()), false);
/// trie.insert("asd".to_string());
/// assert_eq!(trie.contains(&"asd".to_string()), true);
/// ```

use std::mem;

use super::Decomposable;

enum Node<T> {
    Empty,
    Normal(Vec<Node<T>>),
    Compressed { compressed: Vec<T>, child: Box<Node<T>> },
}

impl<T> Node<T> {
    fn new_empty() -> Node<T> {
        Node::Empty
    }

    fn new_compressed<TIt: Iterator<Item=T>>(it: TIt) -> Node<T> {
        let compressed = it.collect::<Vec<_>>();
        let child = Box::new(Node::Empty);

        Node::Compressed { compressed, child }
    }

    fn new_normal(positions_and_nodes: Vec<(usize, Node<T>)>, alphabet_size: usize) -> Node<T> {
        let mut children = Vec::with_capacity(alphabet_size);
        for _ in 0..alphabet_size {
            children.push(Node::Empty);
        }

        for (pos, node) in positions_and_nodes {
            children[pos] = node;
        }

        Node::Normal(children)
    }
}

pub struct Trie<TParts, FIndex: Fn(&TParts) -> usize> {
    root: Node<TParts>,
    index_fn: FIndex,
    alphabet_size: usize,
}

impl<TParts, FIndex: Fn(&TParts) -> usize> Trie<TParts, FIndex> {
    pub fn new(index_fn: FIndex, alphabet_size: usize) -> Trie<TParts, FIndex> {
        let new_node = Node::new_empty();
        Trie { root: new_node, index_fn, alphabet_size }
    }

    pub fn insert<TIt: Iterator<Item=TParts>, T: Decomposable<TParts, TIt>>(&mut self, t: T) {
        enum EitherIt<TItem, TIt1: Iterator<Item=TItem>, TIt2: Iterator<Item=TItem>> {
            First(TIt1),
            Second(TIt2),
        }
        impl<TItem, TIt1: Iterator<Item=TItem>, TIt2: Iterator<Item=TItem>> Iterator for EitherIt<TItem, TIt1, TIt2> {
            type Item = TItem;

            fn next(&mut self) -> Option<<Self as Iterator>::Item> {
                match self {
                    EitherIt::First(it) => it.next(),
                    EitherIt::Second(it) => it.next(),
                }
            }
        }

        let mut stack = vec![(&mut self.root, EitherIt::First(t.decompose()))];

        while let Some((current, mut it)) = stack.pop() {
            match current {
                Node::Empty => {
                    let compressed = it.collect::<Vec<_>>();
                    if !compressed.is_empty() {
                        let child = Box::new(Node::Empty);
                        let new = Node::Compressed { compressed, child };
                        mem::replace(current, new);
                    }
                }
                Node::Normal(ref mut children) => {
                    if let Some(part) = it.next() {
                        let pos = (self.index_fn)(&part);
                        stack.push((&mut children[pos], it));
                    }
                }
                Node::Compressed { ref mut compressed, child } => {
                    let mut current_pos = 0;
                    'compressed: loop {
                        if let Some(new_part) = it.next() {
                            if current_pos == compressed.len() {
                                match **child {
                                    Node::Empty => {
                                        compressed.push(new_part);
                                        compressed.extend(it);
                                        compressed.shrink_to_fit()
                                    }
                                    Node::Normal(ref mut children) => {
                                        let pos = (self.index_fn)(&new_part);
                                        stack.push((&mut children[pos], it));
                                    }
                                    Node::Compressed { .. } => panic!()
                                }
                                break 'compressed;
                            } else {
                                let existing_part = &compressed[current_pos];
                                let pos_existing = (self.index_fn)(existing_part);
                                let pos_new = (self.index_fn)(&new_part);

                                if pos_existing != pos_new {
                                    match **child {
                                        Node::Empty => {
                                            let new_compressed = Node::new_compressed(it);

                                            let mut drain = compressed.drain(current_pos..);
                                            drain.next();
                                            let existing_compressed = Node::new_compressed(drain);

                                            let new_node = Node::new_normal(vec![(pos_new, new_compressed), (pos_existing, existing_compressed)], self.alphabet_size);
                                            mem::replace(child, Box::new(new_node));
                                        }
                                        Node::Normal(ref mut children) => {
                                            let mut drain = compressed.drain(current_pos..);
                                            drain.next();

                                            let (min_pos, max_pos, left_it, right_it) = if pos_existing > pos_new {
                                                (pos_new, pos_existing, it, EitherIt::Second(drain))
                                            } else {
                                                (pos_existing, pos_new, EitherIt::Second(drain), it)
                                            };

                                            let (left, right) = children.split_at_mut(max_pos);
                                            stack.push((&mut left[min_pos], left_it));
                                            stack.push((&mut right[0], right_it));
                                        }
                                        Node::Compressed { .. } => panic!()
                                    }
                                    break 'compressed;
                                } else {}
                            }
                        } else {
                            break 'compressed;
                        }
                        current_pos += 1;
                    }
                }
            }
        }
    }

    pub fn contains<TIt: Iterator<Item=TParts>, T: Decomposable<TParts, TIt>>(&self, t: T) -> bool {
        let mut current = &self.root;
        let mut it = t.decompose();
        'parts_loop: loop {
            current = match current {
                Node::Empty => {
                    break 'parts_loop it.next().is_none();
                }
                Node::Normal(children) => {
                    if let Some(part) = it.next() {
                        let pos = (self.index_fn)(&part);
                        current = &children[pos];
                        current
                    } else {
                        break 'parts_loop false;
                    }
                }
                Node::Compressed { compressed, child } => {
                    for held_part in compressed.iter() {
                        if let Some(part) = it.next() {
                            if (self.index_fn)(held_part) != (self.index_fn)(&part) {
                                break 'parts_loop false;
                            }
                        } else {
                            break 'parts_loop false;
                        }
                    }
                    child
                }
            }
        }
    }

//    pub fn print_tree(&self) {
//        Trie::<TParts, FIndex>::print_me(&self.root, 0);
//    }
//
//    fn print_me(node: &Node<TParts>, indent: usize) {
//        println!("{:indent$}{:?}", "", node, indent = indent);
//        match node {
//            Node::Normal(children) => {
//                children.iter().filter(|c| !c.is_empty()).for_each(|c|
//                    Trie::<TParts, FIndex>::print_me(c, indent + 2)
//                )
//            }
//            Node::Compressed { compressed: _, child } => {
//                Trie::<TParts, FIndex>::print_me(child, indent + 2);
//            }
//            _ => {}
//        }
//    }
}
