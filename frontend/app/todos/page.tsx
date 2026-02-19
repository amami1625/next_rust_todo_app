"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import {
  type Todo,
  listTodos,
  createTodo,
  toggleTodo,
  deleteTodo,
  updateTodo,
} from "../lib/api";
import { TodoItem } from "./TodoItem";

export default function TodosPage() {
  const router = useRouter();
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTitle, setNewTitle] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);

  // トークンを localStorage から取得する関数
  function getToken(): string | null {
    return localStorage.getItem("token");
  }

  // 認証チェック＆Todo 取得
  useEffect(() => {
    const token = getToken();
    if (!token) {
      router.push("/");
      return;
    }
    listTodos(token)
      .then((res) => setTodos(res.data))
      .catch(() => {
        // トークンが無効なら再ログイン
        localStorage.removeItem("token");
        router.push("/");
      })
      .finally(() => setLoading(false));
  }, [router]);

  // Todo 作成
  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    if (!newTitle.trim()) return;
    setError("");
    const token = getToken();
    if (!token) return;
    try {
      const todo = await createTodo(token, newTitle.trim());
      setTodos((prev) => [...prev, todo]);
      setNewTitle("");
    } catch (err) {
      setError(err instanceof Error ? err.message : "作成に失敗しました");
    }
  }

  // 完了状態の切り替え
  async function handleToggle(todo: Todo) {
    const token = getToken();
    if (!token) return;
    try {
      const updated = await toggleTodo(token, todo.id, !todo.done);
      setTodos((prev) => prev.map((t) => (t.id === updated.id ? updated : t)));
    } catch (err) {
      setError(err instanceof Error ? err.message : "更新に失敗しました");
    }
  }

  // タイトルの更新
  async function handleUpdate(todo: Todo, title: string) {
    const token = getToken();
    if (!token) return;
    try {
      const updated = await updateTodo(token, todo.id, title);
      setTodos((prev) => prev.map((t) => (t.id === updated.id ? updated : t)));
    } catch (err) {
      setError(err instanceof Error ? err.message : "更新に失敗しました");
    }
  }

  // Todo 削除
  async function handleDelete(id: number) {
    const token = getToken();
    if (!token) return;
    try {
      await deleteTodo(token, id);
      setTodos((prev) => prev.filter((t) => t.id !== id));
    } catch (err) {
      setError(err instanceof Error ? err.message : "削除に失敗しました");
    }
  }

  // ログアウト
  function handleLogout() {
    localStorage.removeItem("token");
    router.push("/");
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <p className="text-gray-500">読み込み中...</p>
      </div>
    );
  }

  const doneTodos = todos.filter((t) => t.done);
  const pendingTodos = todos.filter((t) => !t.done);

  return (
    <div className="min-h-screen bg-gray-50">
      {/* ヘッダー */}
      <header className="bg-white shadow-sm">
        <div className="max-w-lg mx-auto px-4 py-4 flex items-center justify-between">
          <h1 className="text-xl font-bold">Todo</h1>
          <button
            onClick={handleLogout}
            className="text-sm text-gray-500 hover:text-gray-700"
          >
            ログアウト
          </button>
        </div>
      </header>

      <main className="max-w-lg mx-auto px-4 py-6 space-y-6">
        {/* Todo 追加フォーム */}
        <form onSubmit={handleCreate} className="flex gap-2">
          <input
            type="text"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
            placeholder="新しい Todo を入力..."
            className="flex-1 border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            type="submit"
            className="bg-blue-600 text-white rounded-lg px-4 py-2 font-medium hover:bg-blue-700"
          >
            追加
          </button>
        </form>

        {error && <p className="text-red-500 text-sm">{error}</p>}

        {/* 未完了 */}
        {pendingTodos.length > 0 && (
          <section>
            <h2 className="text-sm font-medium text-gray-500 mb-2">
              未完了 ({pendingTodos.length})
            </h2>
            <ul className="space-y-2">
              {pendingTodos.map((todo) => (
                <TodoItem
                  key={todo.id}
                  todo={todo}
                  onToggle={handleToggle}
                  onDelete={handleDelete}
                  onUpdate={handleUpdate}
                />
              ))}
            </ul>
          </section>
        )}

        {/* 完了済み */}
        {doneTodos.length > 0 && (
          <section>
            <h2 className="text-sm font-medium text-gray-500 mb-2">
              完了済み ({doneTodos.length})
            </h2>
            <ul className="space-y-2">
              {doneTodos.map((todo) => (
                <TodoItem
                  key={todo.id}
                  todo={todo}
                  onToggle={handleToggle}
                  onDelete={handleDelete}
                  onUpdate={handleUpdate}
                />
              ))}
            </ul>
          </section>
        )}

        {todos.length === 0 && (
          <p className="text-center text-gray-400 py-12">
            Todo がまだありません
          </p>
        )}
      </main>
    </div>
  );
}

