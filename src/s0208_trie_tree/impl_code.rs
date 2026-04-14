use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<String, TrieNode>,
    val: String,
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, url: Vec<String>) {
        let mut cur = &mut self.root;
        for part in url {
            cur = cur
                .children
                .entry(part.clone())
                .or_insert_with(|| TrieNode {
                    val: part,
                    ..Default::default()
                });
        }
        cur.is_end = true;
    }

    pub fn search(&self, url: Vec<String>) -> bool {
        self.search_endpoint(url).map_or(false, |node| node.is_end)
    }

    pub fn startswith(&self, url: Vec<String>) -> bool {
        self.search_endpoint(url).is_some()
    }

    fn search_endpoint(&self, url: Vec<String>) -> Option<&TrieNode> {
        let mut cur = &self.root;
        for part in url {
            match cur.children.get(&part) {
                Some(node) => cur = node,
                None => return None,
            }
        }
        Some(cur)
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_trie_insert_search() {
        let mut trie = Trie::new();
        let url = vec!["/main".to_string(), "/process".to_string()];
        trie.insert(url.clone());
        let result = trie.search(url);
        assert!(result, "can't match correct path");
    }

    #[test]
    fn test_trie_startswith() {
        let mut trie = Trie::new();
        let url = vec!["/main".to_string(), "/process".to_string()];
        trie.insert(url);
        let surl = vec!["/main".to_string()];
        let result = trie.startswith(surl);
        assert!(result, "can't match correct prefix path");
    }
}
