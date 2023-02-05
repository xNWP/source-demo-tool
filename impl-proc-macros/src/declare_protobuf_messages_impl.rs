use proc_macro2::{ Span, TokenStream };
use syn:: {
    Ident,
    LitInt,
    Result,
    Token,
    Type,
    braced,
    parse_macro_input,
    parse::{ Parse, ParseStream }
};
use quote:: {
    quote,
    format_ident
};

pub fn declare_protobuf_messages(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let message_container = parse_macro_input!(tokens as MessagesContainer);

    let mut message_tokens = Vec::new();
    for msg in &message_container.messages {
        message_tokens.push(gen_message(&msg));
    }

    let message_id_tokens = gen_message_ids(&message_container);
    let message_container_tokens = gen_message_container(&message_container);

    quote! {
        use crate:: {
            protobuf_message::ProtobufMessage,
            parse_tools::{ parse_varint, ParseVarIntExit },
            event_data::EventData,
        };
        use source_demo_tool_impl_proc_macros::event;
        use buf_redux::BufReader;

        #( #message_tokens )*

        #message_id_tokens
        #message_container_tokens
    }.into()
}

fn gen_message_container(message_container: &MessagesContainer) -> TokenStream {
    let impl_to_string_tokens = gen_message_container_impl_to_string(&message_container);
    let parse_from_bufredux_reader_tokens = gen_message_container_parse_from_bufredux_reader(&message_container);
    let enum_declaration_tokens = gen_message_container_enum_declaration(&message_container);
    let parse_from_id_and_bufredux_reader_tokens = gen_message_container_parse_from_id_and_bufredux_reader(&message_container);

    let impl_protobuf_message_enum_traits_tokens = gen_message_container_impl_protobuf_message_traits(&message_container);

    let ident = message_container.get_ident();

    quote!(
        #enum_declaration_tokens
        #impl_to_string_tokens

        impl #ident {
            #parse_from_bufredux_reader_tokens
            #parse_from_id_and_bufredux_reader_tokens
        }

        #impl_protobuf_message_enum_traits_tokens
    ).into()
}

fn gen_message_container_impl_protobuf_message_traits
(msg_container: &MessagesContainer) -> TokenStream {
    let ident = msg_container.get_ident();

    let impl_to_vec_tokens = gen_message_container_impl_to_vec(&msg_container);
    let impl_type_count_tokens = gen_message_container_impl_type_count(&msg_container);

    quote!{
        impl crate::protobuf_message::ProtobufMessageEnumTraits
            for #ident {
                #impl_to_vec_tokens
                #impl_type_count_tokens
            }
        }
}

fn gen_message_container_impl_type_count(msg_container: &MessagesContainer) -> TokenStream {
    let count = msg_container.messages.len();
    quote! {
        fn type_count(&self) -> usize {
            #count
        }
    }
}

fn gen_message_container_enum_declaration(message_container: &MessagesContainer) -> TokenStream {
    let mut names = Vec::new();
    let mut data_names = Vec::new();
    for msg in &message_container.messages {
        names.push(msg.name.clone());
        data_names.push(msg.get_ident());
    }

    let ident = message_container.get_ident();

    quote!(
        #[derive(Debug, Clone)]
        pub enum #ident {
            #( #names ( #data_names ) ),*
        }
    ).into()
}

fn gen_message_container_parse_from_id_and_bufredux_reader(message_container: &MessagesContainer) -> TokenStream {
    let mut messages_match_tokens = Vec::new();

    for msg in &message_container.messages {
        let id_name = msg.name.to_string();
        let id_name = to_snake(id_name, true);
        let id_name = Ident::new(id_name.as_str(), Span::call_site());
        let struct_name = msg.get_ident();
        let enum_name = msg.name.clone();

        let container_ident = message_container.get_ident();

        messages_match_tokens.push(quote!(
            message_id:: #id_name => match #struct_name ::from_protobuf_messages(proto_messages) {
                Ok((value, warns)) => ( #container_ident :: #enum_name (value), warns),
                Err(e) => return Err(ExitErr::InvalidOrCorrupt(e))
            }
        ));
    }

    quote!(
        pub fn parse_from_id_and_bufredux_reader(id: u64, mut reader: &mut BufReader<&[u8]>)
            -> Result<(Self, crate::demo_file::packet::FromProtobufMessagesWarnings), crate::demo_file::packet::ParseMessageErr> {

            type ExitErr = crate::demo_file::packet::ParseMessageErr;

            let start_bytes = reader.buf_len();
            let mut proto_messages = Vec::new();

            while reader.buf_len() > 0 {
                let message = match ProtobufMessage::from_readable(&mut reader) {
                    Ok(pbm) => pbm,
                    Err(e) => return Err(ExitErr::InvalidOrCorrupt({
                        let mut ev = event!{""};
                        ev.details = format!{"bad data parse, proto_message: {:?}", e};
                        ev
                    }))
                };
                proto_messages.push(message);
            }

            let (message, warnings) = match id {
                #( #messages_match_tokens ),*
                n => return Err(ExitErr::UnknownCommand(n))
            };

            Ok(( message, warnings ))
        }
    ).into()
}

fn gen_message_container_parse_from_bufredux_reader(message_container: &MessagesContainer) -> TokenStream {
    let mut messages_match_tokens = Vec::new();

    for msg in &message_container.messages {
        let id_name = msg.name.to_string();
        let id_name = to_snake(id_name, true);
        let id_name = Ident::new(id_name.as_str(), Span::call_site());
        let struct_name = msg.get_ident();
        let enum_name = msg.name.clone();

        let container_ident = message_container.get_ident();

        messages_match_tokens.push(quote!(
            message_id:: #id_name => match #struct_name ::from_protobuf_messages(proto_messages) {
                Ok((value, warns)) => ( #container_ident :: #enum_name (value), warns),
                Err(e) => return Err(ExitErr::InvalidOrCorrupt(e))
            }
        ));
    }

    quote!(
        pub fn parse_from_bufredux_reader(mut reader: &mut BufReader<&[u8]>)
            -> Result<(Self, crate::demo_file::packet::FromProtobufMessagesWarnings), crate::demo_file::packet::ParseMessageErr> {

            type ExitErr = crate::demo_file::packet::ParseMessageErr;

            let network_command = match parse_varint(&mut reader) {
                ParseVarIntExit::Ok(n) => n,
                _ => return Err(ExitErr::InvalidOrCorrupt(event!{"bad varint read, network_command"}))
            };

            let command_size = match parse_varint(&mut reader) {
                ParseVarIntExit::Ok(n) => n,
                _ => return Err(ExitErr::InvalidOrCorrupt(event!{"bad varint read, command_size"}))
            };


            let start_bytes = reader.buf_len();
            let mut proto_messages = Vec::new();

            while (start_bytes - reader.buf_len()) < command_size as usize {
                let message = match ProtobufMessage::from_readable(&mut reader) {
                    Ok(pbm) => pbm,
                    Err(e) => return Err(ExitErr::InvalidOrCorrupt({
                        let mut ev = event!{""};
                        ev.details = format!{"bad data parse, proto_message: {:?}", e};
                        ev
                    }))
                };
                proto_messages.push(message);
            }

            if (start_bytes - reader.buf_len()) != command_size as usize {
                return Err(ExitErr::InvalidOrCorrupt(event!{"bad read, data read did not equal expected size"}))
            }

            let (message, warnings) = match network_command {
                #( #messages_match_tokens ),*
                n => return Err(ExitErr::UnknownCommand(n))
            };

            Ok(( message, warnings ))
        }
    ).into()
}

fn gen_message_container_impl_to_vec(message_container: &MessagesContainer) -> TokenStream {
    let enum_ident = message_container.get_ident();
    let mut match_funcs = Vec::new();

    for msg in &message_container.messages {
        let msg_ident = msg.name.clone();
        match_funcs.push(quote!{
            #enum_ident :: #msg_ident (msg_data) => msg_data.to_vec()
        });
    }

    quote!{
        fn to_vec(&self)
            -> Vec<(
                &'static str,
                crate::demo_file::packet::protobuf_value::ProtobufValue
            )> {
            match self {
                #( #match_funcs ),*
            }
        }
    }
}

fn gen_message_container_impl_to_string(message_container: &MessagesContainer) -> TokenStream {
    let mut names = Vec::new();
    let mut name_strings = Vec::new();
    for msg in &message_container.messages {
        names.push(msg.name.clone());
        name_strings.push(msg.name.to_string());
    }

    let ident = message_container.get_ident();

    quote!(
        impl ToString for #ident {
            fn to_string(&self) -> String {
                match *self {
                    #( Self:: #names (_) => #name_strings .into() ),*
                }
            }
        }
    ).into()
}

fn gen_message_ids(net_messages: &MessagesContainer) -> TokenStream {
    let mut id_symbols = Vec::new();
    for msg in &net_messages.messages {
        let name = msg.name.to_string();
        let name = to_snake(name, true);
        let name = Ident::new(name.as_str(), Span::call_site());
        let id = msg.id;
        id_symbols.push(quote!(
            pub const #name: u64 = #id;
        ));
    }

    quote!(
        mod message_id {
            #( #id_symbols )*
        }
    ).into()
}

fn gen_message(msg: &Message) -> TokenStream {
    let struct_def = gen_message_struct_def(&msg);
    let struct_impl = gen_message_struct_impl(&msg);
    let id_def = gen_id_def(&msg);

    quote! {
        #struct_def
        #struct_impl
        #id_def
    }.into()
}

fn gen_message_struct_def(msg: &Message) -> TokenStream {
    let struct_name = msg.get_ident();

    let mut fields = Vec::new();
    let mut sub_msgs = Vec::new();
    for f in &msg.fields {
        let name = Ident::new(f.name.as_str(), Span::call_site());

        let ty = f.get_type(!f.is_repeated);

        if f.wire_type == WireType::Proto {
            let sub_msg = Message {
                name: Ident::new(f.name.as_str(), Span::call_site()),
                fields: f.sub_messages.clone(),
                id: 0
            };
            sub_msgs.push(gen_message_struct_def(&sub_msg));
        }

        fields.push(quote!{
            pub #name: #ty
        });
    }

    quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #( #fields ),*
        }

        #( #sub_msgs )*
    }.into()
}

fn gen_message_struct_impl(msg: &Message) -> TokenStream {
    let struct_ident = msg.get_ident();
    let impl_from_protobuf_messages = gen_message_impl_from_protobuf_messages(msg);
    let impl_to_vec = gen_message_impl_to_vec(msg);

    let mut sub_msg_impls = Vec::new();
    for f in &msg.fields {
        if f.wire_type == WireType::Proto {
            let sub_msg = Message {
                name: Ident::new(f.name.as_str(), Span::call_site()),
                fields: f.sub_messages.clone(),
                id: 0
            };
            sub_msg_impls.push(gen_message_struct_impl(&sub_msg));
        }
    }

    quote!{
        impl #struct_ident {
            #impl_from_protobuf_messages
            #impl_to_vec
        }

        #( #sub_msg_impls )*
    }
}

fn gen_message_impl_to_vec(msg: &Message) -> TokenStream {
    let mut insert_funcs = Vec::new();
    for field in &msg.fields {
        let field_name_ident = Ident::new(
            format!("{}", field.name).as_str(),
            Span::call_site()
        );
        let field_name_string = &field.name;

        let value_inner_expr = match field.wire_type {
            WireType::VarInt  => quote!{ ProtobufValue:: VarInt(*v) },
            WireType::Length  => quote!{ ProtobufValue:: Length(v.clone()) },
            WireType::String  => quote!{ ProtobufValue:: String(v.clone()) },
            WireType::Fixed32 => quote!{ ProtobufValue::Fixed32(*v) },
            WireType::Float32 => quote!{ ProtobufValue::Float32(*v) },
            WireType::Proto   => quote!{ ProtobufValue::  Proto(v.to_vec()) },
        };

        let value_expr = {
            if field.is_repeated {
                quote! {
                    if self. #field_name_ident .is_empty() {
                        ProtobufValue::None
                    } else {
                        let mut proto_vals = Vec::new();
                        for v in &self. #field_name_ident {
                            proto_vals.push( #value_inner_expr );
                        }
                        ProtobufValue::Repeated(proto_vals)
                    }
                }
            } else {
                quote! {
                    match &self. #field_name_ident {
                        Some(v) => #value_inner_expr,
                        None => ProtobufValue::None
                    }
                }
            }
        };

        insert_funcs.push(quote! {
            {
                let name = #field_name_string;
                let val = #value_expr;
                rval.push((name, val));
            }
        })
    }

    quote! {
        pub fn to_vec(&self) -> Vec<
            (&'static str, crate::demo_file::packet::protobuf_value::ProtobufValue)
        > {
            use crate::demo_file::packet::protobuf_value::{
                ProtobufValue, ProtobufField
            };
            let mut rval = Vec::new();

            #( #insert_funcs )*

            rval
        }
    }
}

fn gen_message_impl_from_protobuf_messages(msg: &Message) -> TokenStream {
    let mut field_declarations = Vec::new();
    let mut field_match_funcs = Vec::new();
    let mut field_id_idents = Vec::new();
    let mut mandatory_field_checks = Vec::new();
    let mut field_names = Vec::new();

    for f in &msg.fields {
        let name_str = &f.name;
        let name = Ident::new(name_str.as_str(), Span::call_site());
        field_names.push(name.clone());

        if f.is_repeated {
            field_declarations.push(quote!{
                let mut #name = Vec::new();
            });
        } else {
            let ty = f.get_type(true);
            field_declarations.push(quote!{
                let mut #name: #ty = None;
            });
        }

        let id_ident = to_snake(f.name.clone(), true);
        let id_ident = Ident::new(id_ident.as_str(), Span::call_site());
        field_id_idents.push(id_ident);

        let if_let_path = match f.wire_type {
            WireType::VarInt => quote!{ crate::protobuf_message::WireMessage::VarInt(x) },
            WireType::Length | WireType::Proto | WireType::String => quote!{ crate::protobuf_message::WireMessage::Length(x) },
            WireType::Fixed32 | WireType::Float32 => quote!{ crate::protobuf_message::WireMessage::Fixed32(x) },
        };
        let set_value_transform = match f.wire_type {
            WireType::String => quote!{ String::from_utf8(x).unwrap() },
            WireType::Float32 => quote!{ f32::from_le_bytes(x.to_le_bytes()) },
            _ => quote!{ x }
        };

        let set_value;
        if f.wire_type == WireType::Proto {
            let tmp_msg = Message {
                fields: Vec::new(),
                id: 0,
                name: Ident::new(f.name.as_str(), Span::call_site())
            };
            let data_ident = tmp_msg.get_ident();

            let set_value_inner = match f.is_repeated {
                true => quote!{{ #name .push( #set_value_transform ); sub_warnings.push(warns); }},
                false => quote!{{ #name = Some( #set_value_transform ); sub_warnings.push(warns); }}
            };
            set_value = quote!{
                {
                    let tmp_msgs = match ProtobufMessage::many_from_vec(&x) {
                        Ok(v) => v,
                        Err(e) => return match e {
                            crate::protobuf_message::FromReadableErr::BufferErr(ev) => Err(ev),
                            crate::protobuf_message::FromReadableErr::InvalidOrCorrupt(ev) => Err(ev),
                            crate::protobuf_message::FromReadableErr::UnsupportedWireType(_ev, n) => {
                                    Err(crate::event_data::EventData {
                                        details: format!{"unsupported wire type: {}", n},
                                        line_number: line!(),
                                        function_name: "".into(),
                                        file_name: file!().into()
                                    })
                                }
                        }
                    };
                    match #data_ident ::from_protobuf_messages(tmp_msgs) {
                        Ok((x, warns)) => #set_value_inner,
                        Err(ev) => return Err(ev)
                    }
                }
            };
        } else {
            set_value = match f.is_repeated {
                true => quote!( #name .push( #set_value_transform ) ),
                false => quote!( #name = Some( #set_value_transform ) )
            };
        }
        field_match_funcs.push(quote! {
            {
                if let #if_let_path = msg.message {
                    #set_value
                } else {
                    return Err(source_demo_tool_impl_proc_macros::event!{"bad protobuf type"})
                }
            }
        });

        if !f.is_optional {
            let id = f.id;
            if f.is_repeated {
                mandatory_field_checks.push(quote! {
                    if #name.is_empty() {
                        missing_fields.push((#id, #name_str));
                    }
                });
            } else {
                mandatory_field_checks.push(quote! {
                    if #name.is_none() {
                        missing_fields.push((#id, #name_str));
                    }
                });
            }
        }
    }

    let id_ident = get_id_mod_ident(msg);
    let struct_ident = msg.get_ident();

    quote!{
        fn from_protobuf_messages(messages: Vec<ProtobufMessage>) -> Result<(Self, crate::demo_file::packet::FromProtobufMessagesWarnings), crate::event_data::EventData> {
            #( #field_declarations )*

            type FnWarns = crate::demo_file::packet::FromProtobufMessagesWarnings;
            let mut sub_warnings = Vec::new();
            let mut unknown_fields = Vec::new();

            for msg in messages {
                match msg.field_number {
                    #( #id_ident :: #field_id_idents => #field_match_funcs ),*
                    n => unknown_fields.push(msg)
                }
            }

            let mut missing_fields = Vec::new();
            #( #mandatory_field_checks )*

            Ok((
                #struct_ident { #( #field_names ),* },
                FnWarns { unknown_fields, missing_fields, sub_warnings }
            ))
        }
    }
}

fn gather_ids(msg: &Message) -> Vec<(Ident, u8)> {
    let mut ids = Vec::new();
    for f in &msg.fields {
        let field_name = to_snake(f.name.to_string(), true);
        let field_name = Ident::new(field_name.as_str(), Span::call_site());
        let id = f.id;
        ids.push((field_name, id));
    }
    ids
}

fn gen_id_def(msg: &Message) -> TokenStream {
    let module_name = get_id_mod_ident(msg);
    let mut names = Vec::new();
    let mut ids = Vec::new();
    let all_ids = gather_ids(&msg);
    for (i, u) in all_ids {
        names.push(i);
        ids.push(u);
    }

    let mut sub_msgs = Vec::new();
    for f in &msg.fields {
        if f.wire_type == WireType::Proto {
            sub_msgs.push(gen_id_def(&Message {
                fields: f.sub_messages.clone(),
                id: 0,
                name: Ident::new(f.name.as_str(), Span::call_site())
            }));
        }
    }

    quote!{
        mod #module_name {
            #( pub const #names: u8 = #ids; )*
        }

        #( #sub_msgs )*
    }
}

fn get_id_mod_ident(msg: &Message) -> Ident {
    let name = to_snake(msg.name.to_string(), false) + "_id";
    Ident::new(name.as_str(), Span::call_site())
}

fn to_snake(input: String, screaming: bool) -> String {
    let mut output = String::new();
    for c in input.as_bytes() {
        if c.is_ascii_uppercase() {
            if output.len() > 0 {
                if output.as_bytes()[output.len() - 1] != b'_' {
                    output.push('_');
                }
            }
        }
        let ch = match screaming {
            true => c.to_ascii_uppercase(),
            false => c.to_ascii_lowercase()
        };
        output.push(ch.into())
    }
    output
}

struct MessagesContainer {
    name: String,
    messages: Vec<Message>
}

impl MessagesContainer {
    fn get_ident(&self) -> Ident {
        Ident::new((self.name.clone() + "Message").as_str(), Span::call_site())
    }
}

impl Parse for MessagesContainer {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let name = name.to_string();
        let _: Token![,] = input.parse()?;

        let messages;
        let _ = braced!(messages in input);
        let messages = messages
            .parse_terminated::<Message, Token![,]>(Message::parse)
            .unwrap()
            .into_iter()
            .collect();

        Ok(MessagesContainer { name, messages })
    }
}

#[derive(Clone)]
struct Message {
    name: Ident,
    fields: Vec<MessageField>,
    id: u64
}

impl Message {
    fn get_ident(&self) -> Ident {
        format_ident!("{}Data", self.name)
    }
}

impl Parse for Message {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _ : Token![=] = input.parse()?;
        let id: LitInt = input.parse()?;
        let id: u64 = id.base10_parse()?;

        let fields;
        let _ = braced!(fields in input);
        let fields = fields
            .parse_terminated::<MessageField, Token![,]>(MessageField::parse)
            .unwrap()
            .into_iter()
            .collect();

        Ok(Message { name, fields, id })
    }
}

#[derive(Clone)]
struct MessageField {
    is_optional: bool,
    is_repeated: bool,
    name: String,
    wire_type: WireType,
    id: u8,
    sub_messages: Vec<MessageField>
}

impl MessageField {
    fn get_type(&self, optional: bool) -> Type {
        let mut ty: String = match self.wire_type {
            WireType::VarInt => "u64".into(),
            WireType::Length => "Vec<u8>".into(),
            WireType::Proto => self.name.clone() + "Data",
            WireType::Fixed32 => "u32".into(),
            WireType::String => "String".into(),
            WireType::Float32 => "f32".into(),
        };
        if self.is_repeated {
            ty = "Vec<".to_owned() + &ty + ">";
        }

        if optional {
            ty = "Option<".to_owned() + &ty + ">";
        }
        syn::parse_str::<Type>(ty.as_str()).unwrap()
    }
}

impl Parse for MessageField {
    fn parse(input: ParseStream) -> Result<Self> {
        let is_optional = input.parse::<kw::optional>().is_ok();
        let is_repeated = input.parse::<kw::repeated>().is_ok();

        let name = input.parse::<Ident>()?;
        let name = name.to_string();

        let _: Token![:] = input.parse()?;

        let wire_type;
        if input.parse::<kw::VarInt>().is_ok() {
            wire_type = WireType::VarInt;
        }
        else if input.parse::<kw::Length>().is_ok() {
            wire_type = WireType::Length;
        }
        else if input.parse::<kw::Proto>().is_ok() {
            wire_type = WireType::Proto;
        }
        else if input.parse::<kw::Fixed32>().is_ok() {
            wire_type = WireType::Fixed32;
        }
        else if input.parse::<kw::String>().is_ok() {
            wire_type = WireType::String;
        }
        else if input.parse::<kw::Float32>().is_ok() {
            wire_type = WireType::Float32;
        }
        else {
            panic!("invalid wire type")
        }

        let _: Token![=] = input.parse()?;

        let id: LitInt = input.parse()?;
        let id: u8 = id.base10_parse()?;

        const MAX_FIELD_ID: u8 = (1 << 5) - 1; // 5 bits max
        if id > MAX_FIELD_ID {
            panic!("max allowable field id is: {MAX_FIELD_ID}")
        }

        let sub_messages = {
            if wire_type == WireType::Proto {
                let parse_buffer;
                let _ = braced!(parse_buffer in input);
                parse_buffer
                    .parse_terminated::<MessageField, Token![,]>(MessageField::parse)
                    .unwrap()
                    .into_iter()
                    .collect()
            }
            else {
                Vec::new()
            }
        };

        Ok(MessageField { is_optional, is_repeated, name, wire_type, id, sub_messages })
    }
}

mod kw {
    use syn::custom_keyword;
    custom_keyword!(VarInt);
    custom_keyword!(Length);
    custom_keyword!(Fixed32);
    custom_keyword!(Proto);
    custom_keyword!(String);
    custom_keyword!(Float32);
    custom_keyword!(optional);
    custom_keyword!(repeated);
}

#[derive(Clone, PartialEq, Eq)]
enum WireType {
    VarInt,
    Length,
    Proto,
    Fixed32,
    String,
    Float32,
}
