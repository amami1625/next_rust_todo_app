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
  const [priority, setPriority] = useState("low");
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
      const todo = await createTodo(token, newTitle.trim(), priority);
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
      <div className="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 flex items-center justify-center">
        <div className="flex flex-col items-center gap-3">
          <div className="w-6 h-6 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
          <p className="text-sm text-gray-400">読み込み中...</p>
        </div>
      </div>
    );
  }

  const doneTodos = todos.filter((t) => t.done);
  const pendingTodos = todos.filter((t) => !t.done);

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
      {/* ヘッダー */}
      <header className="bg-white/80 backdrop-blur-sm border-b border-gray-100 sticky top-0 z-10">
        <div className="max-w-lg mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-7 h-7 bg-blue-600 rounded-lg flex items-center justify-center">
              <svg className="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
              </svg>
            </div>
            <h1 className="text-lg font-bold text-gray-800">Todo</h1>
          </div>
          <button
            onClick={handleLogout}
            className="text-sm text-gray-400 hover:text-gray-600 transition-colors px-3 py-1.5 rounded-lg hover:bg-gray-100"
          >
            ログアウト
          </button>
        </div>
      </header>

      <main className="max-w-lg mx-auto px-4 py-6 space-y-5">
        {/* Todo 追加フォーム */}
        <form onSubmit={handleCreate} className="bg-white rounded-2xl border border-gray-100 shadow-sm p-4 space-y-3">
          <input
            type="text"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
            placeholder="新しい Todo を入力..."
            className="w-full border border-gray-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent placeholder-gray-300"
          />
          <div className="flex gap-2">
            <select
              name="priority"
              value={priority}
              onChange={(e) => setPriority(e.target.value)}
              className="border border-gray-200 rounded-xl px-3 py-2 text-sm text-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
            >
              <option value="low">低</option>
              <option value="middle">中</option>
              <option value="high">高</option>
            </select>
            <button
              type="submit"
              className="flex-1 bg-blue-600 text-white rounded-xl py-2 text-sm font-medium hover:bg-blue-700 transition-colors"
            >
              追加
            </button>
          </div>
        </form>

        {error && (
          <div className="bg-red-50 border border-red-100 text-red-600 text-sm rounded-xl px-4 py-3">
            {error}
          </div>
        )}

        {/* 未完了 */}
        {pendingTodos.length > 0 && (
          <section>
            <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2 px-1">
              未完了 · {pendingTodos.length}件
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
            <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2 px-1">
              完了済み · {doneTodos.length}件
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
          <div className="text-center py-16">
            <div className="w-12 h-12 bg-gray-100 rounded-2xl flex items-center justify-center mx-auto mb-3">
              <svg className="w-6 h-6 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
            <p className="text-sm text-gray-400">Todo がまだありません</p>
          </div>
        )}
      </main>
    </div>
  );
}
