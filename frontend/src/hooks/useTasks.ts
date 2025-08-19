import { useState, useEffect, useCallback } from 'react';
import { Task } from '../types';
import { taskApi, ApiError } from '../services/taskApi';

interface UseTasksState {
  tasks: Task[];
  isLoading: boolean;
  error: string | null;
}

interface UseTasksActions {
  loadTasks: () => Promise<void>;
  addTask: (title: string) => Promise<boolean>;
  toggleTask: (taskId: string) => Promise<boolean>;
  deleteCompletedTasks: () => Promise<boolean>;
  clearError: () => void;
}

export const useTasks = (): UseTasksState & UseTasksActions => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const handleError = useCallback((err: unknown) => {
    if (err instanceof ApiError) {
      setError(err.message);
    } else if (err instanceof Error) {
      setError(err.message);
    } else {
      setError('An unexpected error occurred');
    }
  }, []);

  const clearError = useCallback(() => {
    setError(null);
  }, []);

  const loadTasks = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      const response = await taskApi.getTasks();
      if (response.error) {
        setError(response.error);
        return;
      }
      
      if (response.data) {
        setTasks(response.data);
      }
    } catch (err) {
      handleError(err);
    } finally {
      setIsLoading(false);
    }
  }, [handleError]);

  const addTask = useCallback(async (title: string): Promise<boolean> => {
    if (!title.trim()) {
      setError('Task title cannot be empty');
      return false;
    }

    try {
      setError(null);
      const response = await taskApi.createTask({ title: title.trim() });
      
      if (response.error) {
        setError(response.error);
        return false;
      }

      // Refresh tasks after successful creation
      await loadTasks();
      return true;
    } catch (err) {
      handleError(err);
      return false;
    }
  }, [loadTasks, handleError]);

  const toggleTask = useCallback(async (taskId: string): Promise<boolean> => {
    try {
      setError(null);
      const response = await taskApi.toggleTaskCompletion(taskId);
      
      if (response.error) {
        setError(response.error);
        return false;
      }

      // Refresh tasks after successful toggle
      await loadTasks();
      return true;
    } catch (err) {
      handleError(err);
      return false;
    }
  }, [loadTasks, handleError]);

  const deleteCompletedTasks = useCallback(async (): Promise<boolean> => {
    try {
      setError(null);
      const response = await taskApi.deleteCompletedTasks();
      
      if (response.error) {
        setError(response.error);
        return false;
      }

      // Refresh tasks after successful deletion
      await loadTasks();
      return true;
    } catch (err) {
      handleError(err);
      return false;
    }
  }, [loadTasks, handleError]);

  // Load tasks on mount
  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  return {
    tasks,
    isLoading,
    error,
    loadTasks,
    addTask,
    toggleTask,
    deleteCompletedTasks,
    clearError,
  };
};