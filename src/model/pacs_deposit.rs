use super::super::chain::*;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// 类型
pub type ReportId = Vec<u8>;
pub type PropName = Vec<u8>;
pub type PropValue = Vec<u8>;

// 存证信息
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct Report<AccountId, Moment> {
  // 存证id
  pub id: ReportId,
  // 存证企业
  pub com: AccountId,
  // 操作人员
  pub operator: AccountId,
  // 属性列表
  pub props: Option<Vec<ReportProperty>>,
  // 存证时间
  pub registered: Moment,
}

// 存证属性
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportProperty {
  // 属性名
  pub name: PropName,
  // 属性值
  pub value: PropValue,
}

impl ReportProperty {
  pub fn new(name: PropName, value: PropValue) -> ReportProperty {
    ReportProperty {
      name: name,
      value: value,
    }
  }
}

// 报告注册参数
#[derive(Clone, Debug, Eq, PartialEq, Encode)]
pub struct RegisterReportCall {
  pub id: ReportId,
  pub props: Option<Vec<ReportProperty>>,
}

// 报告属性参数
#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportData {
  // 报告id
  pub id: String,
  // 存证企业
  pub com: Option<String>,
  // 操作人员
  pub operator: Option<String>,
  // 属性
  pub props: Vec<ReportPropertyData>,
}
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportPropertyData {
  // 属性名
  pub name: String,
  // 属性值
  pub value: String,
}

// 成功生成报告
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportRegisteredEvent {
  pub who: AccountId,
  pub com: AccountId,
  pub id: Vec<u8>,
}

// 报告列表
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComOfReportStore<'a> {
  pub report_id: &'a ReportId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportsStore<'a> {
  pub report_id: &'a ReportId,
}

// 报告提交记录
#[derive(Debug, Clone, Decode, Encode, Deserialize, Serialize)]
pub struct ReportDataH {
  pub hash: String,
  pub value: ReportData,
}

// 报告详情
#[derive(Debug, Clone, Decode, Encode, Deserialize, Serialize)]
pub struct ReportDetail {
  // 数据唯一值
  pub key: String,
  // 当前值
  pub curent_value: Option<ReportDataH>,
  // 历史记录
  pub history: Vec<ReportDataH>,
}

// 报告列表
#[derive(Debug, Clone, Decode, Encode, Deserialize, Serialize)]
pub struct ReportList {
  pub list: Vec<ReportDetail>,
}
