## 项目介绍

- **技术栈**: Rust、Axum、Tonic(gRPC)、SeaORM、Redis、Consul
- **架构概览**: 本项目已经拆分为 gRPC 微服务架构，包含 3 个子工程：
  - **grpc-dsl**: 定义 gRPC 的 `.proto` 文件，并生成共享的实体与客户端/服务端接口
  - **core**: 核心业务服务，使用 SeaORM 访问数据库，对外暴露 gRPC 接口并在 Consul 中注册服务
  - **client**: Axum HTTP 网关，对外提供 HTTP API，内部通过 gRPC 调用 `core`，并使用 Redis 做缓存

## 项目结构

```text
axum-web
├── Cargo.toml              # Workspace 定义，包含 client/core/grpc-dsl
├── README.md
├── Dockerfile              # 旧单体版本的 Docker 构建脚本（尚未适配 workspace）
├── start.sh                # 使用 Podman 构建并运行镜像的脚本（目前仍基于旧 Dockerfile）
├── resources/              # 示例 SQL、HTTP 测试文件
├── client/                 # Axum HTTP 网关
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # HTTP 入口，负责日志初始化、Consul 服务发现、路由注册
│       ├── context/        # AppState、统一返回包装、错误类型
│       ├── controller/     # HTTP 控制器（/api/user/...）
│       └── redisconfig.rs  # Redis 封装
├── core/                   # 核心 gRPC 服务
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # gRPC Server 入口，注册到 Consul
│       ├── db.rs           # SeaORM 数据库连接
│       ├── dao/            # UserDao 接口与实现
│       └── entity/         # SeaORM 实体定义（mo_app_user 等）
└── grpc-dsl/               # gRPC 协议与生成代码
    ├── Cargo.toml
    ├── build.rs            # 使用 tonic-build 生成 Rust 代码
    ├── proto/
    │   └── user.proto      # 用户相关 gRPC 协议定义
    └── src/lib.rs          # `tonic::include_proto!("user")`
```

## 运行环境

- **Rust 版本**: 1.91 及以上
- **依赖服务**:
  - MySQL（通过 `DATABASE_URL` 访问）
  - Redis（通过 `REDIS_URL` 访问）
  - Consul（本机已安装，默认 `http://127.0.0.1:8500`）
- **推荐日志环境变量**:
  - `RUST_LOG=debug` 方便调试

### 关键环境变量

- **数据库与缓存**
  - `DATABASE_URL`：MySQL 连接串，core 使用
  - `REDIS_URL`：Redis 连接串，core/client 都会使用
- **Consul 相关**
  - `CONSUL_HTTP_ADDR`：Consul HTTP 地址，默认 `http://127.0.0.1:8500`
  - `CORE_SERVICE_HOST`：core 在 Consul 中注册的 Address，默认 `127.0.0.1`
- **HTTP / gRPC 端口**
  - `PORT`：client 对外 HTTP 监听端口，默认 `8080`
  - `CORE_GRPC_ADDR`：可选，手动指定 core 的 gRPC 地址（如 `http://127.0.0.1:50051`），设置后优先于 Consul

## 启动方式

> 假设本机 Consul agent 已经在运行，并监听 `CONSUL_HTTP_ADDR`（默认 `http://127.0.0.1:8500`）。

### 1. 启动 core（gRPC 微服务）

```bash
# 在项目根目录
RUST_LOG=debug \
DATABASE_URL="mysql://user:password@127.0.0.1:3306/db_name" \
REDIS_URL="redis://127.0.0.1:6379" \
cargo run -p core

# core 默认监听 0.0.0.0:50051
# 启动时会向 Consul 注册名为 core-user 的服务，并配置 TCP 健康检查
```

可选参数：

```bash
# 修改在 Consul 中注册的 Address
CORE_SERVICE_HOST=192.168.1.100 RUST_LOG=debug ... cargo run -p core

# 修改 Consul 地址
CONSUL_HTTP_ADDR=http://192.168.1.10:8500 RUST_LOG=debug ... cargo run -p core
```

### 2. 启动 client（HTTP 网关）

```bash
# 在项目根目录
RUST_LOG=debug \
REDIS_URL="redis://127.0.0.1:6379" \
PORT=8080 \
cargo run -p client

# client 启动时会执行以下优先级查找 core 地址：
# 1. 若设置了 CORE_GRPC_ADDR，则优先使用
# 2. 否则从 Consul 查询 core-user 的健康实例
# 3. 若 Consul 不可用，则退回到 http://127.0.0.1:50051
```

手动覆盖 core 地址的示例：

```bash
CORE_GRPC_ADDR=http://127.0.0.1:50051 \
RUST_LOG=debug \
REDIS_URL="redis://127.0.0.1:6379" \
PORT=8080 \
cargo run -p client
```

启动后，client 会对外暴露 HTTP API，默认监听 `0.0.0.0:8080`。

## HTTP 接口说明（client）

所有接口前缀为 `/api/user`：

- **POST `/api/user/add_user`**
  - **请求体(JSON)**：
    ```json
    {
      "id": 1,               // 新增可不传或 null；更新时必传
      "empId": "E001",
      "userName": "张三",
      "age": 20,
      "birthday": "2024-01-01 00:00:00"
    }
    ```
  - **行为**：通过 gRPC 调用 core 新增或更新用户，返回新增的 ID

- **POST `/api/user/query_user`**
  - **请求体**：无
  - **返回**：用户列表（来源于 core 的 gRPC 响应）

- **POST `/api/user/query_user_by_id`**
  - **请求体（`application/x-www-form-urlencoded`）**：
    - `id`: 用户 ID
  - **行为**：优先从 Redis 读取缓存；缓存不存在或格式不兼容时，调用 core 查询并回填缓存

- **POST `/api/user/delete_user`**
  - **请求体（`application/x-www-form-urlencoded`）**：
    - `id`: 用户 ID
  - **行为**：调用 core 删除用户，并异步清理 Redis 缓存

- **POST `/api/user/update_user`**
  - **请求体(JSON)**：同 `add_user`，但 `id` 必填
  - **行为**：调用 core 更新用户，并异步刷新 Redis 缓存

所有接口统一使用 `ResWrapper` 作为返回格式：

```json
{
  "code": 0,
  "msg": "success",
  "data": { ... }
}
```

## Consul 服务发现说明

- **core 注册**
  - 服务名称：`core-user`
  - 地址：由 `CORE_SERVICE_HOST` 和固定端口 `50051` 组成
  - 健康检查：TCP 检查 `<host>:50051`，间隔 10 秒

- **client 发现**
  - 调用 Consul API：`GET /v1/health/service/core-user?passing=true`
  - 从返回结果中取第一个健康实例的 `Service.Address` 和 `Service.Port`，拼出 gRPC 地址
  - 若 Consul 不可用或没有健康实例，则回退到 `CORE_GRPC_ADDR` 或默认 `http://127.0.0.1:50051`

## 容器化（当前状态）

- 根目录下的 `Dockerfile` 和 `start.sh` 仍然基于单体 `axum-web` 可执行文件的旧实现，尚未针对 workspace（client/core/grpc-dsl）进行调整。
- 若需要容器化当前微服务版本，建议分别为 `core` 与 `client` 编写独立 Dockerfile，并更新 `start.sh` 为以两个容器形式启动（或使用 docker-compose / Kubernetes）。
