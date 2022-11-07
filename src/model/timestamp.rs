use subxt::{
  system::{System}, Store,
  DefaultNodeRuntime,
};
use sp_runtime::{
	traits::{
		AtLeast32Bit, Scale, Member
	}
};
use frame_support::{
  Parameter,
};
use codec::{Encode};
use core::marker::PhantomData;
use std::fmt::Debug;

// 模块定义
#[subxt::module]
pub trait Timestamp: System {
	type Moment: Parameter + Default + AtLeast32Bit + Member
		+ Scale<Self::BlockNumber, Output = Self::Moment> + Copy;
}

pub type Moment = u64;
impl Timestamp for DefaultNodeRuntime {
  type Moment = Moment;
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct NowStore<T: Timestamp> {
    #[store(returns = T::Moment)]
    pub _runtime: PhantomData<T>,
}

/// Impls `Default::default` for some types that have a `_runtime` field of type
/// `PhantomData` as their only field.
macro_rules! default_impl {
    ($name:ident) => {
        impl<T: Timestamp> Default for $name<T> {
            fn default() -> Self {
                Self {
                    _runtime: PhantomData,
                }
            }
        }
    };
}

default_impl!(NowStore);