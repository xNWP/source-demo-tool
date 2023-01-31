mod declare_protobuf_messages_impl;
mod event_impl;

#[proc_macro]
pub fn declare_protobuf_messages(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    declare_protobuf_messages_impl::declare_protobuf_messages(tokens)
}

#[proc_macro]
pub fn event(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    event_impl::event(tokens)
}