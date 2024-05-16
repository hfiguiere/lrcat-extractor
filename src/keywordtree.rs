/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::collections::{BTreeMap, HashMap};

use super::keywords::Keyword;
use super::lrobject::LrObject;

/// Keyword tree
/// Operate as a hash multimap of parent -> `Vec<child>`
#[derive(Default)]
pub struct KeywordTree {
    // HashMap. Key is the parent id. Values: the children ids.
    map: HashMap<i64, Vec<i64>>,
}

impl KeywordTree {
    pub fn new() -> KeywordTree {
        KeywordTree::default()
    }

    /// Get children for keyword with `id`
    pub fn children_for(&self, id: i64) -> Vec<i64> {
        if let Some(children) = self.map.get(&id) {
            return children.clone();
        }
        vec![]
    }

    fn add_child(&mut self, keyword: &Keyword) {
        self.map.entry(keyword.parent).or_default();
        self.map
            .get_mut(&keyword.parent)
            .unwrap()
            .push(keyword.id());
    }

    /// Add children to the tree node.
    pub fn add_children(&mut self, children: &BTreeMap<i64, Keyword>) {
        for child in children.values() {
            self.add_child(child);
        }
    }

    #[cfg(test)]
    pub fn test() {
        let mut keywords: BTreeMap<i64, Keyword> = BTreeMap::new();
        keywords.insert(1, Keyword::new(1, 0, "", ""));
        keywords.insert(2, Keyword::new(2, 1, "", ""));
        keywords.insert(3, Keyword::new(3, 2, "", ""));
        keywords.insert(4, Keyword::new(4, 0, "", ""));
        keywords.insert(5, Keyword::new(5, 2, "", ""));

        let mut tree = KeywordTree::new();
        tree.add_children(&keywords);

        assert_eq!(tree.map.len(), 3);
        assert_eq!(tree.map[&0].len(), 2);
        assert_eq!(tree.map[&1].len(), 1);
        assert_eq!(tree.map[&2].len(), 2);

        let children = tree.children_for(0);
        assert_eq!(children, vec![1, 4]);
    }
}

#[cfg(test)]
#[test]
fn keyword_tree_test() {
    KeywordTree::test();
}
