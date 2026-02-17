# 🚀 Task Manager Backend (Rust + Axum)

A secure, scalable, and production-ready Task Management Backend built using **Rust**, **Axum**, and **PostgreSQL**.

This backend provides authentication, JWT-based authorization, task management (CRUD), and built-in security mechanisms like account lockout protection and scheduled login reset.

---

# API testing 
.http file and logs files ----> api (folder) please check

## 🛠 Tech Stack

- 🦀 Rust
- ⚡ Axum (Web Framework)
- 🐘 PostgreSQL
- 🗄 SQLx (Async Database Driver)
- 🔐 jsonwebtoken (JWT Authentication)
- 🔒 Argon2 (Password Hashing)
- 🧵 Tokio (Async Runtime)
- 🛡 tower-http (CORS & Tracing)

---

## ✨ Features

- ✅ User Registration & Login
- 🔐 JWT Authentication (24-hour expiry)
- 🔒 Secure password hashing using Argon2
- 📋 Full Task CRUD operations
- 🛡 Account lock after 3 failed login attempts
- ⏳ Automatic reset of login attempts every 30 minutes
- 🌐 CORS enabled
- 📜 Centralized structured error handling
- 🏗 Clean layered architecture

---

## 🧠 Architecture


- **Router** → Defines API routes
- **Handler** → Extracts request data
- **Service** → Business logic
- **Repository** → Database queries
- **Database** → PostgreSQL

---

## 📂 Project Structure

src/
│
├── config/
├── db/
├── dto/
├── errors/
├── handlers/
├── middlewares/
├── models/
├── repository/
├── routers/
├── services/
└── utils/


---

## ⚙️ Environment Variables

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgres://username:password@localhost:5432/taskdb
SECRET_KEY=your_super_secret_key
PORT=8000

🗄 Database Setup

Make sure PostgreSQL is running.

1️⃣ Create Database
CREATE DATABASE taskdb;

2️⃣ Run Migrations
sqlx migrate run


This will create:
users table
tasks table

ENUM types:
user_role
task_status


🚀 Running the Project
cargo run

Server will start at:
http://127.0.0.1:8000

# Register
POST /auth/api/auth/register
{
  "username": "test",
  "email": "test@gmail.com",
  "password": "password_123"
}

# Login
POST /auth/api/auth/login
{
  "email": "test@gmail.com",
  "password": "password_123"
}

# Response
{
  "token": "JWT_TOKEN",
  "user": {
    "id": 1,
    "username": "test",
    "email": "test@gmail.com"
  }
}

📋 Task APIs (Protected Routes)

All task routes require:

Authorization: Bearer <JWT_TOKEN>

# Create Task
POST /task/api/auth/create

{
  "title": "Rust Task",
  "description": "Learning Axum backend",
  "status": "Pending"
}

# Get All Tasks
GET /task/api/auth/get

# Update Task
PUT /task/api/auth/update

{
  "id": 1,
  "title": "Updated Title",
  "description": "Updated description",
  "status": "Completed"
}

# Delete Task
DELETE /task/api/auth/delete

{
  "id": 1
}

🔐 Security

Passwords hashed using Argon2

JWT expires in 24 hours

Account locked after 3 failed attempts

Background scheduler resets attempts every 30 minutes

Proper HTTP status codes

Centralized error handling


🧪 Testing

You can test using:
Postman
VS Code REST Client
curl

📈 Future Improvements

Role-based authorization

Refresh token support

Rate limiting

Docker support

CI/CD integration

Swagger / OpenAPI documentation
