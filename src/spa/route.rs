use crate::{
	prelude::*,
	pw::PipewireObject,
	spa::{SpaPod, SpaProps},
};

#[cfg_attr(feature = "dox", doc(cfg(feature = "experimental")))]
#[derive(Debug, Clone)]
pub struct SpaRoute {
	params: SpaPod,
}

impl SpaRoute {
	pub fn with_params(params: SpaPod) -> crate::Result<Self> {
		// TODO: assert id/type, also check for existence of appropriate fields
		Ok(Self { params })
	}

	pub fn into_params(self) -> SpaPod {
		self.params
	}

	pub fn props(&self) -> Option<SpaProps> {
		self
			.params
			.find_spa_property(&libspa_sys::spa_param_route_SPA_PARAM_ROUTE_props)
			.map(|props| SpaProps::with_params(props).unwrap()) // TODO: assert this too in constructor?
	}

	pub fn info(&self) -> crate::Result<impl Iterator<Item = (String, String)>> {
		let res = match self
			.params
			.find_spa_property(&libspa_sys::spa_param_route_SPA_PARAM_ROUTE_info)
		{
			Some(params) => params.struct_fields(true),
			None => Ok(Vec::new().into_iter()),
		}?;

		res
			.map(|(key, value)| {
				(&value).try_into().map(|v| (key, v)).map_err(|e| {
					Error::new(
						LibraryErrorEnum::InvalidArgument,
						&format!("expected a string route info value, got {:?} instead: {:?}", value, e),
					)
				})
			})
			.collect::<Result<Vec<_>, _>>()
			.map(|v| v.into_iter())
	}

	pub fn profile_indices(&self) -> impl Iterator<Item = u32> {
		match self
			.params
			.find_spa_property(&libspa_sys::spa_param_route_SPA_PARAM_ROUTE_profiles)
		{
			Some(params) => params.array_iterator::<i32>().map(|i| i.try_into().unwrap()).collect(),
			None => Vec::new(),
		}
		.into_iter()
	}

	pub fn device_indices(&self) -> impl Iterator<Item = u32> {
		match self
			.params
			.find_spa_property(&libspa_sys::spa_param_route_SPA_PARAM_ROUTE_devices)
		{
			Some(params) => params.array_iterator::<i32>().map(|i| i.try_into().unwrap()).collect(),
			None => Vec::new(),
		}
		.into_iter()
	}

	pub fn index(&self) -> u32 {
		self
			.params
			.spa_property(&libspa_sys::spa_param_route_SPA_PARAM_ROUTE_index)
			.unwrap()
	}

	pub fn device_index(&self) -> u32 {
		self
			.params
			.spa_property(&libspa_sys::spa_param_route_SPA_PARAM_ROUTE_index)
			.unwrap()
	}

	pub fn has_volume(&self) -> bool {
		self.props().map(|props| props.has_volume()).unwrap_or_default()
	}

	pub fn contains_device(&self, device_index: u32) -> bool {
		self.device_indices().any(|i| i == device_index)
	}
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "experimental")))]
#[derive(Debug)]
pub struct SpaRoutes {
	routes: Vec<SpaRoute>,
}

impl SpaRoutes {
	pub fn with_params<I: IntoIterator<Item = SpaRoute>>(routes: I) -> Self {
		Self {
			routes: routes.into_iter().collect(),
		}
	}

	pub fn into_routes(self) -> Vec<SpaRoute> {
		self.routes
	}

	pub fn into_params(self) -> impl Iterator<Item = SpaPod> {
		self.routes.into_iter().map(|r| r.into_params())
	}

	pub async fn from_object<O: IsA<PipewireObject>>(obj: &O) -> crate::Result<Self> {
		let params = obj.params_future(Some("Route"), None).await?;
		params.map(|route| SpaRoute::with_params(route)).collect()
	}

	pub fn has_volume(&self, device_index: u32) -> bool {
		self
			.by_device_index(device_index)
			.map(|dev| dev.has_volume())
			.unwrap_or_default()
	}

	pub fn by_index(&self, index: u32) -> Option<&SpaRoute> {
		self.routes.iter().find(|r| r.index() == index)
	}

	pub fn by_device_index(&self, device_index: u32) -> Option<&SpaRoute> {
		self.routes.iter().find(|r| r.contains_device(device_index))
	}
}

impl FromIterator<SpaRoute> for SpaRoutes {
	fn from_iter<I: IntoIterator<Item = SpaRoute>>(i: I) -> Self {
		Self::with_params(i)
	}
}
