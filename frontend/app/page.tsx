"use client";

import { useEffect, useState } from "react";

type Todo = {
  id: number;
  title: string;
  completed: boolean;
};

export default function Home() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const run = async () => {
      try {
        const res = await fetch("http://localhost:3001/todos");
        if (!res.ok) {
          throw new Error(`HTTP ${res.status}`);
        }
        const data = (await res.json()) as Todo[];
        setTodos(data);
      } catch (e) {
        setError(e instanceof Error ? e.message : "Unknown error");
      }
    };
    run();
  }, []);

  return (
    <main style={{ padding: 24 }}>
      <h1>Todos</h1>

      {error && <p style={{ color: "crimson" }}>Failed to load: {error}</p>}

      <ul>
        {todos.map((t) => (
          <li key={t.id}>
            <label
              style={{ textDecoration: t.completed ? "line-through" : "none" }}
            >
              <input type="checkbox" checked={t.completed} readOnly /> {t.title}
            </label>
          </li>
        ))}
      </ul>
    </main>
  );
}
