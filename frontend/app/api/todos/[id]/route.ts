import { proxyToAxum, envErrorResponse } from "@/lib/axum";

type Params = { id: string };

export async function GET(_: Request, ctx: { params: Promise<Params> }) {
  try {
    const { id } = await ctx.params;
    return await proxyToAxum(`/todos/${id}`, { cache: "no-store" });
  } catch {
    return envErrorResponse();
  }
}

export async function PATCH(req: Request, ctx: { params: Promise<Params> }) {
  try {
    const { id } = await ctx.params;
    const body = await req.text();
    return await proxyToAxum(`/todos/${id}`, {
      method: "PATCH",
      headers: { "content-type": "application/json" },
      body,
    });
  } catch {
    return envErrorResponse();
  }
}

export async function DELETE(_: Request, ctx: { params: Promise<Params> }) {
  try {
    const { id } = await ctx.params;
    return await proxyToAxum(`/todos/${id}`, { method: "DELETE" });
  } catch {
    return envErrorResponse();
  }
}
