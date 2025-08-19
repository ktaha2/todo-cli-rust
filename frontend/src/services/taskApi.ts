import { Task, NewTaskRequest, TaskResponse, DeleteResponse, ApiResponse } from '../types';
import { config } from '../utils/config';

class ApiError extends Error {
  constructor(message: string, public status?: number) {
    super(message);
    this.name = 'ApiError';
  }
}

class TaskApiService {
  private baseUrl: string;

  constructor() {
    this.baseUrl = config.apiBaseUrl;
  }

  private async handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
    if (!response.ok) {
      let errorMessage = `HTTP error! status: ${response.status}`;
      
      try {
        const errorBody = await response.json();
        errorMessage = errorBody.error || errorMessage;
      } catch {
        // If we can't parse the error body, use the default message
      }
      
      throw new ApiError(errorMessage, response.status);
    }

    try {
      const data = await response.json();
      return { data };
    } catch (error) {
      throw new ApiError('Failed to parse response');
    }
  }

  private async makeRequest<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<ApiResponse<T>> {
    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, {
        headers: {
          'Content-Type': 'application/json',
          ...options.headers,
        },
        ...options,
      });

      return await this.handleResponse<T>(response);
    } catch (error) {
      if (error instanceof ApiError) {
        throw error;
      }
      
      return {
        error: error instanceof Error ? error.message : 'Network error occurred'
      };
    }
  }

  async getTasks(): Promise<ApiResponse<Task[]>> {
    return this.makeRequest<Task[]>('/tasks');
  }

  async createTask(newTask: NewTaskRequest): Promise<ApiResponse<TaskResponse>> {
    return this.makeRequest<TaskResponse>('/tasks', {
      method: 'POST',
      body: JSON.stringify(newTask),
    });
  }

  async toggleTaskCompletion(taskId: string): Promise<ApiResponse<TaskResponse>> {
    return this.makeRequest<TaskResponse>(`/tasks/${taskId}/complete`, {
      method: 'PUT',
    });
  }

  async deleteCompletedTasks(): Promise<ApiResponse<DeleteResponse>> {
    return this.makeRequest<DeleteResponse>('/tasks/completed', {
      method: 'DELETE',
    });
  }

  async checkHealth(): Promise<ApiResponse<{ status: string; message: string }>> {
    return this.makeRequest('/');
  }
}

// Export singleton instance
export const taskApi = new TaskApiService();
export { ApiError };