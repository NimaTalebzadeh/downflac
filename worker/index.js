const MIN_HOURS = 1;
const MAX_HOURS = 168;

function hexId() {
  const b = crypto.getRandomValues(new Uint8Array(8));
  return [...b].map((x) => x.toString(16).padStart(2, "0")).join("");
}

function json(body, status = 200) {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json" },
  });
}

function bad(status, error) {
  return json({ error }, status);
}

async function create(req, env) {
  let b;
  try {
    b = await req.json();
  } catch {
    return bad(400, "bad json");
  }
  for (const f of ["id", "time", "sign", "format", "bitrate", "label"]) {
    if (typeof b[f] !== "string" || b[f] === "") return bad(400, `missing ${f}`);
  }
  const hours = Number(b.expiry_hours);
  if (!Number.isInteger(hours) || hours < MIN_HOURS || hours > MAX_HOURS) {
    return bad(400, `expiry_hours must be ${MIN_HOURS}..${MAX_HOURS}`);
  }
  const rec = {
    id: b.id,
    time: b.time,
    sign: b.sign,
    format: b.format,
    bitrate: b.bitrate,
    label: b.label,
    pic: typeof b.pic === "string" ? b.pic : "",
  };
  const share = hexId();
  await env.SHARES.put(share, JSON.stringify(rec), {
    expirationTtl: hours * 3600,
  });
  return json({ share, url: `${new URL(req.url).origin}/#s/${share}` });
}

async function resolve(req, env) {
  let b;
  try {
    b = await req.json();
  } catch {
    return bad(400, "bad json");
  }
  if (typeof b.share !== "string" || !/^[0-9a-f]{16}$/.test(b.share)) {
    return bad(400, "bad share");
  }
  const raw = await env.SHARES.get(b.share);
  if (!raw) return bad(404, "not found or expired");
  return json(JSON.parse(raw));
}

export default {
  async fetch(req, env) {
    const { pathname } = new URL(req.url);
    if (req.method !== "POST") return new Response(null, { status: 404 });
    if (pathname === "/api/share") return create(req, env);
    if (pathname === "/api/resolve") return resolve(req, env);
    return new Response(null, { status: 404 });
  },
};