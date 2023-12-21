extern crate proc_macro;
use proc_macro::Group;
use proc_macro::*;
use std::iter::FromIterator;

struct CommandDataStruct {
    name: String,
    description: String,
    run: String,
}

static mut COMMAND_LIST: Vec<CommandDataStruct> = vec![];

#[proc_macro_attribute]
pub fn command(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args_iter = args.into_iter();

    let name = match args_iter.next() {
        Some(TokenTree::Literal(literal)) => literal.to_string(),
        Some(_token) => panic!("Expected literal but got something else"),
        None => panic!("Expected literal but got nothing"),
    };
    match args_iter.next() {
        Some(TokenTree::Punct(_punct)) => {}
        Some(_token) => panic!("Expected punct but got something else"),
        None => panic!("Expected punct but got nothing"),
    };

    let description = match args_iter.next() {
        Some(TokenTree::Literal(literal)) => literal.to_string(),
        Some(_token) => panic!("Expected literal but got something else"),
        None => panic!("Expected literal but got nothing"),
    };

    let mut item_iter = item.clone().into_iter();
    match item_iter.next() {
        Some(TokenTree::Ident(_ident)) => {}
        Some(_token) => panic!("Expected ident but got something else"),
        None => panic!("Expected ident but got nothing"),
    };

    let run = match item_iter.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        Some(_token) => panic!("Expected ident but got something else"),
        None => panic!("Expected ident but got nothing"),
    };

    unsafe {
        COMMAND_LIST.push(CommandDataStruct {
            name,
            description,
            run,
        })
    }

    item
}

fn render_command_fields(command: &CommandDataStruct) -> TokenStream {
    let mut tokens: Vec<TokenTree> = Vec::new();

    tokens.push(TokenTree::Ident(Ident::new("name", Span::call_site())));
    tokens.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
    tokens.push(TokenTree::Literal(command.name.parse::<Literal>().unwrap()));

    tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

    tokens.push(TokenTree::Ident(Ident::new(
        "description",
        Span::call_site(),
    )));
    tokens.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
    tokens.push(TokenTree::Literal(
        command.description.parse::<Literal>().unwrap(),
    ));
    tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

    tokens.push(TokenTree::Ident(Ident::new("run", Span::call_site())));
    tokens.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
    tokens.push(TokenTree::Ident(Ident::new(
        &command.run,
        Span::call_site(),
    )));
    tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

    TokenStream::from_iter(tokens)
}

fn render_commands() -> TokenStream {
    let mut tokens: Vec<TokenTree> = Vec::new();
    unsafe {
        for command in COMMAND_LIST.iter() {
            tokens.push(TokenTree::Ident(Ident::new("Command", Span::call_site())));
            tokens.push(TokenTree::Group(Group::new(
                Delimiter::Brace,
                render_command_fields(&command),
            )));
            tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
        }
    }

    TokenStream::from_iter(tokens)
}

#[proc_macro]
pub fn command_list(_item: TokenStream) -> TokenStream {
    let stream = TokenStream::from_iter([
        TokenTree::Punct(Punct::new('&', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Bracket, render_commands())),
    ]);
    stream
}
