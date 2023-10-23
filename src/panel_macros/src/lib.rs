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
                                ui.add(egui::Slider::new(&mut val, 0.0..=100.0).text(stringify!(#field_name)).integer());
                                if self.#field_name != val {
                                    log::info!("{} has changed from {} to {}", stringify!(#field_name), self.#field_name, val);
                                }
                                self.#field_name = val;
                            }
                        } else if format!("{}", quote!(#field_type)) == format!("{}", quote!(String)) {
                            quote! {
                                let mut s = self.#field_name.clone();
                                ui.horizontal(|ui| {
                                    ui.label(stringify!(#field_name));
                                    ui.add(
                                    egui::TextEdit::singleline(&mut s)
                                        .cursor_at_end(true)
                                        .hint_text(stringify!(#field_name)));
                                });
                                if s != "".to_string() {
                                    self.#field_name = s;
                                } 
                            }
                        } else if format!("{}", quote!(#field_type)) == format!("{}", quote!(Pos2)) {
                            quote! {
                                let mut val_x = self.#field_name.x;
                                let mut val_y = self.#field_name.y;
                                //ui.group(|ui|{
                                    let mut layout = egui::Layout::left_to_right(egui::Align::Center)
                    .with_cross_align(egui::Align::Center)
                    .with_cross_justify(false)
                    .with_main_wrap(true).with_main_align(egui::Align::Center);
                                    ui.allocate_ui(
                                        egui::vec2(ui.available_size_before_wrap().x, 20.),
                                            |ui| ui.with_layout(layout, |ui| {
                                            ui.add(egui::DragValue::new(&mut val_x).clamp_range(0..=100).prefix("x: "));
                                            ui.add(egui::DragValue::new(&mut val_y).clamp_range(0..=100).prefix("y: "));
                                            if self.#field_name.x != val_x {
                                                log::info!("{} has changed from {} to {}", stringify!(#field_name.x), self.#field_name.x, val_x);
                                                self.#field_name.x = val_x;
                                            }
                                            if self.#field_name.y != val_y {
                                                log::info!("{} has changed from {} to {}", stringify!(#field_name.y), self.#field_name.y, val_y);
                                                self.#field_name.y = val_y;
                                            }
                                        }));
                                //});
                            }
                        }
                        else if format!("{}", quote!(#field_type)) == format!("{}", quote!(egui::Color32)) {
                            quote! {
                                let mut val = self.#field_name;
                                ui.color_edit_button_srgba(&mut val);
                                self.#field_name = val;
                            }
                        }
                        else {
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
