use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, WhereClause};

#[proc_macro_derive(View)]
pub fn derive_view(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut generics = input.generics.clone();
    
    let color_param: GenericParam = parse_quote!(Color);
    generics.params.push(color_param);

    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut expanded_where_clause = where_clause.cloned().unwrap_or_else(|| WhereClause {
        where_token: <syn::Token![where]>::default(),
        predicates: syn::punctuated::Punctuated::new(),
    });

    expanded_where_clause.predicates.push(parse_quote! {
        #name #ty_generics: embedded_declarative_ui::view::CompositeView<Color>
    });
    expanded_where_clause.predicates.push(parse_quote! {
        Color: embedded_graphics::prelude::PixelColor
    });

    let expanded = quote! {
        impl #impl_generics embedded_declarative_ui::view::View<Color> for #name #ty_generics 
        #expanded_where_clause
        {
            async fn draw<Target, Error>(
                &self,
                size: embedded_graphics::prelude::Size,
                draw_target: &mut embedded_declarative_ui::draw_target::LayoutDrawTarget<'_, Target>,
            ) where
                Target: embedded_graphics::prelude::DrawTarget<Color = Color, Error = Error>
                    + embedded_graphics::prelude::OriginDimensions,
                Error: 'static,
            {
                self.body().await.draw(size, draw_target).await
            }

            async fn size(
                &self,
                available_size: embedded_graphics::prelude::Size,
            ) -> embedded_graphics::prelude::Size {
                self.body().await.size(available_size).await
            }
        }
    };

    TokenStream::from(expanded)
}
