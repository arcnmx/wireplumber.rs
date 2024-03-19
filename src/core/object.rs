use crate::{
	core::{Object, ObjectExt, ObjectFeatures},
	prelude::*,
};

pub trait ObjectExt2: IsA<Object> + 'static {
	type Features: Into<ObjectFeatures> + From<ObjectFeatures>;

	#[doc(alias = "wp_object_activate")]
	fn activate<P, Q, F: Into<Self::Features>>(&self, features: F, cancellable: Option<&P>, callback: Q)
	where
		P: IsA<::gio::Cancellable>,
		Q: FnOnce(Result<(), Error>) + Send + 'static,
	{
		ObjectExt::object_activate(self, features.into().into(), cancellable, callback)
	}

	#[doc(alias = "wp_object_activate_closure")]
	fn activate_closure<P, F: Into<Self::Features>>(&self, features: F, cancellable: Option<&P>, closure: glib::Closure)
	where
		P: IsA<gio::Cancellable>,
	{
		ObjectExt::object_activate_closure(self, features.into().into(), cancellable, closure)
	}

	#[doc(alias = "wp_object_activate")]
	fn activate_future<F: Into<Self::Features>>(
		&self,
		features: F,
	) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
		ObjectExt::object_activate_future(self, features.into().into())
	}

	#[doc(alias = "wp_object_deactivate")]
	fn deactivate<F: Into<Self::Features>>(&self, features: F) {
		ObjectExt::object_deactivate(self, features.into().into())
	}

	#[doc(alias = "wp_object_get_active_features")]
	fn active_features(&self) -> Self::Features {
		ObjectExt::object_active_features(self).into()
	}

	#[doc(alias = "wp_object_get_supported_features")]
	fn supported_features(&self) -> Self::Features {
		ObjectExt::object_supported_features(self).into()
	}

	#[doc(alias = "wp_object_update_features")]
	fn update_features<A: Into<Self::Features>, D: Into<Self::Features>>(&self, activated: A, deactivated: D) {
		ObjectExt::object_update_features(self, activated.into().into(), deactivated.into().into())
	}
}
