FROM rust:1.91 as builder
WORKDIR /usr/src/axum-web
COPY src ./src
COPY Cargo.toml .env ./
RUN cargo build --release

# 使用 Debian slim 镜像作为运行时环境
FROM debian:bookworm-slim

# 安装运行时所需的库
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建应用用户
RUN useradd -ms /bin/bash app

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /usr/src/axum-web/target/release/axum-web .

# 复制环境配置文件（如果有的话）
COPY .env* ./

# 更改文件所有权
RUN chown -R app:app .

# 切换到非 root 用户
USER app

# 暴露端口（根据您的 Axum 应用配置相应端口）
EXPOSE 3000

# 启动应用
CMD ["./axum-web"]