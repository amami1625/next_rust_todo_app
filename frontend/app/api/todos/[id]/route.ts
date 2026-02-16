import { NextResponse } from "next/server";

type Params = { id: string };

export async function GET(_: Request, ctx: { params: Promise<Params> }) {
  const base = process.env.AXUM_API_BASE;
  if (!base) return NextResponse.json({ error: "AXUM_API_BASE is not set" }, { status: 500 });

  const { id } = await ctx.params;

  const res = await fetch(`${base}/todos/${id}`, { cache: "no-store" });
  const text = await res.text();

  return new NextResponse(text, {
    status: res.status,
    headers: { "content-type": res.headers.get("content-type") ?? "application/json" },
  });
}

export async function PATCH(req: Request, ctx: { params: Promise<Params> }) {
  const base = process.env.AXUM_API_BASE;
  if (!base) return NextResponse.json({ error: "AXUM_API_BASE is not set" }, { status: 500 });

  const { id } = await ctx.params;

  const body = await req.text();
  const res = await fetch(`${base}/todos/${id}`, {
    method: "PATCH",
    headers: { "content-type": "application/json" },
    body,
  });

  const text = await res.text();
  return new NextResponse(text, {
    status: res.status,
    headers: { "content-type": res.headers.get("content-type") ?? "application/json" },
  });
}

export async function DELETE(_: Request, ctx: { params: Promise<Params> }) {
  const base = process.env.AXUM_API_BASE;
  if (!base) return NextResponse.json({ error: "AXUM_API_BASE is not set" }, { status: 500 });

  const { id } = await ctx.params;

  const res = await fetch(`${base}/todos/${id}`, { method: "DELETE" });

  // 204 は body が無いのでそのまま返すのが安全
  return new NextResponse(null, { status: res.status });
}
