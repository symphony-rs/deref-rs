use proc_macro::TokenStream;
use syn::parse_macro_input;

mod deref;

/// Derive macro to implement the Deref trait
///
/// Use the `#[deref]` attribute to mark the field to implement Deref for
///
/// # Examples
/// ```rust
/// use deref::Deref;
///
/// #[derive(Deref)]
/// struct Hello<T> {
///     #[deref]
///     inner: T,
/// }
/// ```
#[proc_macro_derive(Deref, attributes(deref))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match deref::impl_deref_trait(&input, false) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive macro to implement the DerefMut trait
///
/// Use the `#[deref_mut]` attribute to mark the field to implement DerefMut for
///
/// # Examples
/// ```rust
/// use deref::DerefMut;
///
/// #[derive(DerefMut)]
/// struct HelloMut<T> {
///     #[deref_mut]
///     inner: T,
/// }
/// ```
#[proc_macro_derive(DerefMut, attributes(deref_mut))]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match deref::impl_deref_trait(&input, true) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
