use serde::{Deserialize, Serialize};

/// 应用
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TeeApp {
    pub id: u64,
    /// creator of app
    /// 创建者
    pub creator: String,
    /// name of the app.
    /// 程序名字
    pub name: String,
    /// docker imagge of the App.
    /// docker 镜像
    pub image: String,
    pub status: u8,
}
