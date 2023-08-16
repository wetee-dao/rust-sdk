
use crate::client::Client;

/// TEE程序
pub struct Wetee {
    pub base: Client,
}

impl Wetee {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    // pub async fn get_wait_pool(&mut self) -> Result<Vec<MTeeApp>, Box<dyn std::error::Error>> {
    //     // 获取区块链接口
    //     let api = self.base.get_client().await?;

    //     let address = wetee_chain::storage().wetee().wait_pool();
    //     let iter = api.storage().fetch(&address, None).await?;

    //     match iter {
    //         Some(v) => {
    //             let mut list: Vec<MTeeApp> = vec![];
    //             for (_index, v) in v.0.to_vec().into_iter().enumerate() {
    //                 let app = self.get_app(v).await?;
    //                 list.push(app);
    //             }

    //             return Ok(list);
    //         }
    //         None => {
    //             return Ok(vec![]);
    //         }
    //     };
    // }

    // pub async fn get_app(& self, app_id: u64) -> Result<MTeeApp, Box<dyn std::error::Error>> {
    //     // 获取区块链接口
    //     let api = self.base.get_client().await?;

    //     let address = wetee_chain::storage().wetee().tee_apps(app_id);
    //     let iter = api.storage().fetch(&address, None).await?;

    //     match iter {
    //         Some(v) => {
    //             return Ok(MTeeApp {
    //                 id: app_id,
    //                 name: String::from_utf8(v.name.clone()).unwrap(),
    //                 image: String::from_utf8(v.image.clone()).unwrap(),
    //                 status: v.status.clone(),
    //                 creator: v.creator.to_string(),
    //             });
    //         }
    //         None => {
    //             return Err(Error::msg("app 不存在").into());
    //         }
    //     };
    // }

    // pub async fn run_app(& self, from: String, app_id: u64) -> Result<u64, anyhow::Error> {
    //     // 获取区块链接口
    //     // let api = self.base.get_client().await?;

    //     // let tx = wetee_chain::tx().wetee().run_app(app_id);

    //     // let from_pair = account::get_from_address(from.clone()).unwrap();

    //     // let signer = pair_signer(from_pair);
    //     // let trans = api
    //     //     .tx()
    //     //     .sign_and_submit_then_watch_default(&tx, &signer)
    //     //     .await?
    //     //     .wait_for_finalized()
    //     //     .await?;

    //     // let _events = trans.wait_for_success().await?;
    //     // let events = trans.fetch_events().await?;

    //     // let failed_event = events.find_first::<wetee_chain::system::events::ExtrinsicFailed>()?;
    //     // if let Some(_ev) = failed_event {
    //     //     return Err(Error::msg("交易错误"));
    //     // } else {
    //     //     let transfer_event = events.find_first::<wetee_chain::wetee::events::AppRuning>()?;
    //     //     if let Some(event) = transfer_event {
    //     //         println!("Balance transfer success: {event:?}");
    //     //         return Ok(app_id);
    //     //     } else {
    //     //         return Err(Error::msg("无法找到交易信息"));
    //     //     }
    //     // }
    //     Ok(0)
    // }
}
