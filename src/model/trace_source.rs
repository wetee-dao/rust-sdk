use super::super::chain::*;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// pub type Decimal = I16F16;
pub type Decimal = Vec<u8>;
pub type ProductId = Vec<u8>;
pub type PropName = Vec<u8>;
pub type PropValue = Vec<u8>;
pub type ShipmentId = Vec<u8>;
pub type ShippingEventIndex = u128;
pub type DeviceId = Vec<u8>;

#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct Product<AccountId> {
  id: ProductId,
  owner: AccountId,
  props: Option<Vec<ProductProperty>>,
  registered: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ProductProperty {
  pub name: PropName,
  pub value: PropValue,
}

impl ProductProperty {
  pub fn new(name: PropName, value: PropValue) -> Self {
    Self {
      name: name,
      value: value,
    }
  }
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct ReadPoint {
  pub latitude: Decimal,
  pub longitude: Decimal,
}

impl ReadPoint {
  pub fn new(latitude: Decimal, longitude: Decimal) -> Self {
    Self {
      latitude: latitude,
      longitude: longitude,
    }
  }
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub enum ShippingOperation {
  Pickup,
  Scan,
  Deliver,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub enum ReadingType {
  Humidity,
  Pressure,
  Shock,
  Tilt,
  Temperature,
  Vibration,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct Reading<Moment> {
  pub device_id: DeviceId,
  pub reading_type: ReadingType,
  #[codec(compact)]
  pub timestamp: Moment,
  pub sensor_value: Decimal,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReadingData<Moment> {
  pub device_id: String,
  pub reading_type: String,
  #[codec(compact)]
  pub timestamp: Moment,
  pub sensor_value: String,
}

// 产品参数
#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ProductData {
  // 产品id
  pub id: String,
  // 属性
  pub props: Vec<ProductPropertyData>,
}

#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ProductPropertyData {
  // 属性名
  pub name: String,
  // 属性值
  pub value: String,
}

// 批次参数
#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ShipmentData {
  // 批次id
  pub id: String,
  // 属性
  pub products: Vec<String>,
}

// 追踪批次参数
#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct TrackData {
  // 批次id
  pub id: String,
  // 操作类型
  pub shipping_operation: String,
  // pub timestamp: u64,
  pub location: Option<ReadPointData>,
  pub readings: Option<Vec<ReadingData<Moment>>>,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReadPointData {
  pub latitude: String,
  pub longitude: String,
}

// 产品注册参数
#[derive(Clone, Debug, Eq, PartialEq, Encode)]
pub struct RegisterProductCall {
  pub id: ProductId,
  pub props: Option<Vec<ProductProperty>>,
}

// 批次注册参数
#[derive(Clone, Debug, Eq, PartialEq, Encode)]
pub struct RegisterShipmentCall {
  pub id: ShipmentId,
  pub products: Vec<ProductId>,
}

// 追踪批次参数
#[derive(Clone, Debug, Eq, PartialEq, Encode)]
pub struct TrackShipmentCall {
  pub id: ShipmentId,
  pub operation: ShippingOperation,
  #[codec(compact)]
  pub timestamp: Moment,
  pub location: Option<ReadPoint>,
  pub readings: Option<Vec<Reading<Moment>>>,
}
