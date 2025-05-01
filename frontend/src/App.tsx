import React, { useEffect, useState } from 'react';
import { Task } from './types';

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [newTitle, setNewTitle] = useState('');


  useEffect(() => {
    fetch('http://localhost:8080/tasks')
      .then((res) => res.json())
      .then((data) => setTasks(data))
      .catch((err) => console.error('Failed to fetch tasks:', err));
  }, []);

  const handleAddTask = () => {
    if (!newTitle.trim()) return;
  
    fetch('http://localhost:8080/tasks', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ title: newTitle }),
    })
      .then(() => {
        setNewTitle('');
        return fetch('http://localhost:8080/tasks');
      })
      .then((res) => res.json())
      .then((data) => setTasks(data))
      .catch((err) => console.error('Error adding task:', err));
  };

  const handleToggleComplete = (id: string) => {
    fetch(`http://localhost:8080/tasks/${id}/complete`, {
      method: 'PUT',
    })
      .then(() => fetch('http://localhost:8080/tasks'))
      .then((res) => res.json())
      .then((data) => setTasks(data))
      .catch((err) => console.error('Failed to toggle task completion:', err));
  };

  const handleDeleteCompleted = () => {
    fetch('http://localhost:8080/tasks/completed', {
      method: 'DELETE',
    })
      .then(() => fetch('http://localhost:8080/tasks'))
      .then((res) => res.json())
      .then((data) => setTasks(data))
      .catch((err) => console.error('Failed to delete completed tasks:', err));
  };
  

  return (
    <div style={{ padding: '2rem' }}>
      <h1>🦀 RustyTasks</h1>

      <input
        type="text"
        value={newTitle}
        onChange={(e) => setNewTitle(e.target.value)}
        placeholder="New task title"
        style={{ marginRight: '1rem' }}
      />
      <button onClick={handleAddTask}>Add Task</button>

      <h2 style={{ marginTop: '2rem' }}>Tasks</h2>

      {tasks.length === 0 ? (
        <p>No tasks found.</p>
      ) : (
<>  
      
        <ul>
          {tasks.map((task) => (
            <li key={task.id}>
              <input
                  type="checkbox"
                  checked={task.completed}
                  onChange={() => handleToggleComplete(task.id)}
                />

              {task.title}
            </li>
          ))}
        </ul>

        <button onClick={handleDeleteCompleted} style={{ marginTop: '1rem' }}>🗑️ Delete Completed Tasks </button>

      </>
      )}
      
    </div>
  );
}

export default App;
