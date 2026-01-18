use crate::entity::prelude::{CreateUser as DbCreateUser, User as DbUser};
use crate::UserDao;
use grpc_dsl::user::user_service_server::UserService;
use grpc_dsl::user::{
    AddUserRequest, Empty, IdRequest, IdResponse, RowsAffected, UserData, UserListResponse,
    UserResponse,
};
use tonic::{Request, Response, Status};

pub struct UserServiceImpl<D: UserDao + Send + Sync + 'static> {
    dao: D,
}

impl<D: UserDao + Send + Sync + 'static> UserServiceImpl<D> {
    pub fn new(dao: D) -> Self {
        Self { dao }
    }
}

impl From<DbUser> for UserData {
    fn from(u: DbUser) -> Self {
        UserData {
            id: u.id,
            emp_id: u.emp_id,
            user_name: u.user_name,
            age: u.age.unwrap_or_default() as u32,
            birthday: u.birthday.unwrap_or_default(),
            create_time: match u.create_time {
                Some(t) => t.and_utc().timestamp_millis(),
                None => 0i64,
            },
            update_time: u
                .update_time
                .and_then(|t| Some(t.and_utc().timestamp_millis()))
                .unwrap_or(0i64),
        }
    }
}

impl From<AddUserRequest> for DbCreateUser {
    fn from(req: AddUserRequest) -> Self {
        DbCreateUser {
            id: if req.id == 0 { None } else { Some(req.id) },
            emp_id: req.emp_id,
            user_name: req.user_name,
            age: req.age as u8,
            birthday: req.birthday,
        }
    }
}

#[tonic::async_trait]
impl<D> UserService for UserServiceImpl<D>
where
    D: UserDao + Send + Sync + 'static,
{
    async fn add_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<IdResponse>, Status> {
        let req = request.into_inner();
        let create: DbCreateUser = req.into();
        let id = self
            .dao
            .add_user(&create)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(IdResponse { id }))
    }

    async fn get_all_users(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<UserListResponse>, Status> {
        let users = self
            .dao
            .get_all_users()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let users = users.into_iter().map(UserData::from).collect();
        Ok(Response::new(UserListResponse { users }))
    }

    async fn get_user_by_id(
        &self,
        request: Request<IdRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let id = request.into_inner().id;
        let user = self
            .dao
            .query_user_by_id(id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        match user {
            Some(u) => Ok(Response::new(UserResponse {
                found: true,
                user: Some(UserData::from(u)),
            })),
            None => Ok(Response::new(UserResponse {
                found: false,
                user: None,
            })),
        }
    }

    async fn delete_user(
        &self,
        request: Request<IdRequest>,
    ) -> Result<Response<RowsAffected>, Status> {
        let id = request.into_inner().id;
        let count = self
            .dao
            .delete_user(id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(RowsAffected { count }))
    }

    async fn update_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<RowsAffected>, Status> {
        let req = request.into_inner();
        let create: DbCreateUser = req.into();
        let count = self
            .dao
            .update_user(&create)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(RowsAffected { count }))
    }
}
