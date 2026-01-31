// API 配置
const API_BASE_URL = '/api'; // 使用代理

// 认证相关API
export const AUTH_API = {
  LOGIN: `${API_BASE_URL}/auth/login`,
  REGISTER: `${API_BASE_URL}/auth/register`,
};

// 用户相关API
export const USER_API = {
  GET_USERS: `${API_BASE_URL}/user/query_user`,
  ADD_USER: `${API_BASE_URL}/user/add_user`,
  DELETE_USER: `${API_BASE_URL}/user/delete_user`,
  UPDATE_USER: `${API_BASE_URL}/user/update_user`,
  GET_USER_BY_ID: `${API_BASE_URL}/user/query_user_by_id`,
};