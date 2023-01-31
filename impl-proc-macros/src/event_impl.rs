use proc_macro2::TokenStream;
use syn:: {
    Result,
    parse::{ Parse, ParseStream },
    parse_macro_input, LitStr
};
use quote::quote;

pub fn event(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parse_data = parse_macro_input!(tokens as EventParseData);
    gen_event_struct(&parse_data).into()
}

fn gen_event_struct(event: &EventParseData) -> TokenStream {
    let msg = &event.msg;

    quote! {
        crate::event_data::EventData {
            details: #msg.into(),
            line_number: core::line!(),
            function_name: "".into(),
            file_name: core::file!().into()
        }
    }
}

struct EventParseData {
    pub msg: String
}

impl Parse for EventParseData {
    fn parse(input: ParseStream) -> Result<Self> {
        let msg = input.parse::<LitStr>()?.value();
        Ok(EventParseData { msg })
    }
}