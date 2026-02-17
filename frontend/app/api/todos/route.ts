import { proxyToAxum, envErrorResponse } from "@/lib/axum";

export async function GET() {
  try {
    return await proxyToAxum("/todos", { cache: "no-store" });
  } catch {
    return envErrorResponse();
  }
}

export async function POST(req: Request) {
  try {
    const body = await req.text();
    return await proxyToAxum("/todos", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body,
    });
  } catch {
    return envErrorResponse();
  }
}
