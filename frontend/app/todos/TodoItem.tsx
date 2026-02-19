"use client";

import { useState } from "react";
import { type Todo } from "../lib/api";

export function TodoItem({
  todo,
  onToggle,
  onDelete,
  onUpdate,
}: {
  todo: Todo;
  onToggle: (todo: Todo) => void;
  onDelete: (id: number) => void;
  onUpdate: (todo: Todo, title: string) => void;
}) {
  const [update, setUpdate] = useState(false);
  const [updateTitle, setUpdateTitle] = useState(todo.title);

  return (
    <>
      <li className="bg-white rounded-lg shadow-sm px-4 py-3 flex items-center gap-3">
        {/* チェックボックス */}
        <button
          onClick={() => onToggle(todo)}
          className={`w-5 h-5 rounded-full border-2 flex-shrink-0 flex items-center justify-center ${
            todo.done
              ? "bg-green-500 border-green-500"
              : "border-gray-300 hover:border-blue-400"
          }`}
        >
          {todo.done && (
            <svg
              className="w-3 h-3 text-white"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={3}
                d="M5 13l4 4L19 7"
              />
            </svg>
          )}
        </button>

        {/* タイトル */}
        <span
          className={`flex-1 ${todo.done ? "line-through text-gray-400" : ""}`}
        >
          {todo.title}
        </span>

        {/* 更新ボタン */}
        <button
          onClick={() => setUpdate(true)}
          className="text-gray-300 hover:text-blue-400 flex-shrink-0"
        >
          <svg
            className="w-4 h-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
            />
          </svg>
        </button>

        {/* 削除ボタン */}
        <button
          onClick={() => {
            if (confirm("Todoを削除しますか？")) {
              onDelete(todo.id);
            }
          }}
          className="text-gray-300 hover:text-red-400 flex-shrink-0"
        >
          <svg
            className="w-4 h-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </li>
      {update && (
        <li className="bg-blue-50 rounded-lg px-4 py-3 flex items-center gap-2">
          <input
            value={updateTitle}
            onChange={(e) => setUpdateTitle(e.currentTarget.value)}
            autoFocus
            className="flex-1 border border-blue-300 rounded-lg px-3 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={() => {
              onUpdate(todo, updateTitle);
              setUpdate(false);
            }}
            className="bg-blue-600 text-white text-sm rounded-lg px-3 py-1.5 hover:bg-blue-700"
          >
            変更
          </button>
          <button
            onClick={() => setUpdate(false)}
            className="text-sm text-gray-500 hover:text-gray-700 px-2 py-1.5"
          >
            キャンセル
          </button>
        </li>
      )}
    </>
  );
}
