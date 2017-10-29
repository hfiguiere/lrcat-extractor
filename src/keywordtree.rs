/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::{BTreeMap,HashMap};

use super::keywords::Keyword;
use super::lrobject::LrObject;

/// Keyword tree (node)
/// Operate as a hash multimap
pub struct KeywordTreeNode {
    // HashMap. Key is the parent id. Values: the children ids.
    map: HashMap<i64,Vec<i64>>,
}

impl KeywordTreeNode {
    pub fn new() -> KeywordTreeNode {
        KeywordTreeNode { map: HashMap::new() }
    }

    fn add_child(&mut self, keyword: &Keyword) {
        if !self.map.contains_key(&keyword.parent) {
            self.map.insert(keyword.parent, vec!());
        }
        self.map.get_mut(&keyword.parent).unwrap().push(keyword.id());
    }

    /// Add children to the tree node.
    pub fn add_children(&mut self, children: &BTreeMap<i64,Keyword>) {
        for (_, child) in children {
            self.add_child(child);
        }
    }

    #[cfg(test)]
    pub fn test() {
        let mut keywords : BTreeMap<i64, Keyword> = BTreeMap::new();
        keywords.insert(1, Keyword::new(1, 0, "", ""));
        keywords.insert(2, Keyword::new(2, 1, "", ""));
        keywords.insert(3, Keyword::new(3, 2, "", ""));
        keywords.insert(4, Keyword::new(4, 0, "", ""));
        keywords.insert(5, Keyword::new(5, 2, "", ""));

        let mut tree_node = KeywordTreeNode::new();
        tree_node.add_children(&keywords);

        assert_eq!(tree_node.map.len(), 3);
        assert_eq!(tree_node.map[&0].len(), 2);
        assert_eq!(tree_node.map[&1].len(), 1);
        assert_eq!(tree_node.map[&2].len(), 2);
    }
}


#[cfg(test)]
#[test]
fn keyword_tree_node_test() {
    KeywordTreeNode::test();
}
