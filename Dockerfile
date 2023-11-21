FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/demo-task-manager-service .
CMD ["/app/demo-task-manager-service"]