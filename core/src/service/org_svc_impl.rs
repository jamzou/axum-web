use crate::{OrgDao, entity::prelude::Org};
use chrono::Utc;
use grpc_dsl::common::{Empty, IdRequest, IdResponse, RowsAffected};
use grpc_dsl::org::{AddOrgRequest, OrgData, OrgListResponse, OrgResponse, org_service_server::OrgService};
use tonic::{Request, Response, Status};

pub struct OrgServiceImpl<D: OrgDao + Send + Sync + 'static> {
    dao: D,
}

impl<D: OrgDao + Send + Sync + 'static> OrgServiceImpl<D> {
    pub fn new(dao: D) -> Self {
        Self { dao }
    }
}

impl From<AddOrgRequest> for Org {
    fn from(req: AddOrgRequest) -> Self {
        Org {
            id: req.id,
            code: req.org_code,
            name: req.org_name,
            parent_id: 0, // 默认父部门ID
            deleted: false, // 默认未删除
            deleted_at: None,
            created_by: None,
            updated_by: None,
            created_at: Some(Utc::now().naive_local()),
            updated_at: Some(Utc::now().naive_local()),
        }
    }
}

impl From<Org> for OrgData {
    fn from(org: Org) -> Self {
        OrgData {
            id: org.id,
            org_code: org.code,
            org_name: org.name,
            update_time: org.updated_at
            .and_then(|t| Some(t.and_utc().timestamp_millis())).unwrap_or(0i64),
        }
    }

}

#[tonic::async_trait]
impl<D> OrgService for OrgServiceImpl<D>
where
    D: OrgDao + Send + Sync + 'static,
{
    async fn add_org(
        &self,
        request: Request<AddOrgRequest>,
    ) -> Result<Response<IdResponse>, Status> {
        let req = request.into_inner();
        let create: Org = req.into();

        let id = self.dao.add_org(&create).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(IdResponse { id }))
    }

    async fn update_org(
        &self,
        request: Request<AddOrgRequest>,
    ) -> Result<Response<RowsAffected>, Status> {
        let req = request.into_inner();
        let update: Org = req.into();

        let count = self.dao.update_org(&update).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(RowsAffected { count }))
    }

    async fn delete_org(
        &self,
        request: Request<IdRequest>,
    ) -> Result<Response<RowsAffected>, Status> {
        let req = request.into_inner();

        let count = self.dao.delete_org(req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(RowsAffected { count }))
    }

    async fn get_org_by_id(
        &self,
        request: Request<IdRequest>,
    ) -> Result<Response<OrgResponse>, Status> {
        let req = request.into_inner();
        let org = self.dao.query_org_by_id(req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        let org_response = match org {
            Some(o) => {OrgResponse { found: true, org: Some(o.into()) }},
            None => {OrgResponse { found: false, org: None }},
        };
        Ok(Response::new(org_response))
    }

    async fn get_all_orgs(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<OrgListResponse>, Status> {
        let orgs = self.dao.get_all_orgs().await.map_err(|e| Status::internal(e.to_string()))?;
        let org_datas = orgs.into_iter().map(|o| OrgData::from(o)).collect();
        Ok(Response::new(OrgListResponse { orgs:org_datas }))
    }
}
