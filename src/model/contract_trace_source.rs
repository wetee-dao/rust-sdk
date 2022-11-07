use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct CTST {
  pub id: String,
  pub hash: String,
  pub mark: Option<i32>,
  pub first_mark_addr: Option<String>,
  pub first_mark_time: Option<i64>,
  pub sku_id: String,
  pub brand_id: String,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Clone)]
pub struct CTSTRecord {
  pub id: i64,
  pub token_id: String,
  pub hash: String,
  pub meta: String,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TMeta {
  pub k: String,
  pub v: String,
}
