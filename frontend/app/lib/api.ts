// バックエンドの URL
// "use client" コンポーネントはブラウザで実行されるため、
// ブラウザからアクセスできる URL（localhost）を使う
const BASE_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3000";

export type Todo = {
  id: number;
  user_id: number;
  title: string;
  done: boolean;
  created_at: string;
  updated_at: string;
};

export type PaginatedResponse = {
  data: Todo[];
  total: number;
  page: number;
  limit: number;
};

// ユーザー登録
export async function register(email: string, password: string): Promise<string> {
  const res = await fetch(`${BASE_URL}/auth/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password }),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? "登録に失敗しました");
  return json.token as string;
}

// ログイン
export async function login(email: string, password: string): Promise<string> {
  const res = await fetch(`${BASE_URL}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password }),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? "ログインに失敗しました");
  return json.token as string;
}

// Todo 一覧取得
export async function listTodos(token: string): Promise<PaginatedResponse> {
  const res = await fetch(`${BASE_URL}/todos`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? "取得に失敗しました");
  return json as PaginatedResponse;
}

// Todo 作成
export async function createTodo(token: string, title: string): Promise<Todo> {
  const res = await fetch(`${BASE_URL}/todos`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ title }),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? "作成に失敗しました");
  return json as Todo;
}

// Todo の完了状態を切り替え
export async function toggleTodo(token: string, id: number, done: boolean): Promise<Todo> {
  const res = await fetch(`${BASE_URL}/todos/${id}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ done }),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? "更新に失敗しました");
  return json as Todo;
}

// Todo のタイトルを更新
export async function updateTodo(token: string, id: number, title: string): Promise<Todo> {
  const res = await fetch(`${BASE_URL}/todos/${id}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ title }),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? "更新に失敗しました");
  return json as Todo;

}

// Todo 削除
export async function deleteTodo(token: string, id: number): Promise<void> {
  const res = await fetch(`${BASE_URL}/todos/${id}`, {
    method: "DELETE",
    headers: { Authorization: `Bearer ${token}` },
  });
  if (!res.ok) {
    const json = await res.json();
    throw new Error(json.error ?? "削除に失敗しました");
  }
}
