use crate::model::dao::WithGov;
use crate::{account, Client};

use super::base_hander::BaseHander;
use super::wetee_gov::run_sudo_or_gov;
use sp_core::{crypto::Ss58Codec, sr25519::Public};
pub use wetee_project::{ProjectInfo, TaskInfo, TaskStatus};
use wetee_runtime::{AccountId, Balance, Runtime, RuntimeCall, Signature, WeteeProjectCall};

use substrate_api_client::{ExtrinsicSigner, GetStorage, SubmitAndWatchUntilSuccess};

/// 账户
pub struct WeteeProject {
    pub base: BaseHander,
}

impl WeteeProject {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    // 项目列表
    pub fn project_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<ProjectInfo<AccountId>>, anyhow::Error> {
        let api = self.base.get_client()?;

        // 构建请求
        let result: Vec<ProjectInfo<AccountId>> = api
            .get_storage_map("WeteeProject", "DaoProjects", dao_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    // 创建项目
    pub fn create_project(
        &mut self,
        from: String,
        dao_id: u64,
        name: String,
        desc: String,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::create_project {
            name: name.into(),
            description: desc.into(),
            dao_id,
            creator: AccountId::from(Public::from_string(&from).unwrap()),
        });

        if ext.is_some() {
            return run_sudo_or_gov(api, dao_id, call, ext.unwrap());
        }

        let signer_nonce = api.get_nonce().unwrap();
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    // 成员列表
    pub fn member_list(
        &mut self,
        dao_id: u64,
        project_id: u64,
    ) -> anyhow::Result<Vec<AccountId>, anyhow::Error> {
        let api = self.base.get_client()?;

        // 构建请求
        let result: Vec<AccountId> = api
            .get_storage_double_map("WeteeDAO", "ProjectMembers", dao_id, project_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    // 任务列表
    pub fn task_list(
        &mut self,
        project_id: u64,
    ) -> anyhow::Result<Vec<TaskInfo<AccountId, Balance>>, anyhow::Error> {
        let api = self.base.get_client()?;

        // 构建请求
        let result: Vec<TaskInfo<AccountId, Balance>> = api
            .get_storage_map("WeteeProject", "Tasks", project_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    pub fn task_info(
        &mut self,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<TaskInfo<AccountId, Balance>, anyhow::Error> {
        let api = self.base.get_client()?;

        // 构建请求
        let result: Vec<TaskInfo<AccountId, Balance>> = api
            .get_storage_map("WeteeProject", "Tasks", project_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);
        let task = result
            .into_iter()
            .find(|x| x.id == task_id)
            .ok_or_else(|| anyhow::anyhow!("task not found"))?;
        Ok(task)
    }

    // 创建任务
    pub fn create_task(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        name: String,
        desc: String,
        priority: u8,
        point: u16,
        assignees: Option<Vec<String>>,
        skills: Option<Vec<u8>>,
        max_assignee: Option<u8>,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::create_task {
            name: name.into(),
            description: desc.into(),
            dao_id,
            project_id,
            point,
            priority,
            max_assignee,
            skills,
            assignees: if assignees.is_some() {
                Some(
                    assignees
                        .unwrap()
                        .into_iter()
                        .map(|x| AccountId::from(Public::from_string(&x).unwrap()))
                        .collect(),
                )
            } else {
                None
            },
            amount,
        });

        let signer_nonce = api.get_nonce().unwrap();
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    pub fn start_task(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::start_task {
            dao_id,
            project_id,
            task_id,
        });

        let signer_nonce = api.get_nonce().unwrap();
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    pub fn request_review(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::request_review {
            dao_id,
            project_id,
            task_id,
        });

        let signer_nonce = api.get_nonce().unwrap();
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    // 完成任务
    pub fn task_done(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::task_done {
            dao_id,
            project_id,
            task_id,
        });

        let signer_nonce = api.get_nonce().unwrap();
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }
}
