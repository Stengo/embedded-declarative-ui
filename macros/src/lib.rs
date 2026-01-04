use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(View)]
pub fn derive_view(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics embedded_declarative_ui::view::View<Color> for #name #ty_generics #where_clause 
        where 
            Color: embedded_graphics::pixelcolor::PixelColor,
        {
            async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
            where
                Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
                Error: 'static,
            {
                self.body().await.draw(size, draw_target).await
            }

            async fn size(&self, available_size: Size) -> Size {
                self.body().await.size(available_size).await
            }
        }
    };

    TokenStream::from(expanded)
}
