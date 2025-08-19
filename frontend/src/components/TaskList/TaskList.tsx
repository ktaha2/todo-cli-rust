import React from 'react';
import { Task } from '../../types';
import './TaskList.css';

interface TaskListProps {
  tasks: Task[];
  onToggleTask: (taskId: string) => Promise<boolean>;
  onDeleteCompleted: () => Promise<boolean>;
  isDisabled?: boolean;
}

const TaskList: React.FC<TaskListProps> = ({ 
  tasks, 
  onToggleTask, 
  onDeleteCompleted, 
  isDisabled = false 
}) => {
  const completedTasks = tasks.filter(task => task.completed);
  const hasCompletedTasks = completedTasks.length > 0;

  if (tasks.length === 0) {
    return (
      <div className="no-tasks">
        <p>📝 No tasks found. Add your first task above!</p>
      </div>
    );
  }

  return (
    <div className="task-list">
      <h2>Tasks ({tasks.length})</h2>
      
      <ul className="tasks">
        {tasks.map((task) => (
          <li 
            key={task.id} 
            className={`task-item ${task.completed ? 'completed' : ''}`}
          >
            <label className="task-label">
              <input
                type="checkbox"
                checked={task.completed}
                onChange={() => onToggleTask(task.id)}
                disabled={isDisabled}
                className="task-checkbox"
              />
              <span className="task-title">{task.title}</span>
            </label>
          </li>
        ))}
      </ul>

      {hasCompletedTasks && (
        <div className="task-actions">
          <button 
            className="delete-completed-btn"
            onClick={onDeleteCompleted}
            disabled={isDisabled}
          >
            🗑️ Delete Completed Tasks ({completedTasks.length})
          </button>
        </div>
      )}
    </div>
  );
};

export default TaskList;