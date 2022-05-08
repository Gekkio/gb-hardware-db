use html5ever::{
    driver::ParseOpts, namespace_url, ns, parse_fragment, tendril::TendrilSink, LocalName, QualName,
};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use percy_dom::{AttributeValue, IterableNodes, VElement, VirtualNode};

pub fn parse_html_fragment(text: &str) -> IterableNodes {
    let opts = ParseOpts::default();
    let context_name = QualName::new(None, ns!(html), LocalName::from("body"));
    let context_attrs = Vec::new();
    let dom = parse_fragment(RcDom::default(), opts, context_name, context_attrs)
        .from_utf8()
        .read_from(&mut text.as_bytes())
        .unwrap();
    let children = dom.document.children.borrow();
    children
        .iter()
        .filter_map(convert)
        .collect::<Vec<_>>()
        .into()
}

fn convert(node: &Handle) -> Option<VirtualNode> {
    match &node.data {
        NodeData::Text { contents } => Some(VirtualNode::text(contents.borrow().as_ref())),
        NodeData::Element { name, attrs, .. } => {
            let mut element = VElement::new(name.local.as_ref());
            for attr in attrs.borrow().iter() {
                let name = attr.name.local.to_string();
                let value = match attr.value.as_ref() {
                    "" => AttributeValue::Bool(true),
                    value => AttributeValue::String(value.to_string()),
                };
                element.attrs.insert(name, value);
            }
            element
                .children
                .extend(node.children.borrow().iter().filter_map(convert));
            Some(VirtualNode::from(element))
        }
        _ => None,
    }
}
