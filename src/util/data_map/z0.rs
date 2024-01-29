use std::option::Option;
use std::vec::Vec;

pub struct Node<Record> {
    pub character: char,
    pub children: Vec<Node<Record>>,
    pub record: Option<Record>
}

pub struct DataMap<Record> {
    root_node: Node<Record>
}

impl<Record> crate::util::data_map::DataMap<Record> for DataMap<Record> {
    type Key = &'a str;

    fn get_record(&mut self, label: Self::Key) -> &Option<Record> {
        let potential_node = self.get_mutable_node(label, false);
        if potential_node.is_none() {
            return &None;
        }

        &potential_node.unwrap().record
    }

    fn set_record(&mut self, label: &str, record: Record) {
        // self.get_mutable_node() in this situation will never return Option::None because create_missing_branches is enabled.
        self.get_mutable_node(label, true).unwrap().record = Some(record);
    }
}

impl<Record> DataMap<Record> {
    pub fn get_mutable_node(&mut self, label: &str, create_missing_branches: bool) -> Option<&mut Node<Record>> {
        let mut path = label.chars();
        let mut selector: &mut Node<Record> = &mut self.root_node;

        for _ in 0..label.len() {
            let path_segment_option = path.next();
            path_segment_option?;

            let path_segment = path_segment_option.unwrap();

            let potential_selector = selector
                .children
                .iter_mut()
                .find(|child| child.character == path_segment);

            if let Some(new_selector) = potential_selector {
                selector = new_selector;
            } else if create_missing_branches {
                // This is the length but once the new Node is added
                let last_index = selector.children.len();
                selector.children.push(Node {
                    character: path_segment,
                    record: None,
                    children: Vec::new()
                });

                selector = &mut selector.children[last_index];
            } else {
                return None;
            }
        }

        Some(selector)
    }
}

impl<Record> Default for DataMap<Record> {
    fn default()-> Self {
        Self {
            root_node: Node {
                character: '\0',
                children: Vec::new(),
                record: None
            }
        }
    }
}