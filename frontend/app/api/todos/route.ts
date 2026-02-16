import { NextResponse } from "next/server";

export async function GET() {
  const base = process.env.AXUM_API_BASE;
  if (!base) {
    return NextResponse.json(
      { error: "AXUM_API_BASE is not set" },
      { status: 500 }
    );
  }

  const res = await fetch(`${base}/todos`, {
    cache: "no-store",
  });

  // ステータスを維持して中継
  const text = await res.text();
  return new NextResponse(text, {
    status: res.status,
    headers: {
      "content-type": res.headers.get("content-type") ?? "application/json",
    },
  });
}

export async function POST(req: Request) {
  const base = process.env.AXUM_API_BASE;
  if (!base) {
    return NextResponse.json({ error: "AXUM_API_BASE is not set" }, { status: 500 });
  }

  const body = await req.text(); // そのまま中継
  const res = await fetch(`${base}/todos`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body,
  });

  const text = await res.text();
  return new NextResponse(text, {
    status: res.status,
    headers: { "content-type": res.headers.get("content-type") ?? "application/json" },
  });
}
