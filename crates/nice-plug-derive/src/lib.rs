use proc_macro::TokenStream;

mod enums;
mod params;

/// Derive the `Enum` trait for simple enum parameters. See `EnumParam` for more information.
#[proc_macro_derive(Enum, attributes(name, id))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    enums::derive_enum(input)
}

/// Derive the `Params` trait for your plugin's parameters struct. See the `Plugin` trait.
#[proc_macro_derive(Params, attributes(id, persist, nested))]
pub fn derive_params(input: TokenStream) -> TokenStream {
    params::derive_params(input)
}

/// Returns the string literal in an attribute value such as `#[id = "foo"]`, looking through the
/// invisible groups that `macro_rules!` fragments are wrapped in.
fn lit_str(expr: &syn::Expr) -> Option<&syn::LitStr> {
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) => Some(s),
        syn::Expr::Group(group) => lit_str(&group.expr),
        _ => None,
    }
}
