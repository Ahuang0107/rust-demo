mod content_types;
mod doc_props;
mod xl;

pub struct Excel {
    pub doc_props: doc_props::DocProps,
    pub xl: xl::Xl,
    pub content_types: content_types::ContentTypes,
}
