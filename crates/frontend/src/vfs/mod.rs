use std::{rc::Rc, sync::Arc};

use dominator::clone;
use either::Either;
use futures::{channel::{mpsc, oneshot}, stream::{FuturesUnordered, StreamExt}, FutureExt};
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use js_sys::{ArrayBuffer, Uint8Array};
use util::StreamTools;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{DomException, FileSystemHandle};

#[derive(Clone)]
pub struct File {
    pub name: Mutable<String>,
    pub mode: Mutable<u32>,
    pub data: Mutable<Vec<u8>>
}

impl File {
    // TODO there are several things that can go wrong here (e.g., 
    // insufficient permissions, timeouts. etc). If any of these fails the
    // whole drop operation should probably be cancelled and a error message
    // somehow displayed to the user
    pub async fn from_handle(file: &web_sys::FileSystemFileHandle) -> File {
        match JsFuture::from(file.get_file()).await {
            Ok(file) => {
                let file = file.unchecked_into::<web_sys::File>();
                let reader = web_sys::FileReader::new().unwrap();
                let (result_tx, mut result_rx) = mpsc::unbounded();
                let onload = Closure::once(clone!(reader, result_tx => move || {
                    result_tx.unbounded_send(Ok(reader.result().unwrap())).unwrap();
                }));
                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                let onerror = Closure::once(clone!(reader, result_tx => move || {
                    result_tx.unbounded_send(Err(reader.error().unwrap())).unwrap();
                }));
                reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                // start reading
                // TODO improve error handling
                tracing::info!("reading file {}", file.name());
                reader.read_as_array_buffer(&file).unwrap();
                // wait for read to complete
                // TODO improve error handling
                let buffer = match result_rx.select_next_some().await {
                    Ok(buffer) => {
                        tracing::info!("reading file {} done", file.name());
                        buffer.unchecked_into::<ArrayBuffer>()
                    }
                    // TODO do not panic here but return a result, which propagates
                    // upwards to fail the whole operation
                    Err(error) => {
                        panic!("could not read file: {}", error.message())
                    }
                };
                File {
                    name: Mutable::new(file.name()),
                    mode: Mutable::new(0o644),
                    data: Mutable::new(Uint8Array::new(&buffer).to_vec()),
                }
            }
            Err(_) => unimplemented!("error handling")
        }
    }
}

#[derive(Clone)]
pub struct Directory {
    pub name: Mutable<String>,
    pub mode: Mutable<u32>,
    pub directories: MutableVec<Rc<Directory>>,
    pub files: MutableVec<Rc<File>>
}

impl Directory {
    pub async fn from_handle(directory: &web_sys::FileSystemDirectoryHandle) -> Directory {
        let entries = directory.values();
        let handles = FuturesUnordered::new();
        while let Ok(entry) = entries.next() {
            let entry = JsFuture::from(entry).await.unwrap();
            if js_sys::Reflect::get(&entry, &"done".into()).unwrap().is_truthy() {
                break;
            }
            else {
                let handle = js_sys::Reflect::get(&entry, &"value".into()).unwrap()
                    .unchecked_into::<FileSystemHandle>();
                handles.push(async { handle });
            }
        }
        let (files, subdirectories): (Vec<Rc<File>>, Vec<Rc<Directory>>) = handles
            .filter_map(|handle| async move {
                match handle.kind() {
                    web_sys::FileSystemHandleKind::File =>
                        File::from_handle(handle.unchecked_ref::<web_sys::FileSystemFileHandle>())
                            .map(Rc::from)
                            .map(Either::Left)
                            .map(Some).await,
                    web_sys::FileSystemHandleKind::Directory =>
                        Directory::from_handle(handle.unchecked_ref::<web_sys::FileSystemDirectoryHandle>())
                            .boxed_local()
                            .map(Rc::from)
                            .map(Either::Right)
                            .map(Some).await,
                    _ => unreachable!(),
                }
            })
            .partition_map(|handle| handle).await;

        Directory {
            name: Mutable::new(directory.name()),
            mode: Mutable::new(0o755),
            directories: MutableVec::new_with_values(subdirectories),
            files: MutableVec::new_with_values(files),
        }
    }
}

