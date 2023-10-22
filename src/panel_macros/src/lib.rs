extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(GenerateUI)]
pub fn generate_property_ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let ui_code = match &input.data {
        syn::Data::Struct(s) => {
            match &s.fields {
                Fields::Named(fields) => {
                    fields.named.iter().map(|f| {
                        let field_name = &f.ident;
                        let field_type = &f.ty;
                        if format!("{}", quote!(#field_type)) == format!("{}", quote!(f32)) {
                            quote! {
                                let mut val = self.#field_name;
                                ui.add(egui::Slider::new(&mut val, 0.0..=100.0).text(stringify!(#field_name)));
                                if self.#field_name != val {
                                    log::info!("{} has changed from {} to {}", stringify!(#field_name), self.#field_name, val);
                                }
                                self.#field_name = val;
                            }
                        } else if format!("{}", quote!(#field_type)) == format!("{}", quote!(String)) {
                            quote! {
                                ui.text_edit_singleline(&mut self.#field_name);
                            }
                        } else {
                            quote!()
                        }
                    }).collect::<Vec<_>>()
                }
                _ => vec![],
            }
        }
        _ => vec![],
    };

    let expanded = quote! {
        impl #name {
            pub fn get_ui_drawer(&mut self) -> Box<dyn FnMut(&mut egui::Ui) + '_> {
                Box::new(move |ui: &mut egui::Ui| {
                    #(#ui_code)*
                })
            }
        }
    };

    expanded.into()
}
