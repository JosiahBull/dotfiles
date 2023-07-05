/// A simple proc macro to create a 'new' method which returns a static reference to a struct.
/// e.g.
///
/// ```
/// use singleton_derive::Singleton;
///
/// #[derive(Singleton)]
/// struct MyStruct {
///    name: String,
/// }
///
/// impl Default for MyStruct {
///     fn default() -> Self {
///         Self { name: "".to_string() }
///     }
/// }
/// ```
///
/// Will generate:
/// ```
/// static MYSTRUCT_SINGLETON: std::sync::OnceLock<&'static MyStruct> = std::sync::OnceLock::new();
///
/// struct MyStruct {
///    name: String,
/// }
///
/// impl Default for MyStruct {
///     fn default() -> Self {
///         Self { name: "".to_string() }
///     }
/// }
///
/// impl MyStruct {
///    pub fn new() -> &'static Self {
///       MYSTRUCT_SINGLETON.get_or_init(|| {
///         Box::leak(Box::new(MyStruct::default()))
///      })
///   }
/// }
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Singleton)]
pub fn singleton(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let struct_name = &input.ident;

    // Generate the static identifier for the singleton instance, converting CamelCase to snake_case
    let static_ident = syn::Ident::new(
        &format!("__{}",
        struct_name
            .to_string()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .to_uppercase()),
        struct_name.span(),
    );

    // Check if the struct implements the Default trait
    let mut impl_default = false;
    if let Data::Struct(ref data_struct) = input.data {
        if data_struct.fields.iter().any(|f| f.ident.is_some()) {
            impl_default = true;
        }
    }

    if !impl_default {
        // Error when Default trait is not implemented
        let error = syn::Error::new_spanned(
            input,
            "Singleton can only be derived for structs that implement the Default trait",
        );
        return TokenStream::from(error.to_compile_error());
    }

    // Generate the output tokens
    let output = quote! {
        // Generate the static singleton instance
        static #static_ident: std::sync::OnceLock<&'static #struct_name> = std::sync::OnceLock::new();

        // Implement the Singleton trait for the struct
        impl #struct_name {
            pub fn singleton() -> &'static Self {
                #static_ident.get_or_init(|| {
                    ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                })
            }
        }
    };

    // Return the output tokens as a TokenStream
    output.into()
}
