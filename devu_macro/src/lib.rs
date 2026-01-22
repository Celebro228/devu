use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;
use quote::format_ident;


#[proc_macro_attribute]
pub fn startup(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper = format_ident!("__start_{}", name);

    quote! {
        #func

        #[distributed_slice(::devu::ecs::START)]
        fn #wrapper() -> ::bevy_ecs::system::ScheduleSystem {
            Box::new(::bevy_ecs::system::IntoSystem::into_system(#name))
        }
    }.into()
}


#[proc_macro_attribute]
pub fn update(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper = format_ident!("__update_{}", name);

    quote! {
        #func

        #[distributed_slice(::devu::ecs::UPDATE)]
        fn #wrapper() -> ::bevy_ecs::system::ScheduleSystem {
            Box::new(::bevy_ecs::system::IntoSystem::into_system(#name))
        }
    }.into()
}
