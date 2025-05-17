use std::fmt::Display;

use pulldown_cmark::{BlockQuoteKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

#[derive(PartialEq, Debug, Clone)]
pub enum MarkdownElement {
    Paragraph,
    Text(String),
    // Mention,
    // Role,
    Heading(HeadingLevel),
    LineBreak,
    Bold,
    Italic,
    Blockquotes(Option<BlockQuoteKind>),
    List { order: bool },
    ListItem,
    // CodeBlocks,
    // Links,
    // Email,
}

impl MarkdownElement {
    pub fn is_blockquote(&self) -> bool {
        matches!(self, MarkdownElement::Blockquotes(_))
    }
}

impl Display for MarkdownElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownElement::ListItem => write!(f, "list item"),
            MarkdownElement::List { .. } => write!(f, "list"),
            MarkdownElement::Blockquotes(_) => write!(f, "blockquotes"),
            MarkdownElement::Paragraph => write!(f, "p"),
            MarkdownElement::Text(text) => write!(f, "'{text}'"),
            MarkdownElement::Bold => write!(f, "strong"),
            MarkdownElement::Italic => write!(f, "em"),
            MarkdownElement::LineBreak => write!(f, "br"),
            MarkdownElement::Heading(heading_level) => write!(f, "{heading_level}"),
        }
    }
}

impl TryFrom<Tag<'_>> for MarkdownElement {
    type Error = String;

    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        Ok(match value {
            // Tag::Paragraph => MarkdownElement::Paragraph,
            Tag::Emphasis => MarkdownElement::Italic,
            Tag::Strong => MarkdownElement::Bold,
            Tag::Heading { level, .. } => MarkdownElement::Heading(level),
            Tag::BlockQuote(kind) => MarkdownElement::Blockquotes(kind),
            Tag::List(order) => MarkdownElement::List {
                order: order.is_some(),
            },
            Tag::Item => MarkdownElement::ListItem,
            _ => return Err(String::from("This is not possibe right now")),
        })
    }
}

impl TryFrom<TagEnd> for MarkdownElement {
    type Error = String;

    fn try_from(value: TagEnd) -> Result<Self, Self::Error> {
        Ok(match value {
            TagEnd::Item => MarkdownElement::ListItem,
            TagEnd::Paragraph => MarkdownElement::Paragraph,
            TagEnd::Emphasis => MarkdownElement::Italic,
            TagEnd::Strong => MarkdownElement::Bold,
            TagEnd::BlockQuote(kind) => MarkdownElement::Blockquotes(kind),
            TagEnd::Heading(level) => MarkdownElement::Heading(level),
            TagEnd::List(order) => MarkdownElement::List { order },
            _ => return Err(String::from("This is not possible right now")),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkdownNode {
    pub element: MarkdownElement,
    pub start_offset: usize,
    pub end_offset: usize,
    pub childrens: Vec<MarkdownNode>,
    // node_ref: NodeRef<Span>,
}

impl Default for MarkdownNode {
    fn default() -> Self {
        Self {
            element: MarkdownElement::Paragraph,
            start_offset: Default::default(),
            end_offset: Default::default(),
            childrens: Default::default(),
            // node_ref: Default::default(),
        }
    }
}

impl MarkdownNode {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse_markdown_with_offset(input: &str, offset: usize) -> MarkdownNode {
        let mut root = MarkdownNode {
            element: MarkdownElement::Paragraph,
            start_offset: offset,
            end_offset: offset,
            // node_ref: NodeRef::new(),
            childrens: vec![],
        };

        let mut options = Options::empty();
        options.insert(Options::ENABLE_GFM);
        let mut offset = offset;
        let mut parser = Parser::new_ext(input, options);
        root.childrens
            .push(MarkdownNode::parse_events(&mut parser, &mut offset));
        root.end_offset = offset;
        root
    }

    pub fn parse_markdown(input: &str) -> MarkdownNode {
        MarkdownNode::parse_markdown_with_offset(input, 0)
    }

    fn parse_events(parser: &mut Parser, offset: &mut usize) -> MarkdownNode {
        let mut root = MarkdownNode {
            element: MarkdownElement::Paragraph,
            start_offset: *offset,
            end_offset: *offset,
            // node_ref: NodeRef::new(),
            childrens: vec![],
        };

        while let Some(event) = parser.next() {
            if let Some(children) = root.parse_event(event, parser, offset) {
                root.end_offset = *offset;
                root.childrens.push(children);
            }
        }
        root
    }

    fn parse_event(
        &self,
        event: Event,
        parser: &mut Parser,
        offset: &mut usize,
    ) -> Option<MarkdownNode> {
        Some(match event {
            Event::Start(tag) => self.parse_tag(tag, parser, offset)?,
            Event::SoftBreak | Event::HardBreak => MarkdownNode {
                element: MarkdownElement::LineBreak,
                start_offset: *offset,
                end_offset: *offset,
                childrens: vec![],
            },
            Event::Text(cow_str) => {
                let start = *offset;
                *offset += cow_str.len();
                MarkdownNode {
                    element: MarkdownElement::Text(cow_str.to_string()),
                    start_offset: start,
                    end_offset: *offset,
                    childrens: vec![],
                    // node_ref: NodeRef::new(),
                }
            }
            _ => return None,
        })
    }

    fn parse_tag(&self, tag: Tag, parser: &mut Parser, offset: &mut usize) -> Option<MarkdownNode> {
        let element = MarkdownElement::try_from(tag.clone()).ok()?;
        let start = *offset;

        let mut node = MarkdownNode {
            element,
            start_offset: start,
            end_offset: start,
            childrens: vec![],
            // node_ref: NodeRef::new(),
        };

        while let Some(event) = parser.next() {
            match event {
                Event::End(end_tag) => {
                    if let Ok(end_el) = MarkdownElement::try_from(end_tag) {
                        if end_el == node.element {
                            node.end_offset = *offset;
                            break;
                        }
                    }
                }
                _ => {
                    if let Some(child) = self.parse_event(event, parser, offset) {
                        node.childrens.push(child);
                    }
                }
            }
        }

        Some(node)
    }

    pub fn iter(&self) -> MarkdownNodeIterator {
        MarkdownNodeIterator { stack: vec![self] }
    }

    pub fn write_html(&self) -> String {
        let mut html = String::new();

        match &self.element {
            MarkdownElement::Text(text) => {
                html.push_str(text);
                html.push('\n');
            }
            MarkdownElement::LineBreak => {
                html.push_str("<br/>");
            }
            _ => {
                html.push_str(&format!("<{}>\n", self.element));
                for child in &self.childrens {
                    html.push_str(&child.write_html());
                }
                html.push_str(&format!("</{}>\n", self.element));
            }
        }

        html
    }
}

pub struct MarkdownNodeIterator<'a> {
    stack: Vec<&'a MarkdownNode>,
}

impl<'a> Iterator for MarkdownNodeIterator<'a> {
    type Item = &'a MarkdownNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for child in node.childrens.iter().rev() {
                self.stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkdownTree {
    pub root: MarkdownNode,
    pub offset: usize,
}

impl MarkdownTree {
    pub fn new() -> Self {
        let root = MarkdownNode::default();
        MarkdownTree { root, offset: 0 }
    }

    pub fn new_with_root(root: MarkdownNode) -> Self {
        let offset = root.end_offset;
        MarkdownTree { root, offset }
    }

    pub fn new_from_markdown(input: &str) -> Self {
        let root = MarkdownNode::parse_markdown(input);
        MarkdownTree::new_with_root(root)
    }

    pub fn iter(&self) -> MarkdownTreeIterator {
        MarkdownTreeIterator {
            stack: vec![&self.root],
        }
    }

    pub fn write_html(&self) -> String {
        self.root.write_html()
    }
}

pub struct MarkdownTreeIterator<'a> {
    stack: Vec<&'a MarkdownNode>,
}

impl<'a> Iterator for MarkdownTreeIterator<'a> {
    type Item = &'a MarkdownNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for child in node.childrens.iter().rev() {
                self.stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_text() {
        let input = "hello";
        let tree = MarkdownTree::new_from_markdown(input);
        assert_eq!(tree.root.start_offset, 0);
        assert_eq!(tree.root.end_offset, 5);
        let nodes: Vec<_> = tree.iter().collect();

        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].element, MarkdownElement::Paragraph);
        assert_eq!(nodes[1].element, MarkdownElement::Paragraph);
        assert_eq!(nodes[2].element, MarkdownElement::Text("hello".into()));
        assert_eq!(nodes[2].start_offset, 0);
    }

    #[test]
    fn test_parse_bold_text() {
        let input = "**bold**";
        let tree = MarkdownTree::new_from_markdown(input);
        let html = tree.write_html();

        let expected_html = "<p>\n<p>\n<strong>\nbold\n</strong>\n</p>\n</p>\n";
        assert_eq!(html, expected_html);

        let nodes: Vec<_> = tree.iter().collect();
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[2].element, MarkdownElement::Bold);
        assert_eq!(nodes[3].element, MarkdownElement::Text("bold".into()));
    }

    #[test]
    fn test_parse_italic_text() {
        let input = "*italic*";
        let tree = MarkdownTree::new_from_markdown(input);
        let html = tree.write_html();

        let expected_html = "<p>\n<p>\n<em>\nitalic\n</em>\n</p>\n</p>\n";
        assert_eq!(html, expected_html);

        let nodes: Vec<_> = tree.iter().collect();
        assert_eq!(nodes[2].element, MarkdownElement::Italic);
        assert_eq!(nodes[3].element, MarkdownElement::Text("italic".into()));
    }

    #[test]
    fn test_nested_bold_italic() {
        let input = "***bolditalic***";
        let tree = MarkdownTree::new_from_markdown(input);
        let html = tree.write_html();

        let expected_html = "<p>\n<p>\n<em>\n<strong>\nbolditalic\n</strong>\n</em>\n</p>\n</p>\n";
        assert_eq!(html, expected_html);

        let nodes: Vec<_> = tree.iter().collect();
        assert_eq!(nodes.len(), 5);
        assert_eq!(nodes[2].element, MarkdownElement::Italic);
        assert_eq!(nodes[3].element, MarkdownElement::Bold);
        assert_eq!(nodes[4].element, MarkdownElement::Text("bolditalic".into()));
    }

    #[test]
    fn test_parse_incomplet_text() {
        let input = "*hello";
        let tree = MarkdownTree::new_from_markdown(input);
        assert_eq!(tree.root.start_offset, 0);
        assert_eq!(tree.root.end_offset, 6);
        let nodes: Vec<_> = tree.iter().collect();

        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].element, MarkdownElement::Paragraph);
        assert_eq!(nodes[1].element, MarkdownElement::Paragraph);
        assert_eq!(nodes[2].element, MarkdownElement::Text("*".into()));
        assert_eq!(nodes[3].element, MarkdownElement::Text("hello".into()));
    }

    #[test]
    fn test_parse_multilines() {
        let input = "line1\n\nline2\n\n\nline3";
        let tree = MarkdownTree::new_from_markdown(input);
        let nodes: Vec<_> = tree.iter().collect();
        println!("{}", tree.write_html());
        assert_eq!(nodes.len(), 10);
    }

    #[test]
    fn test_parse_empty() {
        let input = "";
        let tree = MarkdownTree::new_from_markdown(input);
        let nodes: Vec<_> = tree.iter().collect();
        println!("{}", tree.write_html());
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn test_parse_heading() {
        let input = "# one\n## two\n### three\n#### four\n##### five\n###### six";
        let tree = MarkdownTree::new_from_markdown(input);
        let nodes: Vec<_> = tree.iter().collect();
        println!("{}", tree.write_html());
        assert_eq!(nodes.len(), 19);
    }

    #[test]
    fn test_parse_list() {
        let input = "- one\n- two\n- three";
        let tree = MarkdownTree::new_from_markdown(input);
        let nodes: Vec<_> = tree.iter().collect();
        println!("{}", tree.write_html());
        assert_eq!(nodes.len(), 13);
    }

    #[test]
    fn test_parse_list_same_line() {
        let input = "- List
- List
- List";
        let tree = MarkdownTree::new_from_markdown(input);
        let nodes: Vec<_> = tree.iter().collect();
        println!("{}", tree.write_html());
        assert_eq!(nodes.len(), 13);
    }
}
