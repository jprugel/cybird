use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, ItemStruct, parse_macro_input};
// Alternative: An attribute macro that adds the include automatically
#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    // Keep the original struct and add the include for generated exports
    let expanded = quote! {
        #input

        // Include the build script generated C exports
        include!(concat!(env!("OUT_DIR"), "/auto_exports.rs"));
    };

    TokenStream::from(expanded)
}

use syn::{Attribute, DeriveInput};

// ... existing plugin macro ...

#[proc_macro_derive(Registrable, attributes(registrable))]
pub fn derive_registrable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Look for #[registrable(variant_name)] or #[registrable(EnumName::VariantName)]
    let variant_info = extract_variant_info(&input.attrs);
    let (enum_name, variant_name) = match variant_info {
        Some((enum_name, variant_name)) => (enum_name, variant_name),
        None => {
            // Default to Registrable::TypeName if no attribute provided
            (quote! { Registrable }, name.clone())
        }
    };

    let expanded = quote! {
        impl #impl_generics Into<#enum_name> for #name #ty_generics #where_clause {
            fn into(self) -> #enum_name {
                #enum_name::#variant_name(self)
            }
        }

        impl #impl_generics FromRegistrable<#enum_name> for #name #ty_generics #where_clause {
            fn from_registrable(registrable: &#enum_name) -> Option<&Self> {
                match registrable {
                    #enum_name::#variant_name(item) => Some(item),
                    _ => None,
                }
            }
        }

        impl #impl_generics FromRegistrableMut<#enum_name> for #name #ty_generics #where_clause {
            fn from_registrable_mut(registrable: &mut #enum_name) -> Option<&mut Self> {
                match registrable {
                    #enum_name::#variant_name(item) => Some(item),
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_variant_info(attrs: &[Attribute]) -> Option<(proc_macro2::TokenStream, syn::Ident)> {
    for attr in attrs {
        if attr.path().is_ident("registrable")
            && let Ok(meta_list) = attr.meta.require_list()
        {
            let tokens = &meta_list.tokens;
            let token_str = tokens.to_string();

            if token_str.contains("::") {
                // Parse "EnumName::VariantName"
                let parts: Vec<&str> = token_str.split("::").collect();
                if parts.len() == 2 {
                    let enum_name: proc_macro2::TokenStream = parts[0].parse().unwrap();
                    let variant_name: syn::Ident = syn::parse_str(parts[1]).unwrap();
                    return Some((enum_name, variant_name));
                }
            } else {
                // Parse just "VariantName", assume Registrable enum
                if let Ok(variant_name) = syn::parse_str::<syn::Ident>(&token_str) {
                    return Some((quote! { Registrable }, variant_name));
                }
            }
        }
    }
    None
}

#[proc_macro_derive(Context, attributes(context))]
pub fn derive_context(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Extract context configuration
    let context_info = extract_context_info(&input.attrs, &input.data);
    let (registrable_type, storage_access) = match context_info {
        Some((reg_type, access)) => (reg_type, access),
        None => {
            return syn::Error::new_spanned(
                &input,
                "Context derive requires either a single Vec<T> field or #[context(registrable = T, field = field_name)] attribute"
            ).to_compile_error().into();
        }
    };

    let expanded = quote! {
        impl #impl_generics Context for #name #ty_generics #where_clause {
            type Registrable = #registrable_type;

            fn register<T>(&mut self, registrable: T)
            where
                T: Into<Self::Registrable>,
            {
                self.#storage_access.push(registrable.into());
            }

            fn get_registrables<T>(&self) -> Vec<&T>
            where
                T: FromRegistrable<Self::Registrable>,
            {
                self.#storage_access
                    .iter()
                    .filter_map(|registrable| T::from_registrable(registrable))
                    .collect()
            }

            fn get_registrables_mut<T>(&mut self) -> Vec<&mut T>
            where
                T: FromRegistrableMut<Self::Registrable>,
            {
                self.#storage_access
                    .iter_mut()
                    .filter_map(|registrable| T::from_registrable_mut(registrable))
                    .collect()
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_context_info(
    attrs: &[Attribute],
    data: &Data,
) -> Option<(syn::Type, proc_macro2::TokenStream)> {
    // First, check for explicit attributes
    for attr in attrs {
        if attr.path().is_ident("context") {
            if let Ok(meta_list) = attr.meta.require_list() {
                return parse_context_attribute(&meta_list.tokens);
            }
        }
    }

    // If no attributes, try to infer from struct fields
    if let Data::Struct(data_struct) = data {
        if let syn::Fields::Named(fields) = &data_struct.fields {
            // Look for a single Vec<T> field
            let vec_fields: Vec<_> = fields
                .named
                .iter()
                .filter_map(|field| {
                    if let syn::Type::Path(type_path) = &field.ty {
                        if let Some(segment) = type_path.path.segments.last() {
                            if segment.ident == "Vec" {
                                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments
                                {
                                    if let Some(syn::GenericArgument::Type(inner_type)) =
                                        args.args.first()
                                    {
                                        let field_name = field.ident.as_ref()?;
                                        let access = quote! { #field_name };
                                        return Some((inner_type.clone(), access));
                                    }
                                }
                            }
                        }
                    }
                    None
                })
                .collect();

            if vec_fields.len() == 1 {
                let (registrable_type, field_access) = &vec_fields[0];
                return Some((registrable_type.clone(), field_access.clone()));
            }
        } else if let syn::Fields::Unnamed(fields) = &data_struct.fields {
            // Handle tuple struct with single Vec<T> field (like PluginContext(Vec<Registrable>))
            if fields.unnamed.len() == 1 {
                if let syn::Type::Path(type_path) = &fields.unnamed[0].ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if segment.ident == "Vec" {
                            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(syn::GenericArgument::Type(inner_type)) =
                                    args.args.first()
                                {
                                    // For tuple structs, use numeric index
                                    let field_access = quote! { 0 };
                                    return Some((inner_type.clone(), field_access));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn parse_context_attribute(
    tokens: &proc_macro2::TokenStream,
) -> Option<(syn::Type, proc_macro2::TokenStream)> {
    // Parse something like: registrable = MyEnum, field = my_field
    let tokens_str = tokens.to_string();
    let mut registrable_type = None;
    let mut field_name = None;

    for part in tokens_str.split(',') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("registrable =") {
            registrable_type = syn::parse_str::<syn::Type>(value.trim()).ok();
        } else if let Some(value) = part.strip_prefix("field =") {
            if let Ok(ident) = syn::parse_str::<syn::Ident>(value.trim()) {
                field_name = Some(quote! { #ident });
            }
        }
    }

    match (registrable_type, field_name) {
        (Some(reg_type), Some(field)) => Some((reg_type, field)),
        _ => None,
    }
}
