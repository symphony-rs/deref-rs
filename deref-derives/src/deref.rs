use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DataStruct, DeriveInput, Fields, Index, Type};

/// Unified implementation function for Deref and DerefMut traits
pub fn impl_deref_trait(input: &DeriveInput, is_mut: bool) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let generics = &input.generics;
    let attr_name = if is_mut { "deref_mut" } else { "deref" };
    let trait_name = if is_mut { "DerefMut" } else { "Deref" };

    // Get struct fields
    let fields = match &input.data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                format!("{} can only be used on structs", trait_name),
            ));
        }
    };

    // Find the field marked with #[deref] or #[deref_mut]
    let (deref_field, field_type) = find_deref_field(fields, attr_name)?;

    // Generate implementation code
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Basic Deref implementation
    let deref_impl = quote! {
        impl #impl_generics std::ops::Deref for #name #ty_generics #where_clause {
            type Target = #field_type;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.#deref_field
            }
        }
    };

    // If DerefMut, also need to implement DerefMut trait
    if is_mut {
        let deref_mut_impl = quote! {
            impl #impl_generics std::ops::DerefMut for #name #ty_generics #where_clause {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#deref_field
                }
            }
        };

        Ok(quote! {
            #deref_impl
            #deref_mut_impl
        })
    } else {
        Ok(deref_impl)
    }
}

/// Function to find the field marked with the specified attribute
fn find_deref_field(fields: &Fields, attr_name: &str) -> syn::Result<(DerefField, Type)> {
    match fields {
        Fields::Named(fields_named) => {
            let mut deref_field = None;
            let mut field_type = None;

            for field in &fields_named.named {
                if has_attribute(&field.attrs, attr_name) {
                    if deref_field.is_some() {
                        return Err(syn::Error::new_spanned(
                            field,
                            format!("Only one field can be marked with #[{}]", attr_name),
                        ));
                    }

                    let ident = field
                        .ident
                        .clone()
                        .ok_or_else(|| syn::Error::new_spanned(field, "Field must have a name"))?;

                    deref_field = Some(DerefField::Named(ident));
                    field_type = Some(field.ty.clone());
                }
            }

            match (deref_field, field_type) {
                (Some(field), Some(ty)) => Ok((field, ty)),
                _ => Err(syn::Error::new_spanned(
                    fields_named,
                    format!("Must have one field marked with #[{}]", attr_name),
                )),
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            let mut deref_index = None;
            let mut field_type = None;

            for (index, field) in fields_unnamed.unnamed.iter().enumerate() {
                if has_attribute(&field.attrs, attr_name) {
                    if deref_index.is_some() {
                        return Err(syn::Error::new_spanned(
                            field,
                            format!("Only one field can be marked with #[{}]", attr_name),
                        ));
                    }

                    deref_index = Some(index);
                    field_type = Some(field.ty.clone());
                }
            }

            match (deref_index, field_type) {
                (Some(index), Some(ty)) => Ok((DerefField::Unnamed(index), ty)),
                _ => Err(syn::Error::new_spanned(
                    fields_unnamed,
                    format!("Must have one field marked with #[{}]", attr_name),
                )),
            }
        }
        Fields::Unit => Err(syn::Error::new_spanned(fields, "Unit structs are not supported")),
    }
}

/// Type representing the Deref field
enum DerefField {
    Named(syn::Ident),
    Unnamed(usize),
}

impl quote::ToTokens for DerefField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            DerefField::Named(ident) => ident.to_tokens(tokens),
            DerefField::Unnamed(index) => {
                let index = Index::from(*index);
                index.to_tokens(tokens);
            }
        }
    }
}

/// Function to check if an attribute exists
fn has_attribute(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(name))
}
