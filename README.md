# 🦀 RustyTasks - A Modern Todo Application

A full-stack todo application built with **Rust** (backend) and **React** (frontend), showcasing modern architecture patterns and best practices.

## 🚀 Features

- **Full-stack Application**: REST API backend with React frontend
- **Command Line Interface**: CLI tool for managing tasks
- **Modern Architecture**: Modular, organized codebase with separation of concerns
- **Type Safety**: Full TypeScript on frontend, strong typing in Rust
- **Error Handling**: Comprehensive error handling with user-friendly messages
- **Responsive UI**: Modern, clean interface with loading states and feedback
- **Database Integration**: PostgreSQL with connection pooling
- **Environment Configuration**: Flexible configuration management

## 📁 Project Structure

```
├── src/                     # Rust backend
│   ├── lib.rs              # Shared library modules
│   ├── main.rs             # Web server (bin: server)
│   ├── cli_main.rs         # CLI application (bin: cli)
│   ├── models.rs           # Data models and types
│   ├── database.rs         # Database service layer
│   ├── handlers.rs         # HTTP request handlers
│   ├── error.rs            # Custom error types
│   └── config.rs           # Configuration management
├── frontend/                # React frontend
│   ├── src/
│   │   ├── components/     # Reusable UI components
│   │   │   ├── TaskInput/  # Task creation component
│   │   │   ├── TaskList/   # Task display component
│   │   │   ├── ErrorMessage/   # Error display
│   │   │   └── LoadingSpinner/ # Loading indicator
│   │   ├── hooks/          # Custom React hooks
│   │   │   └── useTasks.ts # Task state management
│   │   ├── services/       # API communication
│   │   │   └── taskApi.ts  # Task API service
│   │   ├── types/          # TypeScript type definitions
│   │   └── utils/          # Utility functions
│   └── public/             # Static assets
└── Cargo.toml              # Rust project configuration
```

## 🛠️ Technology Stack

### Backend (Rust)
- **actix-web**: High-performance web framework
- **sqlx**: Async SQL toolkit with compile-time checked queries
- **serde**: Serialization/deserialization framework
- **uuid**: UUID generation and handling
- **dotenv**: Environment variable management
- **tokio**: Async runtime

### Frontend (React + TypeScript)
- **React 19**: Modern React with hooks
- **TypeScript**: Type-safe JavaScript
- **Custom Hooks**: For state management
- **CSS Modules**: Component-scoped styling
- **Fetch API**: HTTP client for API communication

### Database
- **PostgreSQL**: Robust relational database

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- Node.js (v16 or higher)
- PostgreSQL database
- Git

### 1. Clone the Repository

```bash
git clone https://github.com/ktaha2/todo-cli-rust.git
cd todo-cli-rust
```

### 2. Database Setup

Create a PostgreSQL database and run the following SQL:

```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
```

### 3. Environment Configuration

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgresql://username:password@localhost/database_name
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

For the frontend, create `frontend/.env`:

```env
REACT_APP_API_BASE_URL=http://localhost:8080
```

### 4. Backend Setup

```bash
# Install dependencies and build
cargo build

# Run the web server
cargo run --bin server

# Or run the CLI tool
cargo run --bin cli
```

### 5. Frontend Setup

```bash
cd frontend
npm install

# For development (with legacy OpenSSL support for older React Scripts)
NODE_OPTIONS="--openssl-legacy-provider" npm start

# For production build
NODE_OPTIONS="--openssl-legacy-provider" npm run build
```

## 🏗️ Architecture Overview

### Backend Architecture

The backend follows a **modular architecture** with clear separation of concerns:

- **Models**: Data structures and type definitions
- **Database Service**: Business logic and data access layer
- **Handlers**: HTTP request/response handling
- **Error Management**: Centralized error types and handling
- **Configuration**: Environment-based settings

#### Key Benefits:
- **Shared Logic**: Both web API and CLI use the same business logic
- **Type Safety**: Compile-time guarantees with Rust's type system
- **Error Handling**: Comprehensive error types with proper HTTP responses
- **Testing**: Modular structure enables easy unit testing

### Frontend Architecture

The frontend uses a **component-based architecture** with modern React patterns:

- **Components**: Focused, reusable UI components
- **Custom Hooks**: Encapsulated state management logic
- **Services**: API communication layer
- **Types**: Shared TypeScript interfaces
- **Utils**: Configuration and utility functions

#### Key Benefits:
- **Separation of Concerns**: Components focus on UI, hooks on logic
- **Type Safety**: Full TypeScript coverage
- **Error Handling**: User-friendly error messages and loading states
- **Maintainability**: Small, focused components are easy to maintain

## 🔧 API Endpoints

### Health Check
- `GET /` - API health status

### Tasks
- `GET /tasks` - Retrieve all tasks
- `POST /tasks` - Create a new task
- `PUT /tasks/{id}/complete` - Toggle task completion status
- `DELETE /tasks/completed` - Delete all completed tasks

### Request/Response Examples

**Create Task:**
```json
POST /tasks
{
  "title": "Learn Rust"
}

Response:
{
  "status": "success",
  "task": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Learn Rust",
    "completed": false
  }
}
```

## 🧪 Testing

```bash
# Backend tests
cargo test

# Frontend tests
cd frontend
npm test
```

## 🔧 Development

### Adding New Features

1. **Backend**: Add models in `models.rs`, business logic in `database.rs`, and endpoints in `handlers.rs`
2. **Frontend**: Create components in `components/`, add API methods in `services/taskApi.ts`, and manage state in hooks

### Code Style

- **Rust**: Follow standard Rust conventions with `cargo fmt`
- **TypeScript**: Use strict TypeScript configuration
- **CSS**: Component-scoped styles with consistent naming

## 📈 Performance Considerations

- **Backend**: Uses connection pooling for database efficiency
- **Frontend**: Components re-render only when necessary
- **Caching**: API responses can be cached for better performance
- **Bundle Size**: Tree-shaking eliminates unused code

## 🔒 Security

- **Input Validation**: All inputs are validated on both client and server
- **Error Handling**: Sensitive information is not exposed in error messages
- **CORS**: Configured for development (adjust for production)
- **SQL Injection**: Protected by using parameterized queries

## 🚀 Deployment

### Backend
```bash
cargo build --release
./target/release/server
```

### Frontend
```bash
cd frontend
NODE_OPTIONS="--openssl-legacy-provider" npm run build
# Serve the `build` directory with a web server
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📝 License

This project is open source and available under the MIT License.

## 🙏 Acknowledgments

- Built with love using Rust and React
- Inspired by modern web development practices
- Thanks to the open source community for amazing tools