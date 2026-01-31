<template>
  <div class="dashboard-container">
    <header class="dashboard-header">
      <div class="header-left">
        <h1>后台管理系统</h1>
      </div>
      <div class="header-right">
        <span class="user-info">{{ userInfo?.user_name || '用户' }}</span>
        <button class="logout-btn" @click="handleLogout">退出登录</button>
      </div>
    </header>
    
    <div class="dashboard-content">
      <aside class="sidebar">
        <nav>
          <ul>
            <li><a href="#" class="active">首页</a></li>
            <li><a href="#">用户管理</a></li>
            <li><a href="#">组织管理</a></li>
            <li><a href="#">系统设置</a></li>
            <li><a href="#">数据统计</a></li>
          </ul>
        </nav>
      </aside>
      
      <main class="main-content">
        <div class="welcome-card">
          <h2>欢迎回来，{{ userInfo?.user_name }}！</h2>
          <p>这是您的后台管理仪表板，您可以在这里管理用户和系统设置。</p>
        </div>
        
        <div class="stats-grid">
          <div class="stat-card">
            <h3>总用户数</h3>
            <p class="stat-number">1,234</p>
          </div>
          <div class="stat-card">
            <h3>在线用户</h3>
            <p class="stat-number">128</p>
          </div>
          <div class="stat-card">
            <h3>今日活跃</h3>
            <p class="stat-number">892</p>
          </div>
          <div class="stat-card">
            <h3>系统健康</h3>
            <p class="stat-number">正常</p>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

interface UserInfo {
  user_name: string
  role: string
  user_id: number
}

const router = useRouter()
const userInfo = ref<UserInfo | null>(null)

onMounted(() => {
  // 从localStorage获取用户信息
  const storedUserInfo = localStorage.getItem('user_info')
  if (storedUserInfo) {
    try {
      userInfo.value = JSON.parse(storedUserInfo)
    } catch (error) {
      console.error('Failed to parse user info:', error)
    }
  }
})

const handleLogout = () => {
  // 清除本地存储的token和用户信息
  localStorage.removeItem('access_token')
  localStorage.removeItem('user_info')
  
  // 跳转到登录页
  router.push('/login')
}
</script>

<style scoped>
.dashboard-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 40px;
  background: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-left h1 {
  margin: 0;
  color: #333;
  font-size: 24px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.user-info {
  color: #666;
  font-weight: 500;
  font-size: 16px;
}

.logout-btn {
  padding: 10px 20px;
  background: #ff4757;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.3s;
  font-size: 16px;
}

.logout-btn:hover {
  background: #ff3742;
}

.dashboard-content {
  display: flex;
  flex: 1;
}

.sidebar {
  width: 280px;
  background: #2c3e50;
  color: white;
  height: calc(100vh - 70px);
  position: fixed;
  top: 70px;
  left: 0;
  overflow-y: auto;
}

.sidebar nav ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.sidebar nav ul li {
  border-bottom: 1px solid #34495e;
}

.sidebar nav ul li a {
  display: block;
  padding: 16px 24px;
  color: #ecf0f1;
  text-decoration: none;
  transition: background-color 0.3s;
  font-size: 16px;
}

.sidebar nav ul li a:hover,
.sidebar nav ul li a.active {
  background: #34495e;
}

.main-content {
  flex: 1;
  margin-left: 280px;
  padding: 40px;
}

.welcome-card {
  background: white;
  padding: 40px;
  border-radius: 10px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  margin-bottom: 40px;
}

.welcome-card h2 {
  margin-top: 0;
  color: #333;
  font-size: 24px;
}

.welcome-card p {
  font-size: 16px;
  color: #666;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 30px;
}

.stat-card {
  background: white;
  padding: 30px;
  border-radius: 10px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.stat-card h3 {
  margin: 0 0 15px 0;
  color: #7f8c8d;
  font-size: 18px;
}

.stat-number {
  margin: 0;
  font-size: 32px;
  font-weight: bold;
  color: #2c3e50;
}

@media (max-width: 768px) {
  .dashboard-header {
    padding: 15px 20px;
  }
  
  .dashboard-content {
    flex-direction: column;
  }
  
  .sidebar {
    position: static;
    height: auto;
    width: 100%;
  }
  
  .main-content {
    margin-left: 0;
    padding: 20px;
  }
  
  .welcome-card {
    padding: 25px;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
    gap: 20px;
  }
}
</style>