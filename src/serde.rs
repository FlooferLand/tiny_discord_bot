pub mod arc_rwlock_serde {
	use crate::ArcLock;
	use serde::de::Deserializer;
	use serde::ser::Serializer;
	use serde::{Deserialize, Serialize};
	use std::sync::Arc;
	use tokio::sync::RwLock;

	pub fn serialize<S, T>(val: &ArcLock<T>, s: S) -> Result<S::Ok, S::Error>
	where S: Serializer,
	      T: Serialize,
	{
		tokio::runtime::Builder::new_current_thread()
			.enable_all()
			.build()
			.unwrap()
			.block_on(async { T::serialize(&*val.read().await, s) })
	}

	pub fn deserialize<'de, D, T>(d: D) -> Result<ArcLock<T>, D::Error>
	where D: Deserializer<'de>,
	      T: Deserialize<'de>,
	{
		Ok(Arc::new(RwLock::new(T::deserialize(d)?)))
	}
}
