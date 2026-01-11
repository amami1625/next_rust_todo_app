"use client";

import { useEffect, useState } from "react";

type Todo = {
  id: number;
  title: string;
  completed: boolean;
};

export default function Home() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [title, setTitle] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const loadTodos = async () => {
    const res = await fetch("http://localhost:3001/todos");
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = (await res.json()) as Todo[];
    setTodos(data);
  };

  useEffect(() => {
    loadTodos().catch((e) =>
      setError(e instanceof Error ? e.message : "Unknown error")
    );
  }, []);

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    const trimmed = title.trim();
    if (!trimmed) {
      setError("Title is required");
      return;
    }

    try {
      setIsSubmitting(true);
      const res = await fetch("http://localhost:3001/todos", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title: trimmed }),
      });

      if (!res.ok) {
        const text = await res.text();
        throw new Error(`HTTP ${res.status}: ${text}`);
      }

      setTitle("");
      await loadTodos();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <main style={{ padding: 24 }}>
      <h1>Todos</h1>

      <form onSubmit={onSubmit} style={{ marginBottom: 16 }}>
        <input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="New todo"
        />
        <button type="submit" disabled={isSubmitting} style={{ marginLeft: 8 }}>
          Add
        </button>
      </form>

      {error && <p style={{ color: "crimson" }}>Failed: {error}</p>}

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
