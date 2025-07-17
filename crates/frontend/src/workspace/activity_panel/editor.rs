use std::rc::Rc;

use codemirror_sys::{autocomplete, commands, lang_python, language, search, state, view};
use dominator::{clone, html, stylesheet, svg, Dom};
use futures_signals::signal::{self, Signal, SignalExt};
use wasm_bindgen::prelude::*;

// remove this
macro_rules! object(
    { $($key:expr => $value:expr),+ $(,)?} => {
        {
            let object = ::js_sys::Object::new();
            $(
                let key = <_ as ::core::convert::Into<::wasm_bindgen::JsValue>>::into($key);
                let value = <_ as ::core::convert::Into<::wasm_bindgen::JsValue>>::into($value);
                ::js_sys::Reflect::set(&object, &key, &value).unwrap();
            )+
            object
        }
    }
);

pub struct Editor {
    pub file: Rc<crate::vfs::File>,
}

impl Editor {
    pub fn new(file: Rc<crate::vfs::File>) -> Editor {
        Editor {
            file
        }
    }

    pub fn render(
        this: &Rc<Editor>
    ) -> impl Signal<Item = Option<dominator::Dom>> {
        stylesheet!(".cm-editor", {
            .style("height", "100%")
            .style_important("outline", "none")
        });
        
        let update_closure = clone!(this => move |update: view::ViewUpdate| {
            if update.doc_changed() {
                // autosave
                let data = update.state().doc().to_string().as_bytes().to_vec();
                this.file.data.replace(data);
            }
        });

        let data = String::from_utf8(this.file.data.get_cloned()).unwrap();
    
        let language = state::Compartment::new();
        let state = state::EditorState::create(&object! {
            "doc" => JsValue::from(data),
            "extensions" => [
                view::line_numbers(),
                view::highlight_active_line_gutter(),
                view::highlight_special_chars(),
                commands::history(),
                language::fold_gutter(),
                view::draw_selection(),
                view::drop_cursor(),
                language::indent_on_input(),
                language::syntax_highlighting(&language::DEFAULT_HIGHLIGHT_STYLE.with(|style| style.clone()), None),
                language::bracket_matching(),
                autocomplete::close_brackets(),
                autocomplete::autocompletion(),
                view::rectangular_selection(),
                view::crosshair_cursor(),
                view::highlight_active_line(),
                search::highlight_selection_matches(),
                view::KEYMAP.with(|keymap| keymap.of(&js_sys::Array::new()
                    .concat(&autocomplete::CLOSE_BRACKETS_KEYMAP.with(|keymap_option| keymap_option.clone()))
                    .concat(&commands::DEFAULT_KEYMAP.with(|keymap_option| keymap_option.clone()))
                    .concat(&search::SEARCH_KEYMAP.with(|keymap_option| keymap_option.clone()))
                    .concat(&commands::HISTORY_KEYMAP.with(|keymap_option| keymap_option.clone()))
                    .concat(&language::FOLD_KEYMAP.with(|keymap_option| keymap_option.clone()))
                    .concat(&autocomplete::COMPLETION_KEYMAP.with(|keymap_option| keymap_option.clone()))
                    .concat(&js_sys::Array::of1(&commands::IDENT_WITH_TAB.with(|keymap_option| keymap_option.clone())))),
                ),
                view::EditorView::update_listener()
                    .of(&Closure::<dyn Fn(_)>::new(update_closure).into_js_value()),
                /* dynamic options */
                language.of(&lang_python::python()),
            ].into_iter().collect::<js_sys::Array>(),
        });
        
        let view = view::EditorView::new(&object! {
            "state" => state,
        });

        signal::always(Some(html!("div", {
            .class("block")
            .class("h-full")
            // to check if the file contents changed and to change the view to reflect that
            .future(this.file.data.signal_cloned().for_each(clone!(view => move |new_data| {
                match String::from_utf8(new_data) {
                    Ok(new_text) => {
                        let state = view.state();
                        let current_text = state.doc().to_string();

                        if current_text != new_text {
                            let transaction = object! {
                                "changes" => {
                                    object! {
                                        "from" => 0,
                                        "to" => state.doc().length(),
                                        "insert" => new_text
                                    }
                                }
                            };
                            view.dispatch(&transaction);
                        }
                    },
                    Err(err) => {
                        panic!("Not valid Unicode: {err}");
                    },
                }
                async {}
            })))
            .after_inserted(move |parent| {
                parent.append_child(&view.dom()).unwrap();
            })
        })))
    }

    // this should also be turned into some sort of signal
    pub fn label(&self) -> Dom {
        html!("span", {
            .text_signal(self.file.name.signal_cloned())
        })
    }

    pub fn icon(&self) -> Dom {
        const PATH: &str = "M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z";
        svg!("svg", {
            .attr("height", "1.25em")
            .attr("viewBox", "0 0 24 24")
            .child(svg!("path", {
                .attr("d", PATH)
            }))
        })
    }
}

