
use super::list_pattern::ListPattern;
use super::structs::*;
use super::parser_object::*;

#[derive(Clone)]
enum MarkerKind {
    Unordered,
    Number(String),
    Alphabetic(String),
}

#[derive(Clone)]
enum ListMetaItem {
    Base{
        depth: usize,
        marker: MarkerKind,
        is_rounded: bool,
        text: String,
    },
    InnerList(usize, Vec<ListMetaItem>),
}

impl ListMetaItem {
    fn get_depth(&self) -> usize {
        match self {
            ListMetaItem::InnerList(depth, _) |
            ListMetaItem::Base{depth, marker: _, is_rounded: _, text: _} => {
                *depth
            },
        }
    }

    fn is_list(&self) -> bool {
        match self {
            ListMetaItem::InnerList(_, _) => true,
            ListMetaItem::Base{depth: _, marker: _, is_rounded: _, text: _} => false,
        }
    }

    fn insert(&mut self, depth: usize, marker: MarkerKind, is_rounded: bool, text: String) {
        if let ListMetaItem::InnerList(own_depth, items) = self {
            if *own_depth == depth {
                items.push(Self::Base{depth, marker, is_rounded, text});
            }
            else if *own_depth < depth { 
                if let Some(elem) = items.last_mut() && elem.is_list() && depth >= elem.get_depth() {
                    elem.insert(depth, marker, is_rounded, text);
                } else {
                    items.push(Self::InnerList(depth, vec![ Self::Base{depth, marker, is_rounded, text} ]));
                }
            } else {
                assert!(*own_depth > depth);
                let old_self = self.clone();
                let new_self = Self::InnerList(depth, vec![
                    old_self, Self::Base{depth, marker, is_rounded, text}
                ]);
                *self = new_self;
            }
        }
    }

    fn convert(&self) -> ListItem {
        match self {
            ListMetaItem::InnerList(_, list) => {
                let mut output = vec![];
                for item in list {
                    output.push(item.convert());
                }
                ListItem::InnerList(output)
            },
            ListMetaItem::Base{depth: _, marker, is_rounded, text} => 
                match marker {
                    MarkerKind::Unordered => ListItem::Unordered(text.clone()),
                    MarkerKind::Number(marker) => if *is_rounded {
                        let marker_num = marker.split(')').nth(0).unwrap_or("");
                        let num = marker_num.parse::<usize>().unwrap_or(0);
                        ListItem::NumberedRounded(num, text.clone())
                    } else { 
                        let marker_num = marker.split('.').nth(0).unwrap_or("");
                        let num = marker_num.parse::<usize>().unwrap_or(0);
                        ListItem::Numbered(num, text.clone())
                    }
                    MarkerKind::Alphabetic(marker) => if *is_rounded {
                        let marker_text = marker.split(')').nth(0).unwrap_or("");
                        ListItem::AlphabeticalRounded(marker_text.to_string(), text.clone())
                    } else { 
                        let marker_text = marker.split('.').nth(0).unwrap_or("");
                        ListItem::Alphabetical(marker_text.to_string(), text.clone())
                    },
                },
        }
    }
}

fn get_pattern(text: &str) -> Option<ListPattern> {
    if text.starts_with(ListPattern::Unordered) {
        Some(ListPattern::Unordered)
    } else if text.starts_with(ListPattern::Numbered) {
        Some(ListPattern::Numbered)
    } else if text.starts_with(ListPattern::NumberedRounded) {
        Some(ListPattern::NumberedRounded)
    } else if text.starts_with(ListPattern::Alphabetical) {
        Some(ListPattern::Alphabetical)
    } else if text.starts_with(ListPattern::AlphabeticalRounded) {
        Some(ListPattern::AlphabeticalRounded)
    } else {
        None
    }
}

struct Wrap((MarkerKind, bool));

impl std::convert::From<(ListPattern, String)> for Wrap {
    fn from(item: (ListPattern, String)) -> Self {
        let (pattern, text) = item;
        match pattern {
            ListPattern::Unordered => Wrap((MarkerKind::Unordered, false)),
            ListPattern::Numbered => Wrap((MarkerKind::Number(text), false)),
            ListPattern::Alphabetical => Wrap((MarkerKind::Alphabetic(text), false)),
            ListPattern::NumberedRounded => Wrap((MarkerKind::Number(text), true)),
            ListPattern::AlphabeticalRounded => Wrap((MarkerKind::Alphabetic(text), true)),
        }
    }
}

pub fn parse_list(object: &mut ParseObject) -> Option<TopLevelSyntax> {
    if let Some(starting_pattern) = get_pattern(object.text().trim_start()) {
        let mut list = Vec::<ListMetaItem>::new();
    
        let mut line = object.current();
        while let Some(pattern) = get_pattern(line.trim_start()) {
            let trimmed_line = line.trim_start();
            let marker = trimmed_line.split_inclusive(pattern.clone()).nth(0).unwrap().to_string();
            let whitespace_count = line.len() - trimmed_line.len();
            let mut string = &line.trim_start()[marker.len()..];
            // let start_len = string.len();
            string = string.trim();
            let count = whitespace_count;

            let Wrap((marker, is_rounded)) = (pattern, marker).into();
            if count > 1 {
                let pow2 = (count - 1).next_multiple_of(2);
                if let Some(elem) = list.last_mut() && elem.is_list() {
                    // if pow2 is 4 and depth is 2 then we insert it as an inner list
                    elem.insert(pow2, marker, is_rounded, string.to_string());
                } else {
                    list.push(ListMetaItem::InnerList(pow2, vec![ ListMetaItem::Base{depth: pow2, marker, is_rounded, text: string.to_string()} ]));
                }
            } else {
                list.push(ListMetaItem::Base{depth: 0, marker, is_rounded, text: string.to_string()});
            }

    
            object.next();
            if object.current().is_empty() { break }
            line = object.current();
        }
        
        let mut output = vec![];
        for item in list {
            output.push(item.convert());
        }

        Some(TopLevelSyntax::List(output))
    } else {
        None
    }
}

mod tests {
    use super::*;

    mod parse_list {
        use super::*;
        
        #[test]
        fn basic() {
            let mut obj = ParseObject::new("- a\n\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ ListItem::Unordered("a".into()) ]))
            }
        }
        
        #[test]
        fn basic_multiple_items() {
            let mut obj = ParseObject::new("- a\n- b\n- c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Unordered("a".into()),
                        ListItem::Unordered("b".into()),
                        ListItem::Unordered("c".into()),
                ]))
            }
        }
        
        #[test]
        fn with_single_space() {
            let mut obj = ParseObject::new("- a\n - b\n - c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Unordered("a".into()),
                        ListItem::Unordered("b".into()),
                        ListItem::Unordered("c".into()),
                ]))
            }
        }
        
        #[test]
        fn with_inner_list() {
            let mut obj = ParseObject::new("- a\n  - b\n  - c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Unordered("a".into()),
                        ListItem::InnerList(vec![
                            ListItem::Unordered("b".into()),
                            ListItem::Unordered("c".into()),
                        ]),
                ]))
            }
        }
        
        #[test]
        fn with_two_inner_list() {
            let mut obj = ParseObject::new("- zero level\n  - first indent\n    - second indent\n  - first indent\n- zero level\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Unordered("zero level".into()),
                        ListItem::InnerList(vec![
                            ListItem::Unordered("first indent".into()),
                            ListItem::InnerList(vec![
                                ListItem::Unordered("second indent".into()),
                            ]),
                            ListItem::Unordered("first indent".into()),
                        ]),
                        ListItem::Unordered("zero level".into()),
                ]))
            }
        }
        
        #[test]
        fn with_two_sepperate_lists() {
            let mut obj = ParseObject::new("- zero level\n    - second indent\n  - first indent\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Unordered("zero level".into()),
                        ListItem::InnerList(vec![
                            ListItem::InnerList(vec![
                                ListItem::Unordered("second indent".into()),
                            ]),
                            ListItem::Unordered("first indent".into()),
                        ]),
                ]))
            }
        }
    }
    
    mod parse_numbered_list {
        use super::*;
        
        #[test]
        fn basic() {
            let mut obj = ParseObject::new("1. a\n\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ ListItem::Numbered(1, "a".into()) ]))
            }
        }
        
        #[test]
        fn basic_multiple_items() {
            let mut obj = ParseObject::new("1. a\n5. b\n3. c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Numbered(1, "a".into()),
                        ListItem::Numbered(5, "b".into()),
                        ListItem::Numbered(3, "c".into()),
                ]))
            }
        }
        
        #[test]
        fn with_single_space() {
            let mut obj = ParseObject::new("1. a\n 2. b\n 3. c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Numbered(1, "a".into()),
                        ListItem::Numbered(2, "b".into()),
                        ListItem::Numbered(3, "c".into()),
                ]))
            }
        }
        
        #[test]
        fn with_inner_list() {
            let mut obj = ParseObject::new("1. a\n  1. b\n  2. c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Numbered(1, "a".into()),
                        ListItem::InnerList(vec![
                            ListItem::Numbered(1, "b".into()),
                            ListItem::Numbered(2, "c".into()),
                        ]),
                ]))
            }
        }
        
        #[test]
        fn with_two_inner_list() {
            let mut obj = ParseObject::new("1. zero level\n  1. first indent\n    1. second indent\n  2. first indent\n2. zero level\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Numbered(1, "zero level".into()),
                        ListItem::InnerList(vec![
                            ListItem::Numbered(1, "first indent".into()),
                            ListItem::InnerList(vec![
                                ListItem::Numbered(1, "second indent".into()),
                            ]),
                            ListItem::Numbered(2, "first indent".into()),
                        ]),
                        ListItem::Numbered(2, "zero level".into()),
                ]))
            }
        }
        
        #[test]
        fn with_two_sepperate_lists() {
            let mut obj = ParseObject::new("1. zero level\n    1. second indent\n  1. first indent\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Numbered(1, "zero level".into()),
                        ListItem::InnerList(vec![
                            ListItem::InnerList(vec![
                                ListItem::Numbered(1, "second indent".into()),
                            ]),
                            ListItem::Numbered(1, "first indent".into()),
                        ]),
                ]))
            }
        }
    }
    
    mod parse_alphabetical_list {
        use super::*;
        
        #[test]
        fn basic() {
            let mut obj = ParseObject::new("a. a\n\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ ListItem::Alphabetical("a".into(), "a".into()) ]))
            }
        }
        
        #[test]
        fn basic_multiple_items() {
            let mut obj = ParseObject::new("a. a\nb. b\nc. c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Alphabetical("a".into(), "a".into()),
                        ListItem::Alphabetical("b".into(), "b".into()),
                        ListItem::Alphabetical("c".into(), "c".into()),
                ]))
            }
        }
        
        #[test]
        fn with_single_space() {
            let mut obj = ParseObject::new("a. a\n b. b\n c. c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Alphabetical("a".into(), "a".into()),
                        ListItem::Alphabetical("b".into(), "b".into()),
                        ListItem::Alphabetical("c".into(), "c".into()),
                ]))
            }
        }
        
        #[test]
        fn with_inner_list() {
            let mut obj = ParseObject::new("a. a\n  b. b\n  c. c\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Alphabetical("a".into(), "a".into()),
                        ListItem::InnerList(vec![
                            ListItem::Alphabetical("b".into(), "b".into()),
                            ListItem::Alphabetical("c".into(), "c".into()),
                        ]),
                ]))
            }
        }
        
        #[test]
        fn with_two_inner_list() {
            let mut obj = ParseObject::new("a. zero level\n  a. first indent\n    a. second indent\n  b. first indent\nc. zero level\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Alphabetical("a".into(), "zero level".into()),
                        ListItem::InnerList(vec![
                            ListItem::Alphabetical("a".into(), "first indent".into()),
                            ListItem::InnerList(vec![
                                ListItem::Alphabetical("a".into(), "second indent".into()),
                            ]),
                            ListItem::Alphabetical("b".into(), "first indent".into()),
                        ]),
                        ListItem::Alphabetical("c".into(), "zero level".into()),
                ]))
            }
        }
        
        #[test]
        fn with_two_sepperate_lists() {
            let mut obj = ParseObject::new("a. zero level\n    a. second indent\n  a. first indent\n\n");
            let result = parse_list(&mut obj);
            assert!(result.is_some());
            if let Some(list) = result {
                assert_eq!(list, TopLevelSyntax::List(vec![ 
                        ListItem::Alphabetical("a".into(), "zero level".into()),
                        ListItem::InnerList(vec![
                            ListItem::InnerList(vec![
                                ListItem::Alphabetical("a".into(), "second indent".into()),
                            ]),
                            ListItem::Alphabetical("a".into(), "first indent".into()),
                        ]),
                ]))
            }
        }
    }

    #[test]
    fn different_list_type_inside() {
        let mut obj = ParseObject::new("- zero level\n    1. second indent\n  - first indent\n\n");
        let result = parse_list(&mut obj);
        assert!(result.is_some());
        if let Some(list) = result {
            assert_eq!(list, TopLevelSyntax::List(vec![ 
                    ListItem::Unordered("zero level".into()),
                    ListItem::InnerList(vec![
                        ListItem::InnerList(vec![
                            ListItem::Numbered(1, "second indent".into()),
                        ]),
                        ListItem::Unordered( "first indent".into()),
                    ]),
            ]))
        }
    }
    
    #[test]
    fn list_of_all_the_things() {
        let mut obj = ParseObject::new("- zero level\n1. zero level\nb. zero level\n3) zero level\nd) zero level\n\n");
        let result = parse_list(&mut obj);
        assert!(result.is_some());
        if let Some(list) = result {
            assert_eq!(list, TopLevelSyntax::List(vec![ 
                    ListItem::Unordered("zero level".into()),
                    ListItem::Numbered(1, "zero level".into()),
                    ListItem::Alphabetical("b".into(), "zero level".into()),
                    ListItem::NumberedRounded(3, "zero level".into()),
                    ListItem::AlphabeticalRounded("d".into(), "zero level".into()),
            ]))
        }
    }
}
