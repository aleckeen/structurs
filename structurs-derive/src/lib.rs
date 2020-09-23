use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Endian attribute value.
enum Endian
{
  Little,
  Big,
  Native,
  Normal,
}

impl Default for Endian
{
  fn default() -> Self
  {
    Self::Normal
  }
}

/// Padding attribute value.
enum Padding
{
  Read,
  Bytes(usize),
}

impl Padding
{
  fn parse(attr: &syn::Attribute) -> Self
  {
    let mut padding = Padding::default();
    if let Some(proc_macro2::TokenTree::Group(g)) = attr.tokens.clone().into_iter().next() {
      let mut tokens = g.stream().into_iter();
      match tokens.next() {
        Some(proc_macro2::TokenTree::Ident(ref i)) => assert_eq!(i, "bytes"),
        token => panic!("expected ident was 'bytes', but found: {:?}", token),
      }
      match tokens.next() {
        Some(proc_macro2::TokenTree::Punct(ref p)) => assert_eq!(p.as_char(), '='),
        token => panic!("expected punct was '=', but found: {:?}", token),
      }
      let length: usize = match tokens.next() {
        Some(proc_macro2::TokenTree::Literal(l)) => match syn::Lit::new(l) {
          syn::Lit::Int(lit_int) => lit_int.base10_parse().unwrap(),
          lit => panic!("expected literal was of type integer, but found: {:?}", lit),
        },
        token => panic!("expected a literal, but found: {:?}", token),
      };
      padding = Padding::Bytes(length);
    }
    padding
  }
}

impl Default for Padding
{
  fn default() -> Self
  {
    Self::Read
  }
}

#[derive(Default)]
struct Attributes
{
  endian: Endian,
  padding: Option<Padding>,
}

impl Attributes
{
  fn new(attrs: &Vec<syn::Attribute>) -> Self
  {
    let mut attributes = Self::default();
    for attr in attrs {
      for segment in &attr.path.segments {
        if segment.ident == "le" {
          attributes.endian = Endian::Little
        } else if segment.ident == "be" {
          attributes.endian = Endian::Big
        } else if segment.ident == "ne" {
          attributes.endian = Endian::Native
        } else if segment.ident == "pad" {
          attributes.padding = Some(Padding::parse(attr));
        }
      }
    }
    attributes
  }
}

enum ArrayLength
{
  Int(usize),
  Const(syn::Expr),
}

#[proc_macro_derive(Read, attributes(le, be, ne, pad))]
pub fn derive_read_struct(input: TokenStream) -> TokenStream
{
  let ast = parse_macro_input!(input as DeriveInput);
  let struct_name = &ast.ident;
  let generics = &ast.generics;
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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

  // Fields to pass into struct construction block.
  let read_impl_fields = fields.iter().map(|f| {
    let field_name = &f.ident;
    // `elem_ty` is the type of the element if the field type is an array, otherwise it is the type
    // of the field. `elements` is the number of elements the array has and if it is not an array,
    // then it is simply 1;
    let (elem_ty, elements) = match array_type(&f.ty) {
      Some(elems) => elems,
      None => (&f.ty, ArrayLength::Int(1)),
    };

    // Read attributes passed to this field.
    let attrs = Attributes::new(&f.attrs);

    let read_func_token = read_func(elem_ty, &attrs.endian);
    let read_func_body = get_body(&read_func_token, elem_ty, &elements);

    let default_func_token = quote! { <#elem_ty as ::std::default::Default>::default() };
    let default_func_body = get_body(&default_func_token, elem_ty, &elements);

    let body = if let Some(pad) = attrs.padding {
      match pad {
        Padding::Read => {
          let elements_token = match &elements {
            ArrayLength::Int(size) => quote! { #size },
            ArrayLength::Const(expr) => quote! { #expr },
          };
          quote! { {
            const PAD_SIZE: usize = ::std::mem::size_of::<#elem_ty>() * #elements_token;
            let mut pad_buf: [u8; PAD_SIZE] = [0; PAD_SIZE];
            reader.read_exact(&mut pad_buf[..])?;
            #default_func_body }
          }
        }
        Padding::Bytes(bytes) => {
          quote! { {
            let mut pad_buf: [u8; #bytes] = [0; #bytes];
            reader.read_exact(&mut pad_buf)?;
            #default_func_body }
          }
        }
      }
    } else {
      quote! { #read_func_body }
    };

    quote! { #field_name: #body }
  });

  let expanded = quote! {
    impl #impl_generics ::structurs::Read for #struct_name #ty_generics #where_clause {
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

fn read_func(ty: &syn::Type, endian: &Endian) -> proc_macro2::TokenStream
{
  match endian {
    Endian::Little => quote! { <#ty as ::structurs::PrimitiveRead>::read_le(reader)? },
    Endian::Big => quote! { <#ty as ::structurs::PrimitiveRead>::read_be(reader)? },
    Endian::Native => quote! { <#ty as ::structurs::PrimitiveRead>::read_ne(reader)? },
    Endian::Normal => quote! { <#ty as ::structurs::Read>::read(reader)? },
  }
}

fn array_type(ty: &syn::Type) -> Option<(&syn::Type, ArrayLength)>
{
  if let syn::Type::Array(syn::TypeArray { elem, len, .. }) = ty {
    Some((
      elem,
      match len {
        syn::Expr::Lit(syn::ExprLit {
          lit: syn::Lit::Int(len),
          ..
        }) => ArrayLength::Int(len.base10_parse().unwrap_or_else(|err| {
          panic!("a parsing error occurred while reading the length of an array: {}", err);
        })),
        _ => ArrayLength::Const(len.clone()),
      },
    ))
  } else {
    None
  }
}

fn get_body(token: &proc_macro2::TokenStream, elem_ty: &syn::Type, ty_length: &ArrayLength)
  -> proc_macro2::TokenStream
{
  match ty_length {
    ArrayLength::Int(size) => {
      let tokens: Vec<proc_macro2::TokenStream> = (0..size.clone()).map(|_| token.clone()).collect();
      if tokens.len() == 1 {
        quote! { #(#tokens)* }
      } else {
        quote! { [ #(#tokens,)* ] }
      }
    }
    ArrayLength::Const(expr) => quote! { {
      let mut body: [#elem_ty; #expr] = [0; #expr];
      for i in 0..#expr {
        body[i] = #token;
      }
      body }
    },
  }
}
