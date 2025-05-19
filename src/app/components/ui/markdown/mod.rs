use pulldown_cmark::{
    BlockQuoteKind, CodeBlockKind, Event, HeadingLevel, LinkType, Options, Parser, Tag, TagEnd,
};

#[derive(PartialEq, Debug, Clone)]
pub enum MarkdownElement {
    Paragraph,
    Text(String),
    Code(String),
    Heading(HeadingLevel),
    LineBreak,
    Bold,
    Italic,
    Blockquotes(Option<BlockQuoteKind>),
    List { order: bool },
    ListItem,
    CodeBlock(Option<String>),
    Link { url: Option<String> },
}

impl TryFrom<Tag<'_>> for MarkdownElement {
    type Error = String;

    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        Ok(match value {
            Tag::Link {
                link_type: LinkType::Autolink,
                dest_url,
                title: _,
                id: _,
            } => MarkdownElement::Link {
                url: Some(dest_url.to_string()),
            },
            Tag::Link {
                link_type: _,
                dest_url,
                title: _,
                id: _,
            } => MarkdownElement::Text(dest_url.to_string()),
            Tag::CodeBlock(kind) => {
                let lang = if let CodeBlockKind::Fenced(info) = kind {
                    let lang = info.split(' ').next().unwrap();
                    if lang.is_empty() {
                        None
                    } else {
                        Some(lang.to_string())
                    }
                } else {
                    None
                };

                MarkdownElement::CodeBlock(lang)
            }
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
            TagEnd::Link => MarkdownElement::Link { url: None },
            TagEnd::CodeBlock => MarkdownElement::CodeBlock(None),
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
                if let Some(last_child) = root.childrens.last_mut() {
                    if let (MarkdownElement::Text(text1), MarkdownElement::Text(text2)) =
                        (&last_child.element, &children.element)
                    {
                        last_child.element = MarkdownElement::Text(format!("{text1}{text2}"));
                        last_child.end_offset = children.end_offset;
                    } else {
                        root.childrens.push(children);
                    }
                } else {
                    root.childrens.push(children);
                }
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
            Event::Code(cow_str) => {
                let start = *offset;
                *offset += cow_str.len();
                MarkdownNode {
                    element: MarkdownElement::Code(cow_str.to_string()),
                    start_offset: start,
                    end_offset: *offset,
                    childrens: vec![],
                    // node_ref: NodeRef::new(),
                }
            }
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
                        if matches!(node.element, MarkdownElement::CodeBlock(..))
                            && matches!(end_el, MarkdownElement::CodeBlock(..))
                        {
                            node.end_offset = *offset;
                            break;
                        }
                        if matches!(node.element, MarkdownElement::Link { .. })
                            && matches!(end_el, MarkdownElement::Link { .. })
                        {
                            node.end_offset = *offset;
                            break;
                        }
                        if end_el == node.element {
                            node.end_offset = *offset;
                            break;
                        }
                    }
                }
                _ => {
                    if let Some(child) = self.parse_event(event, parser, offset) {
                        if let Some(last_child) = node.childrens.last_mut() {
                            if let (MarkdownElement::Text(text1), MarkdownElement::Text(text2)) =
                                (&last_child.element, &child.element)
                            {
                                last_child.element =
                                    MarkdownElement::Text(format!("{text1}{text2}"));
                                last_child.end_offset = child.end_offset;
                            } else {
                                node.childrens.push(child);
                            }
                        } else {
                            node.childrens.push(child);
                        }
                    }
                }
            }
        }

        Some(node)
    }

    pub fn iter(&self) -> MarkdownNodeIterator {
        MarkdownNodeIterator { stack: vec![self] }
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
