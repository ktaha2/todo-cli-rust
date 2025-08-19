// Task model interface
export interface Task {
  id: string;
  title: string;
  completed: boolean;
}

// Request/Response interfaces
export interface NewTaskRequest {
  title: string;
}

export interface TaskResponse {
  status: string;
  task?: Task;
}

export interface DeleteResponse {
  status: string;
  count: number;
}

// API response wrapper
export interface ApiResponse<T = any> {
  data?: T;
  error?: string;
}

// Configuration
export interface AppConfig {
  apiBaseUrl: string;
}