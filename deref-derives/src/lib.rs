use proc_macro::TokenStream;
use syn::parse_macro_input;

mod deref;

/// Derive macro to implement the Deref trait
///
/// Use the `#[auto_ref]` attribute to mark the field to implement Deref for
///
/// # Examples
/// ```rust
/// use deref_derives::Deref;
///
/// #[derive(Deref)]
/// struct Hello<T> {
///     #[auto_ref]
///     inner: T,
/// }
/// ```
#[proc_macro_derive(Deref, attributes(auto_ref))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match deref::impl_deref_trait(&input, false) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive macro to implement the DerefMut trait
///
/// Use the `#[auto_ref]` attribute to mark the field to implement DerefMut for
///
/// Note: This macro automatically implements both Deref and DerefMut traits.
/// You don't need to separately derive Deref when using DerefMut.
///
/// # Examples
/// ```rust
/// use deref_derives::DerefMut;
///
/// #[derive(DerefMut)]
/// struct HelloMut<T> {
///     #[auto_ref]
///     inner: T,
/// }
/// // The above automatically implements both Deref and DerefMut
/// ```
#[proc_macro_derive(DerefMut, attributes(auto_ref))]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match deref::impl_deref_trait(&input, true) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
