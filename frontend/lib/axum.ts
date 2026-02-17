import { NextResponse } from "next/server";

function getBase() {
  const base = process.env.AXUM_API_BASE;
  if (!base) {
    throw new Error("AXUM_API_BASE is not set");
  }
  return base;
}

export async function proxyToAxum(path: string, init?: RequestInit) {
  const base = getBase();
  const res = await fetch(`${base}${path}`, init);

  // 204はボディなし
  if (res.status === 204) {
    return new NextResponse(null, { status: 204 });
  }

  const text = await res.text();

  return new NextResponse(text, {
    status: res.status,
    headers: {
      "content-type": res.headers.get("content-type") ?? "application/json",
    },
  });
}

export function envErrorResponse() {
  return NextResponse.json({ error: "AXUM_API_BASE is not set" }, { status: 500 });
}
