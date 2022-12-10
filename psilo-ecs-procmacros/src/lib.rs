use proc_macro::TokenStream;
use proc_macro_crate::crate_name;
use proc_macro_error::{emit_warning, emit_error, proc_macro_error};
use proc_macro2::{
    TokenStream as TokenStream2,
    Punct,
    Spacing,
    Span,
};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input,
    Expr,
    Ident,
    Token,
    Type,
};

enum AccessLevel {
    Prev, Cur, Mut,
}

impl ToTokens for AccessLevel {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = match self {
            AccessLevel::Prev => "Prev",
            AccessLevel::Cur => "Cur",
            AccessLevel::Mut => "Mut",
        };
        tokens.append(Ident::new(name, Span::call_site()));
    }
}

struct Accessor {
    level: AccessLevel,
    optional: bool,
    typ: Type,
    span: Span,
}

impl Accessor {
    fn unfolded(&self) -> TokenStream2 {
        let level = &self.level;
        let optional = &self.optional;
        let typ = &self.typ;
        let helper = match (level, optional) {
            (AccessLevel::Prev, false) => "unsafe_unwrap_shared",
            (AccessLevel::Cur, false) => "unsafe_unwrap_shared_gd",
            (AccessLevel::Mut, false) => "unsafe_unwrap_exclusive_gd",
            (AccessLevel::Prev, true) => "unsafe_optional_shared",
            (AccessLevel::Cur, true) => "unsafe_optional_shared_gd",
            (AccessLevel::Mut, true) => "unsafe_optional_exclusive_gd",
        };
        let helper = Ident::new(helper, Span::call_site());
        quote!{
            iter.next().unwrap().#helper::<#typ>()
        }
    }
}

impl Parse for Accessor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let level_ident: Ident = input.call(Ident::parse_any)?;
        let mut optional = None;
        let level = if level_ident == "prev" { AccessLevel::Prev }
        else if level_ident == "cur" { AccessLevel::Cur }
        else if level_ident == "mut" { AccessLevel::Mut }
        else if level_ident == "prev_optional" { optional = Some(true); AccessLevel::Prev }
        else if level_ident == "cur_optional" { optional = Some(true); AccessLevel::Cur }
        else if level_ident == "mut_optional" { optional = Some(true); AccessLevel::Mut }
        else {
            return Err(syn::Error::new(level_ident.span(), "expected \"prev\", \"cur\", or \"mut\""))
        };
        let (optional, trailing_bracket) = if let Some(optional) = optional {
            emit_warning!(level_ident.span(),
                          "\"_optional\" suffix is deprecated, \
                          use Option<T> as component type instead");
            (optional, false)
        }
        else {
            let option_attempt = input.fork();
            let optional = match option_attempt.parse::<Ident>() {
                Err(_) => false,
                Ok(x) => x == "Option",
            };
            if optional {
                assert_eq!(input.parse::<Ident>()?, "Option");
                input.parse::<Token![<]>()?;
                (true, true)
            }
            else {
                (false, false)
            }
        };
        let typ: Type = input.parse()?;
        if trailing_bracket {
            input.parse::<Token![>]>()?;
        }
        Ok(Accessor {
            level, optional, typ, span: level_ident.span()
        })
    }
}

/// Argument list that has a given number of expressions on the left, and a
/// given number (or 0 for any nonzero number) of *component accesses* on the
/// right.
struct Comped<const LEFT: usize, const RIGHT: usize> {
    exprs: Vec<Expr>,
    accessors: Vec<Accessor>,
}

impl<const LEFT: usize, const RIGHT: usize> Parse for Comped<LEFT, RIGHT> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut exprs = vec![];
        for _ in 0 .. LEFT {
            exprs.push(input.parse()?);
            input.parse::<Token![,]>()?;
        }
        let mut accessors: Vec<Accessor> = vec![];
        loop {
            accessors.push(input.parse()?);
            if input.is_empty() { break }
            input.parse::<Token![,]>()?;
            if input.is_empty() { break }
            if RIGHT > 0 && accessors.len() >= RIGHT {
                return Err(input.error(format!("This macro requires exactly {} component accessor{}.", RIGHT, if RIGHT == 1 { "" } else { "s" })));
            }
        }
        if RIGHT > 0 && accessors.len() != RIGHT {
            return Err(input.error(format!("This macro requires exactly {} component accessor{}.", RIGHT, if RIGHT == 1 { "" } else { "s" })));
        }
        assert!(accessors.len() > 0);
        if accessors[0].optional {
            emit_error!(accessors[0].span, "The first component must not be optional");
        }
        Ok(Comped {
            exprs, accessors
        })
    }
}

fn make_iterants(accessors: &[Accessor]) -> TokenStream2 {
    let mut ret = TokenStream2::new();
    for (i, accessor) in accessors.iter().enumerate() {
        if i != 0 {
            ret.append(Punct::new(',', Spacing::Alone));
        }
        let level = &accessor.level;
        let optional = &accessor.optional;
        let typ = &accessor.typ;
        use proc_macro_crate::FoundCrate;
        let k = match crate_name("psilo-ecs") {
            Err(_) | Ok(FoundCrate::Itself) => quote!{crate},
            Ok(FoundCrate::Name(wat)) => {
                let q = Ident::new(&wat, Span::call_site());
                quote!{#q}
            },
        };
        ret.append_all(quote!{
            #k::iter::ComponentAccess::#level(std::any::TypeId::of::<#typ>(), #optional)
        });
    }
    ret
}

fn make_unfolded(accessors: &[Accessor]) -> TokenStream2 {
    let mut ret = TokenStream2::new();
    for (i, accessor) in accessors.iter().enumerate() {
        if i != 0 {
            ret.append(Punct::new(',', Spacing::Alone));
        }
        ret.append_all(accessor.unfolded());
    }
    ret
}

#[proc_macro_error]
#[proc_macro]
pub fn ecs_iter(input: TokenStream) -> TokenStream {
    let mut comped = parse_macro_input!(input as Comped<1, 0>);
    assert_eq!(comped.exprs.len(), 1);
    let world = comped.exprs.remove(0);
    let iterants = make_iterants(&comped.accessors);
    let unfolded = make_unfolded(&comped.accessors);
    TokenStream::from(quote! {
        {
            let world = #world.as_ref();
            let iterants = [#iterants];
            let iterator = world.iterate_on(iterants);
            iterator.custom_map(|(eid, array)| {
                let mut iter = array.into_iter();
                unsafe { (eid, #unfolded) }
            })
        }
    })
}

#[proc_macro_error]
#[proc_macro]
pub fn ecs_get(input: TokenStream) -> TokenStream {
    let mut comped = parse_macro_input!(input as Comped<2, 0>);
    assert_eq!(comped.exprs.len(), 2);
    let eid = comped.exprs.remove(1);
    let world = comped.exprs.remove(0);
    let iterants = make_iterants(&comped.accessors);
    let unfolded = make_unfolded(&comped.accessors);
    TokenStream::from(quote! {
        {
            let world = #world.as_ref();
            let iterants = [#iterants];
            let found = world.get_entity(#eid, iterants);
            found.map(|array| {
                let mut iter = array.into_iter();
                unsafe { (#unfolded) }
            })
        }
    })
}

// TODO: this can be optimized significantly, probably won't speed up runtime
// but would speed up compile
#[proc_macro_error]
#[proc_macro]
pub fn ecs_singleton(input: TokenStream) -> TokenStream {
    let mut comped = parse_macro_input!(input as Comped<1, 1>);
    assert_eq!(comped.exprs.len(), 1);
    assert_eq!(comped.accessors.len(), 1);
    let world = comped.exprs.remove(0);
    let iterants = make_iterants(&comped.accessors);
    let unfolded = comped.accessors[0].unfolded();
    let typ = &comped.accessors[0].typ;
    let type_name = quote!{#typ}.to_string();
    TokenStream::from(quote! {
        {
            let world = #world.as_ref();
            let iterants = [#iterants];
            let mut iterator = world.iterate_on(iterants)
            .custom_map(|(_eid, array)| {
                let mut iter = array.into_iter();
                unsafe { #unfolded }
            });
            if let Some(result) = iterator.next() {
                if cfg!(debug_assertions) && iterator.next().is_some() {
                    panic!("Multiple instances of singleton component {:?}", #type_name);
                }
                result
            }
            else {
                panic!("Missing singleton component {:?}", #type_name);
            }
        }
    })
}