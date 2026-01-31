import axios, { type AxiosInstance, type InternalAxiosRequestConfig, type AxiosResponse, type AxiosRequestHeaders } from 'axios';

// 创建axios实例
const request: AxiosInstance = axios.create({
  baseURL: '', // 使用vite代理，所以这里为空
  timeout: 10000, // 请求超时时间
});

// 请求拦截器
request.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // 在发送请求之前做些什么，比如添加token
    const token = localStorage.getItem('access_token');
    if (token) {
      (config.headers as AxiosRequestHeaders).Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    // 对请求错误做些什么
    console.error('Request error:', error);
    return Promise.reject(error);
  }
);

// 响应拦截器
request.interceptors.response.use(
  (response: AxiosResponse) => {
    // 对响应数据做点什么
    return response;
  },
  (error) => {
    // 对响应错误做点什么
    if (error.response?.status === 401) {
      // Token过期或未授权，跳转到登录页
      localStorage.removeItem('access_token');
      localStorage.removeItem('user_info');
      window.location.href = '/login';
    }
    console.error('Response error:', error);
    return Promise.reject(error);
  }
);

export default request;