#!/bin/bash

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Current script directory: $SCRIPT_DIR"
cd "$SCRIPT_DIR"

# 检查是否已经在运行同名容器
if podman ps -a --format "{{.Names}}" | grep -q "axum-demo-v1"; then
    echo "Stopping and removing existing container..."
    podman stop axum-demo-v1 2>/dev/null || true
    podman rm axum-demo-v1 2>/dev/null || true
fi

# 构建镜像
echo "Building image..."
podman build -t axum-demo .

# 运行容器 -p <宿主机端口>:<容器端口>
echo "Running container..."
podman run --name axum-demo-v1 -d -p 3000:3000 axum-demo

echo "Container started successfully!"
echo "Access your application at http://localhost:3000"