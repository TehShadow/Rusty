# 🧭 API Reference — Rust Discord Clone

This project is a full-featured backend for a Discord-style chat app, built with **Axum**, **SQLx**, and **PostgreSQL**.

---

## 🔐 Authentication

| Method | Endpoint     | Body (JSON)                               | Description                         |
|--------|--------------|--------------------------------------------|-------------------------------------|
| POST   | `/register`  | `{ "username": "nova", "password": "..." }` | Register a new user                  |
| POST   | `/login`     | `{ "username": "nova", "password": "..." }` | Log in, receive JWT + session        |
| GET    | `/me`        | *(JWT in Authorization header)*            | Get current user info                |

---

## 👤 Users

| Method | Endpoint      | Description              |
|--------|---------------|--------------------------|
| GET    | `/users/:id`  | Fetch user by UUID       |

---

## 💬 Direct Messages

| Method | Endpoint         | Body (JSON)                       | Description                     |
|--------|------------------|------------------------------------|---------------------------------|
| POST   | `/dm/:user_id`   | `{ "content": "Hello!" }`         | Send direct message             |
| GET    | `/dm/:user_id`   | *(none)*                          | Load chat history with a user   |

---

## 🏠 Rooms (Servers)

| Method | Endpoint                 | Body (JSON)                         | Description                  |
|--------|--------------------------|--------------------------------------|------------------------------|
| POST   | `/rooms`                 | `{ "name": "Rust Fans" }`           | Create a new room            |
| GET    | `/rooms`                 | *(none)*                            | List joined rooms            |
| POST   | `/rooms/:id/join`        | *(none)*                            | Join a room by ID            |
| POST   | `/rooms/:id/messages`    | `{ "content": "What's up!" }`       | Send message to a room       |
| GET    | `/rooms/:id/messages`    | *(none)*                            | Get all room messages        |

---

## 👥 Relationships (Friends / Block)

| Method | Endpoint                          | Description                        |
|--------|-----------------------------------|------------------------------------|
| POST   | `/relationships/:id`              | Send friend request                |
| POST   | `/relationships/:id/accept`       | Accept incoming request            |
| POST   | `/relationships/:id/block`        | Block a user                       |
| DELETE | `/relationships/:id`              | Remove friendship or block         |
| GET    | `/relationships/friends`          | List all friends                   |
| GET    | `/relationships/pending`          | View pending requests              |

---

## 🧪 Development & Testing

| Method | Endpoint        | Description           |
|--------|------------------|------------------------|
| GET    | `/health`        | Health check route     |
| POST   | `/dev/flush-db`  | Reset database (dev)   |

---

## 🔐 Authorization

Most routes require:

- ✅ `Authorization: Bearer <JWT>`
- ✅ Valid session ID (checked against DB)

> Use `/register` and `/login` to obtain a valid token.

---

## 📦 Tech Stack

- [x] Rust + Axum + SQLx
- [x] PostgreSQL
- [x] JWT + Session hybrid auth
- [x] Modular route + handler structure
- [x] Secure password hashing (Argon2)

---