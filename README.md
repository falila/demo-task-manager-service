# demo-task-manager-service
The demo app showcases a task manager service implemented using the Actix Web Rust framework 

## Features

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation

1. Clone the repository:

   ```bash
   git clone git@github.com:falila/demo-task-manager-service.git
   cd task-manager-api
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the application:

   ```bash
   cargo run
   ```

   The API will be accessible at `http://localhost:8000`.

## API Endpoints

- **Create Task:**
  - Endpoint: `POST /tasks`
  - Request Body: JSON with task details.

- **Retrieve Tasks:**
  - Endpoint: `GET /tasks`
  - Returns a list of all tasks.

- **Retrieve Task by ID:**
  - Endpoint: `GET /tasks/{id}`
  - Returns details of the specified task.

- **Update Task:**
  - Endpoint: `PUT /tasks/{id}`
  - Request Body: JSON with updated task details.

- **Delete Task:**
  - Endpoint: `DELETE /tasks/{id}`
  - Deletes the specified task.

## Examples

### Create Task

```bash
curl -X POST -H "Content-Type: application/json" -d '{"title": "Example Task", "description": "This is a sample task."}' http://localhost:8000/tasks
```

### Retrieve Tasks

```bash
curl http://localhost:8000/tasks
```

### Retrieve Task by ID

```bash
curl http://localhost:8000/tasks/{task_id}
```

### Update Task

```bash
curl -X PUT -H "Content-Type: application/json" -d '{"title": "Updated Task", "description": "This task has been updated."}' http://localhost:8000/tasks/{task_id}
```

### Delete Task

```bash
curl -X DELETE http://localhost:8000/tasks/{task_id}
```

### Benchmark Task

```bash
ab -p task.json -T application/json -n 100000 -k -c 30 -q http://127.0.0.1:8080/tasks
```