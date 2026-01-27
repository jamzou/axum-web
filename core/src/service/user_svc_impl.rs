use crate::entity::prelude::{CreateUser as DbCreateUser, User as DbUser};
use crate::entity::mo_app_user::{LoginUser as DbLoginUser, UserInfo as DbUserInfo};
use crate::utils::password_util;
use crate::utils::jwt_util;
use crate::UserDao;
use grpc_dsl::common::{Empty, IdRequest, IdResponse, RowsAffected};
use grpc_dsl::org::OrgData;
use grpc_dsl::user::user_service_server::UserService;
use grpc_dsl::user::{AddUserRequest, LoginUserRequest, RegisterUserRequest, LoginResponse, RegisterResponse, UserData, UserListResponse, UserResponse};
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
            email: u.email.clone().unwrap_or_default(),
            phone: u.phone.clone().unwrap_or_default(),
            org_id: u.org_id.unwrap_or_default(),
            role: u.role.clone().unwrap_or_default(),
            status: u.status.unwrap_or_default() as i32,
            last_login_time: match u.last_login_time {
                Some(t) => t.and_utc().timestamp_millis(),
                None => 0i64,
            },
            create_time: match u.created_at {
                Some(t) => t.and_utc().timestamp_millis(),
                None => 0i64,
            },
            update_time: u
                .updated_at
                .and_then(|t| Some(t.and_utc().timestamp_millis()))
                .unwrap_or(0i64),
            org_data: None,
        }
    }
}

impl From<AddUserRequest> for DbCreateUser {
    fn from(req: AddUserRequest) -> Self {
        DbCreateUser {
            id: if req.id == 0 { None } else { Some(req.id) },
            emp_id: req.emp_id,
            user_name: req.user_name,
            password: req.password,
            email: Some(req.email),
            phone: Some(req.phone),
            org_id: if req.org_id == 0 { None } else { Some(req.org_id) },
            role: Some(req.role),
            status: Some(req.status as i8),
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
        let result = self
            .dao
            .get_user_detail(id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        match result {
            Some((u, o)) => {
                let mut user_data = UserData::from(u);
                user_data.org_data = Some(OrgData::from(o));
                Ok(Response::new(UserResponse {
                    found: true,
                    user: Some(user_data),
                }))
            }
            None => {
                // 如果没找到带 org 的详情，尝试只查 user
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

    async fn login(
        &self,
        request: Request<LoginUserRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let user = self.dao.find_user_by_username(&req.user_name)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        match user {
            Some(user) => {
                // 验证密码
                let is_valid = password_util::verify_password(&req.password, &user.password)
                    .map_err(|e| Status::internal(format!("Password verification error: {}", e)))?;
                
                if is_valid {
                    // 更新最后登录时间
                    self.dao.update_last_login_time(user.id)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                    
                    // 生成JWT token
                    let access_token = jwt_util::generate_access_token(
                        user.id,
                        user.user_name.clone(),
                        user.role.clone().unwrap_or("user".to_string())
                    ).map_err(|e| Status::internal(e.to_string()))?;
                    
                    let login_response = LoginResponse {
                        success: true,
                        message: "Login successful".to_string(),
                        access_token,
                        refresh_token: "".to_string(), // 暂时返回空，可以后续实现刷新令牌
                        user_id: user.id,
                        user_name: user.user_name,
                        role: user.role.unwrap_or("user".to_string()),
                    };
                    
                    Ok(Response::new(login_response))
                } else {
                    Ok(Response::new(LoginResponse {
                        success: false,
                        message: "Invalid username or password".to_string(),
                        access_token: "".to_string(),
                        refresh_token: "".to_string(),
                        user_id: 0,
                        user_name: "".to_string(),
                        role: "".to_string(),
                    }))
                }
            },
            None => Ok(Response::new(LoginResponse {
                success: false,
                message: "User not found".to_string(),
                access_token: "".to_string(),
                refresh_token: "".to_string(),
                user_id: 0,
                user_name: "".to_string(),
                role: "".to_string(),
            })),
        }
    }

    async fn register(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        
        // 检查用户名是否已存在
        let existing_user = self.dao.find_user_by_username(&req.user_name)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        if existing_user.is_some() {
            return Ok(Response::new(RegisterResponse {
                success: false,
                message: "Username already exists".to_string(),
                user_id: 0,
            }));
        }
        
        // 对密码进行哈希处理
        let hashed_password = password_util::hash_password(&req.password)
            .map_err(|e| Status::internal(e.to_string()))?;
        
        // 创建用户
        let new_user = DbCreateUser {
            id: None,
            emp_id: req.emp_id,
            user_name: req.user_name,
            password: hashed_password,
            email: Some(req.email),
            phone: Some(req.phone),
            org_id: if req.org_id == 0 { None } else { Some(req.org_id) },
            role: Some(req.role),
            status: Some(1), // 默认激活状态
        };
        
        let user_id = self.dao.add_user(&new_user)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        Ok(Response::new(RegisterResponse {
            success: true,
            message: "Registration successful".to_string(),
            user_id,
        }))
    }
}
