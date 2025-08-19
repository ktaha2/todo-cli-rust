import React from 'react';
import { useTasks } from './hooks/useTasks';
import TaskInput from './components/TaskInput';
import TaskList from './components/TaskList';
import LoadingSpinner from './components/LoadingSpinner';
import ErrorMessage from './components/ErrorMessage';
import './App.css';

function App() {
  const {
    tasks,
    isLoading,
    error,
    addTask,
    toggleTask,
    deleteCompletedTasks,
    clearError
  } = useTasks();

  return (
    <div className="app">
      <header className="app-header">
        <h1>🦀 RustyTasks</h1>
        <p>A modern task manager built with Rust and React</p>
      </header>

      <main className="app-main">
        {error && (
          <ErrorMessage 
            message={error} 
            onDismiss={clearError}
          />
        )}

        <TaskInput 
          onAddTask={addTask}
          isDisabled={isLoading}
        />

        {isLoading ? (
          <LoadingSpinner />
        ) : (
          <TaskList
            tasks={tasks}
            onToggleTask={toggleTask}
            onDeleteCompleted={deleteCompletedTasks}
            isDisabled={isLoading}
          />
        )}
      </main>
    </div>
  );
}

export default App;
