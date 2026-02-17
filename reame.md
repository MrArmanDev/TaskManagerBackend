# 🚀 Task Manager Backend (Rust + Axum)

A secure and production-ready Task Management Backend built with **Rust**, **Axum**, and **PostgreSQL**.

## 🛠 Tech Stack

- 🦀 Rust
- ⚡ Axum (Web Framework)
- 🐘 PostgreSQL
- 🗄 SQLx (Async DB Driver)
- 🔐 JWT (jsonwebtoken)
- 🔒 Argon2 (Password Hashing)
- 🧵 Tokio (Async Runtime)
- 🛡 Tower HTTP (CORS & Tracing)

---


## ✨ Features

- ✅ User Registration & Login
- 🔐 JWT Authentication
- 🔒 Secure password hashing (Argon2)
- 📋 Task CRUD operations
- 🛡 Account lock after 3 failed login attempts
- ⏳ Auto reset login attempts every 30 minutes
- 🌐 CORS enabled
- 📜 Structured error handling

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


Architecture follows:

Router → Handler → Service → Repository → Database

## ⚙️ Environment Variables

Create a `.env` file in the root:

DATABASE_URL=postgres://username:password@localhost:5432/taskdb
SECRET_KEY=your_super_secret_key
PORT=8000


---

## 🗄 Database Setup

Make sure PostgreSQL is running.

Run migrations: sqlx migrate run


Tables created:

- users
- tasks
- custom ENUM types (user_role, task_status)

---

## 🚀 Run the Project

```bash
cargo run

http://127.0.0.1:8000


# Register
POST /auth/api/auth/register
{
  "username": "test",
  "email": "test@gmail.com",
  "password": "password_123"
}


POST /auth/api/auth/login
{
  "email": "test@gmail.com",
  "password": "password_123"
}

# Returns
{
  "token": "JWT_TOKEN",
  "user": {
    "id": 1,
    "username": "test",
    "email": "test@gmail.com"
  }
}

Authorization: Bearer <JWT_TOKEN>

Create Task
POST /task/api/auth/create

{
  "title": "Rust Task",
  "description": "Learning Axum backend",
  "status": "Pending"
}

Get All Tasks
GET /task/api/auth/get


Update Task
PUT /task/api/auth/update

{
  "id": 1,
  "title": "Updated Title",
  "description": "Updated description",
  "status": "Completed"
}


Delete Task
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


