use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;
use quote::format_ident;


#[proc_macro_attribute]
pub fn pre_startup(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper = format_ident!("__pre_start_{}", name);

    quote! {
        #func

        #[distributed_slice(::devu::ecs::PRE_START)]
        fn #wrapper() -> ::bevy_ecs::system::ScheduleSystem {
            Box::new(::bevy_ecs::system::IntoSystem::into_system(#name))
        }
    }.into()
}

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
pub fn post_startup(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper = format_ident!("__post_start_{}", name);

    quote! {
        #func

        #[distributed_slice(::devu::ecs::POST_START)]
        fn #wrapper() -> ::bevy_ecs::system::ScheduleSystem {
            Box::new(::bevy_ecs::system::IntoSystem::into_system(#name))
        }
    }.into()
}


#[proc_macro_attribute]
pub fn pre_update(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper = format_ident!("__pre_update_{}", name);

    quote! {
        #func

        #[distributed_slice(::devu::ecs::PRE_UPDATE)]
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

#[proc_macro_attribute]
pub fn post_update(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper = format_ident!("__post_update_{}", name);

    quote! {
        #func

        #[distributed_slice(::devu::ecs::POST_UPDATE)]
        fn #wrapper() -> ::bevy_ecs::system::ScheduleSystem {
            Box::new(::bevy_ecs::system::IntoSystem::into_system(#name))
        }
    }.into()
}


#[proc_macro_attribute]
pub fn event(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper = format_ident!("__event_{}", name);

    quote! {
        #func

        #[distributed_slice(::devu::ecs::EVENT)]
        fn #wrapper(app: &mut ::devu::ecs::App) {
            app.add_observer(#name);
        }
    }.into()
}