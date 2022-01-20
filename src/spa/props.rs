use glib::prelude::*;
use crate::{SpaPod, PipewireObject};
use crate::prelude::*;

#[cfg_attr(feature = "dox", doc(cfg(feature = "experimental")))]
#[derive(Debug)]
pub struct SpaProps {
	params: SpaPod,
}

impl SpaProps {
	pub fn with_params(params: SpaPod) -> crate::Result<Self> {
		// TODO: assert id/type
		Ok(Self {
			params,
		})
	}

	pub fn into_params(self) -> SpaPod {
		self.params
	}

	pub fn params(&self) -> crate::Result<impl Iterator<Item=(String, SpaPod)>> {
		match self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_params) {
			Some(params) => params.struct_fields(false),
			None => Ok(Vec::new().into_iter()),
		}
	}

	pub fn has_volume(&self) -> bool {
		self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_channelVolumes).is_some() ||
			self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_volume).is_some()
	}

	pub fn mute(&self) -> bool {
		self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_mute)
			.and_then(|pod| pod.boolean())
			.unwrap_or_default()
	}

	pub fn channel_volume(&self, channel_index: u32) -> f32 {
		self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_channelVolumes)
			.and_then(|pod| pod.array_iterator::<f32>().nth(channel_index as usize))
			.or_else(|| self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_volume)
				.and_then(|pod| pod.float())
			)
			.unwrap_or(1.0f32)
	}

	pub fn set_channel_volume(&self, channel_index: u32, volume: f32) -> Result<(), ()> {
		let succ = match self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_channelVolumes) {
			Some(pod) => {
				let mut data: Vec<f32> = pod.array_iterator().collect();
				*data.get_mut(channel_index as usize).ok_or(())? = volume;
				pod.set_pod(&data.into_iter().collect())
			},
			None => {
				let pod = self.params.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_volume).ok_or(())?;
				pod.set_float(volume)
			}
		};
		if succ {
			Ok(())
		} else {
			Err(())
		}
	}

	pub async fn from_object<O: IsA<PipewireObject>>(obj: &O) -> crate::Result<Self> {
		let params = obj.params_future(Some("Props"), None).await?;
		for item in params {
			if item.find_spa_property(&libspa_sys::spa_prop_SPA_PROP_volume).is_some() {
				return Self::with_params(item)
			}
		}
		Err(todo!())
	}
}
