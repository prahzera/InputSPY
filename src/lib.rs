use napi::bindgen_prelude::*;
use napi_derive::napi;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use rdev::{listen, Event, EventType};
use std::thread;

#[napi]
pub fn start_inputspy(callback: Function<'static>) -> Result<()> {
    let tsfn = callback
        .build_threadsafe_function()
        .build_callback(|ctx| Ok(vec![ctx.value]))?;

    thread::spawn(move || {
        let _ = listen(move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let key_str = format!("{:?}", key);
                let _ = tsfn.call(key_str, ThreadsafeFunctionCallMode::NonBlocking);
            }
        });
    });

    Ok(())
}