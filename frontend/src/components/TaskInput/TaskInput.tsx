import React, { useState } from 'react';
import './TaskInput.css';

interface TaskInputProps {
  onAddTask: (title: string) => Promise<boolean>;
  isDisabled?: boolean;
}

const TaskInput: React.FC<TaskInputProps> = ({ onAddTask, isDisabled = false }) => {
  const [title, setTitle] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!title.trim() || isSubmitting || isDisabled) {
      return;
    }

    setIsSubmitting(true);
    try {
      const success = await onAddTask(title);
      if (success) {
        setTitle('');
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form className="task-input-form" onSubmit={handleSubmit}>
      <input
        type="text"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        placeholder="New task title"
        className="task-input"
        disabled={isDisabled || isSubmitting}
      />
      <button 
        type="submit"
        className="add-task-btn"
        disabled={!title.trim() || isDisabled || isSubmitting}
      >
        {isSubmitting ? '⏳ Adding...' : 'Add Task'}
      </button>
    </form>
  );
};

export default TaskInput;