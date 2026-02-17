# App logs

cargo check       
warning: unused variable: `body`
  --> src/handlers/auth_handler.rs:37:9
   |
37 |     let body = serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response));
   |         ^^^^ help: if this is intentional, prefix it with an underscore: `_body`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `backend` (bin "backend") generated 1 warning (run `cargo fix --bin "backend" -p backend` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.00s

░▒▓ 󰣇  …/backend   main   v1.93.0   23:07  
❯ 

# run app
 cargo watch -x run
[Running 'cargo run']
warning: unused variable: `body`
  --> src/handlers/auth_handler.rs:37:9
   |
37 |     let body = serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response));
   |         ^^^^ help: if this is intentional, prefix it with an underscore: `_body`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `backend` (bin "backend") generated 1 warning (run `cargo fix --bin "backend" -p backend` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/backend`
database connect
server running on http://127.0.0.1:8080


## register user 
@URL = http://127.0.0.1:8080
@email = TestUser@gmail.com
@username = TestUser

@authToken = eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjkwLCJlbWFpbCI6InRlc3QyMkBnbWFpbC5jb20iLCJleHAiOjE3NzE0MzA1Mjh9.nGZZqLGmXdjeCQzJgcUvWZH5RU4vy04SOtor2Ubyv6Q
### Health check
GET {{URL}}/health

### Register
POST {{URL}}/auth/api/auth/register
Content-Type: application/json

{
    "username": "{{username}}",
    "email": "{{email}}",
    "password": "password_123"
}


# response
HTTP/1.1 201 Created
content-type: application/json
set-cookie: token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjkxLCJlbWFpbCI6IlRlc3RVc2VyQGdtYWlsLmNvbSIsImV4cCI6MTc3MTQzNjQ4NH0.-jcoI0bmMJE_VGXIiuuOCml8X4JPEvp92vw4izSNOf4; HttpOnly; path=/; Max-Age=36000; SameSite=Lax
vary: origin, access-control-request-method, access-control-request-headers
access-control-allow-origin: *
content-length: 236
connection: close
date: Tue, 17 Feb 2026 17:41:24 GMT

{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjkxLCJlbWFpbCI6IlRlc3RVc2VyQGdtYWlsLmNvbSIsImV4cCI6MTc3MTQzNjQ4NH0.-jcoI0bmMJE_VGXIiuuOCml8X4JPEvp92vw4izSNOf4",
  "user": {
    "id": 91,
    "username": "TestUser",
    "email": "TestUser@gmail.com"
  }
}



# login 

### Login
POST {{URL}}/auth/api/auth/login
Content-Type: application/json

{
    "email": "{{email}}",
    "password": "password_123"
}

# response
HTTP/1.1 200 OK
content-type: application/json
set-cookie: token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjkxLCJlbWFpbCI6IlRlc3RVc2VyQGdtYWlsLmNvbSIsImV4cCI6MTc3MTQzNjU2NX0.Zl86n1MXOvh0OwKp5aW9jvmToeu-8GxWZspxo8jgmww; HttpOnly; path=/; Max-Age=36000; SameSite=Lax
vary: origin, access-control-request-method, access-control-request-headers
access-control-allow-origin: *
content-length: 236
connection: close
date: Tue, 17 Feb 2026 17:42:45 GMT
[LOGIN] response:
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjkxLCJlbWFpbCI6IlRlc3RVc2VyQGdtYWlsLmNvbSIsImV4cCI6MTc3MTQzNjU2NX0.Zl86n1MXOvh0OwKp5aW9jvmToeu-8GxWZspxo8jgmww",
  "user": {
    "id": 91,
    "username": "TestUser",
    "email": "TestUser@gmail.com"
  }
}

### Create Task
POST {{URL}}/task/api/auth/create
Authorization: Bearer {{authToken}}
Content-Type: application/json

{
    "title": "test 22",
    "description": "session 22",
    "status": "Pending"
}

# response 
HTTP/1.1 201 Created
content-type: application/json
vary: origin, access-control-request-method, access-control-request-headers
access-control-allow-origin: *
content-length: 170
connection: close
date: Tue, 17 Feb 2026 17:44:16 GMT

{
  "id": 45,
  "user_id": 90,
  "title": "test 22",
  "description": "session 22",
  "status": "Pending",
  "created_at": "2026-02-17T17:44:16.748345",
  "updated_at": "2026-02-17T17:44:16.748345"
}


### Task Update Request
PUT {{URL}}/task/api/auth/update
Authorization: Bearer {{authToken}}
Content-Type: application/json

{
    "id": 6,
    "title": "Rust Task Updated",
    "description": "I have successfully updated this task using HTTP client",
    "status": "Completed"
}


# response
HTTP/1.1 404 Not Found
content-type: application/json
vary: origin, access-control-request-method, access-control-request-headers
access-control-allow-origin: *
content-length: 66
connection: close
date: Tue, 17 Feb 2026 17:45:18 GMT

{
  "error": "task is not found or you are not the owner of the task"
}

#### Becuase task is not exits 


