use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

enum Endian
{
  Little,
  Big,
  Native,
  Normal,
}

/// This derive macro reads the fields of a struct and generates a valid
/// [`structurs::Read::read`] function. All of the field types must implement the
/// [`structurs::Read`] trait. Only named structures are supported at this point, but I am
/// thinking aboout expanding the supported types.
#[proc_macro_derive(Read, attributes(le, be, ne))]
pub fn derive_read_struct(input: TokenStream) -> TokenStream
{
  let ast = parse_macro_input!(input as DeriveInput);
  let struct_name = &ast.ident;

  // fields of the input struct must be named (at least for now).
  let fields = if let syn::Data::Struct(syn::DataStruct {
    fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
    ..
  }) = ast.data
  {
    named
  } else {
    panic!("'Read' derive macro only supports structs with named fields.");
  };

  let read_impl_fields = fields.iter().map(|f| {
    let field_name = &f.ident;
    let field_ty = &f.ty;

    let mut endian = Endian::Normal;
    for attr in &f.attrs {
      for segment in &attr.path.segments {
        endian = if segment.ident == "le" {
          Endian::Little
        } else if segment.ident == "be" {
          Endian::Big
        } else if segment.ident == "ne" {
          Endian::Native
        } else {
          continue;
        }
      }
    }

    // If the field type is an array, extract the length and element type of the array.
    if let syn::Type::Array(syn::TypeArray {
      ref elem,
      len: syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(len),
        ..
      }),
      ..
    }) = field_ty
    {
      let len = len.base10_parse().unwrap_or_else(|err| {
        panic!("a parsing error occurred while reading the length of an array: {}", err);
      });
      let mut array_fields = Vec::new();
      for _ in 0..len {
        array_fields.push(read_func(elem, &endian));
      }

      // Generate array reader.
      quote! {
        #field_name: [
          #(#array_fields,)*
        ]
      }
    } else {
      let func = read_func(field_ty, &endian);
      // Generate normal field reader.
      quote! {
        #field_name: #func
      }
    }
  });

  let expanded = quote! {
    impl ::structurs::Read for #struct_name {
      fn read<R>(reader: &mut R) -> ::std::io::Result<Self>
      where
        R: ::std::io::Read
      {
        Ok(Self {
          #(#read_impl_fields,)*
        })
      }
    }
  };

  expanded.into()
}

/// Return the appropriate read function depending on the endiannes.
fn read_func(ty: &syn::Type, endian: &Endian) -> proc_macro2::TokenStream
{
  match endian {
    Endian::Little => quote! {<#ty as ::structurs::PrimitiveRead>::read_le(reader)?},
    Endian::Big => quote! {<#ty as ::structurs::PrimitiveRead>::read_be(reader)?},
    Endian::Native => quote! {<#ty as ::structurs::PrimitiveRead>::read_ne(reader)?},
    Endian::Normal => quote! {<#ty as ::structurs::Read>::read(reader)?},
  }
}
