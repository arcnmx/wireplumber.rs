#[cfg(not(feature = "glib-signal"))]
use std::pin::Pin;
#[cfg(not(feature = "glib-signal"))]
use std::{
	cell::RefCell,
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc,
	},
};
use {crate::prelude::*, std::future::Future};

// XXX: use `impl Future` here if Rust's broken assumptions are ever fixed: https://github.com/rust-lang/rust/issues/42940
// (better yet: return a wrapper type that disconnects on drop like glib-signal does)
#[cfg(not(feature = "glib-signal"))]
pub(crate) fn signal_once<T: ObjectType, F: FnOnce(Box<dyn Fn(&T) + 'static>) -> glib::SignalHandlerId>(
	connect: F,
) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
	let (tx, rx) = futures_channel::oneshot::channel();

	let tx = RefCell::new(Some(tx));

	let signal_id = Arc::new(AtomicU64::default());
	let signal = connect({
		let signal_id = signal_id.clone();
		Box::new(move |core| {
			if let Some(tx) = tx.borrow_mut().take() {
				if let Err(e) = tx.send(()) {
					wp_debug!("oneshot future ignored: {:?}", e);
				}
			}
			match signal_id.fetch_and(0, Ordering::SeqCst) {
				0 => (),
				signal => glib::signal::signal_handler_disconnect(core, unsafe { from_glib(signal) }),
			}
		})
	});
	signal_id.store(unsafe { signal.as_raw() as u64 }, Ordering::SeqCst);

	Box::pin(async move { rx.await.map_err(crate::error::invariant) })
}

#[cfg(feature = "glib-signal")]
pub(crate) fn signal_once<T: ObjectType, R: 'static>(
	signal: glib_signal::SignalStream<T, R>,
) -> impl Future<Output = Result<R, Error>> {
	async move { signal.once().await.map_err(Into::into).map(|(res, _)| res) }
}
