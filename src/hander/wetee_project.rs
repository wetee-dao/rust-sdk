use crate::chain::API_CLIENT_POOL;
use crate::model::chain::QueryKey;
use crate::model::dao::WithGov;
use crate::{account, Client};


use super::wetee_gov::run_sudo_or_gov;
use sp_core::sr25519;
use sp_core::{crypto::Ss58Codec, sr25519::Public};
use sp_runtime::AccountId32;
use wetee_gov::MemmberData;
use wetee_project::ReviewOpinion;
pub use wetee_project::{ProjectInfo, TaskInfo, TaskStatus};
use wetee_runtime::{AccountId, Balance, Runtime, RuntimeCall, Signature, WeteeProjectCall};

use substrate_api_client::{ExtrinsicSigner, GetStorage, SubmitAndWatchUntilSuccess};

/// 账户
pub struct WeteeProject {
    pub base: Client,
}

impl WeteeProject {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    // 项目列表
    pub async fn project_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<ProjectInfo<AccountId>>, anyhow::Error> {
        // 构建请求
        let result: Vec<ProjectInfo<AccountId>> = self.base.get_storage_map("WeteeProject", "DaoProjects", QueryKey::IntKey(dao_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    // 创建项目
    pub async fn create_project(
        &mut self,
        from: String,
        dao_id: u64,
        name: String,
        desc: String,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::create_project {
            name: name.into(),
            description: desc.into(),
            dao_id,
            creator: AccountId::from(Public::from_string(&from).unwrap()),
        });

        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }

        self.base.send_and_sign(call,from).await
    }

    pub async fn project_join_request_with_root(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        user: String,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&user).unwrap().into();
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::project_join_request {
            dao_id,
            project_id,
            who,
        });

        return run_sudo_or_gov(
            &self.base,
            from,
            dao_id,
            call,
            WithGov {
                run_type: 2,
                amount: 0,
                member: MemmberData::GLOBAL,
            },
        ).await;
    }

    pub async fn project_join_request(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&from).unwrap().into();
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::project_join_request {
            dao_id,
            project_id,
            who,
        });

        self.base.send_and_sign(call,from).await
    }

    // 成员列表
    pub async fn member_list(
        &mut self,
        dao_id: u64,
        project_id: u64,
    ) -> anyhow::Result<Vec<AccountId>, anyhow::Error> {
        // 构建请求
        let result: Vec<AccountId> = self.base
            .get_storage_double_map("WeteeDAO", "ProjectMembers", QueryKey::IntKey(dao_id), QueryKey::IntKey(project_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    // 任务列表
    pub async fn task_list(
        &mut self,
        project_id: u64,
    ) -> anyhow::Result<Vec<TaskInfo<AccountId, Balance>>, anyhow::Error> {
        // 构建请求
        let result: Vec<TaskInfo<AccountId, Balance>> = self.base.get_storage_map("WeteeProject", "Tasks", QueryKey::IntKey(project_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    pub async fn task_info(
        &mut self,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<TaskInfo<AccountId, Balance>, anyhow::Error> {
        // 构建请求
        let result: Vec<TaskInfo<AccountId, Balance>> = self.base.get_storage_map("WeteeProject", "Tasks", QueryKey::IntKey(project_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);
        let task = result
            .into_iter()
            .find(|x| x.id == task_id)
            .ok_or_else(|| anyhow::anyhow!("task not found"))?;
        Ok(task)
    }

    // 创建任务
    pub async fn create_task(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        name: String,
        desc: String,
        priority: u8,
        point: u16,
        assignees: Option<Vec<String>>,
        reviewers: Option<Vec<String>>,
        skills: Option<Vec<u8>>,
        max_assignee: Option<u8>,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
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
            reviewers: if reviewers.is_some() {
                Some(
                    reviewers
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

        
        self.base.send_and_sign(call,from).await
    }

    pub async fn start_task(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::start_task {
            dao_id,
            project_id,
            task_id,
        });

        self.base.send_and_sign(call,from).await
    }

    pub async fn request_review(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::request_review {
            dao_id,
            project_id,
            task_id,
        });


        self.base.send_and_sign(call,from).await
    }

    // 完成任务
    pub async fn task_done(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::task_done {
            dao_id,
            project_id,
            task_id,
        });

        self.base.send_and_sign(call,from).await
    }

    // 加入任务
    pub async fn join_task(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::join_task {
            dao_id,
            project_id,
            task_id,
        });

        self.base.send_and_sign(call,from).await
    }

    // 离开任务
    pub async fn leave_task(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::leave_task {
            dao_id,
            project_id,
            task_id,
        });

        self.base.send_and_sign(call,from).await
    }

    // 作为任务评审
    pub async fn join_task_review(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::join_task_review {
            dao_id,
            project_id,
            task_id,
        });

        self.base.send_and_sign(call,from).await
    }

    // 离开任务评审
    pub async fn leave_task_review(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::leave_task_review {
            dao_id,
            project_id,
            task_id,
        });

        self.base.send_and_sign(call,from).await
    }

    pub async fn make_review(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        task_id: u64,
        approve: bool,
        meta: String,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::make_review {
            dao_id,
            project_id,
            task_id,
            opinion: if approve {
                ReviewOpinion::YES
            } else {
                ReviewOpinion::NO
            },
            meta: meta.into(),
        });

        self.base.send_and_sign(call,from).await
    }

    pub async fn apply_project_funds(
        &mut self,
        from: String,
        dao_id: u64,
        project_id: u64,
        amount: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeProject(WeteeProjectCall::apply_project_funds {
            dao_id,
            project_id,
            amount: amount.into(),
        });

        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }

        self.base.send_and_sign(call,from).await
    }
}
