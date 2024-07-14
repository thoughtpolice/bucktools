// buck2-log-server: log storage and retrieval system for buck2 logs

/** @jsx h */
/** @jsxFrag Fragment */
import {
  Fragment as _,
  h,
  Node,
  renderToString as renderHtmlToString,
} from "https://deno.land/x/jsx@v0.1.5/mod.ts";

import { extract, install } from "https://esm.sh/@twind/core@1.1.3";
import presetTailwind from "https://esm.sh/@twind/preset-tailwind@1.1.4";

// MARK: Tailwind setup
install({
  presets: [
    presetTailwind(),
    {
      theme: {
        fontFamily: {
          sans: ["Helvetica", "sans-serif"],
          serif: ["Times", "serif"],
        },
      },
    },
  ],
});

// MARK: Setup

const kv = await Deno.openKv();

const LATEST_LOG_KEY = "logs/latest";

type IndexRecord = {
  size: number;
  chunks: number;
  prefix: string[];
  ttldate: number;
};

// MARK: HTTP Server Interface

const DEFAULT_KEY_TTL_MS = (1000 * 60) * 60 * 24; // default: 1 day

// The kv queue is used to notify the server that a new log has been uploaded
// via watch events.
kv.listenQueue(async (msg: IndexRecord) => {
  await kv.set([LATEST_LOG_KEY], msg, { expireIn: DEFAULT_KEY_TTL_MS });
});

export default {
  async fetch(request: Request): Promise<Response> {
    // FIXME: proper request logging!
    //console.log(request);

    // MARK: URL routing
    const url = new URL(request.url);
    const key = url.pathname.slice(1);

    if (key === "" || key === null) {
      const status = 200;
      const body = "<!doctype html>" + await ssr(app());
      return new Response(
        body,
        {
          status,
          headers: {
            "Content-Type": "text/html; charset=utf-8",
          },
        },
      );
    }

    // MARK: Log upload
    if (key.startsWith("v1/upload")) {
      if (request.method !== "PUT") {
        return new Response("Method not allowed", { status: 405 });
      }

      if (request.body === null) {
        return new Response("Invalid request", { status: 400 });
      }

      const uuid = request.headers.get("x-uuid");
      const logType = request.headers.get("x-type");
      const logFormat = request.headers.get("x-format");
      const ts = request.headers.get("x-timestamp");
      if (
        uuid === null || ts === null || logType === null || logFormat === null
      ) {
        return new Response("Missing parameter", { status: 400 });
      }

      if (logFormat !== "pb-zst") {
        return new Response("Invalid log format", { status: 400 });
      }

      const tss = ts.split("-");
      if (tss.length != 2) {
        return new Response("Invalid timestamp", { status: 400 });
      }

      const [tsDate, tsDateTs] = [tss[0], tss[1]];

      // given the above components, we have the following kv layout
      //
      // logs/$uuid                           -> [size, chunks, idx1/$date/$dateTs/$logType/$uuid]
      // idx1/$date/$dateTs/$logType/$uuid/1  -> binary log data
      // idx1/$date/$dateTs/$logType/$uuid/2  -> binary log data
      // idx1/$date/$dateTs/$logType/$uuid/3  -> binary log data
      // ...
      //

      const kkid = ["logs", uuid];
      const kkprefix = ["logs", "idx1", tsDate, tsDateTs, logType, uuid];
      const stream = request.body;

      // now, read the stream in chunks and write them to the db
      let idx = 1;
      let size = 0;

      console.log("Uploading log", uuid, "with format", logFormat);
      for await (const chunk of stream) {
        const expireIn = DEFAULT_KEY_TTL_MS * 2; // XXX FIXME (aseipp): explain
        const res = await kv.set([...kkprefix, idx.toString()], chunk, {
          expireIn,
        });
        if (!res.ok) {
          return new Response("Internal error: failed to write log", {
            status: 500,
          });
        }
        idx++;
        size += chunk.length;
      }

      console.log(
        "Uploaded log",
        uuid,
        "of size",
        size,
        "with",
        idx - 1,
        "chunks",
      );

      // finally, write the log entry since we're done, and enqueue it
      const ttldate = Date.now() + DEFAULT_KEY_TTL_MS;
      const val: IndexRecord = {
        size,
        ttldate,
        chunks: idx - 1,
        prefix: kkprefix,
      };
      const res = await kv.atomic()
        .check({ key: kkid, versionstamp: null })
        .set(kkid, val, { expireIn: DEFAULT_KEY_TTL_MS })
        .commit();
      if (res.ok) {
        kv.enqueue(val);
        return new Response("OK", { status: 200 });
      } else {
        return new Response("Already exists", { status: 404 });
      }
    } else if (key.startsWith("v1/get")) {
      // MARK: Log retrieval
      const ks = key.split("/").slice(2);
      if (ks.length != 1) {
        return new Response("Invalid Request", { status: 400 });
      }

      const kk = ["logs", ks[0]];
      const obj = await kv.get(kk);
      if (obj.value === null && obj.versionstamp === null) {
        return new Response("Not found", { status: 404 });
      }

      const rec = obj.value as IndexRecord;
      const prefixstr = rec.prefix.join("/");
      console.log(prefixstr, "Log retrieval requested", rec);

      // the ttl is only the earliest expiry, so if the value is returned but
      // should have been expired, don't return it
      if (rec.ttldate < Date.now()) {
        return new Response("Not found", { status: 404 });
      }

      let size = 0;
      let idx = 1;
      // iterate the keyspace and write out each blob to a stream
      const body = new ReadableStream({
        async start(controller) {
          while (true) {
            const res = await kv.get([...rec.prefix, idx.toString()]);
            if (res.value === null && res.versionstamp === null) {
              break;
            }

            controller.enqueue(res.value as Uint8Array);
            size += (res.value as Uint8Array).length;
            idx++;
          }

          console.log(prefixstr, "Log chunks", idx - 1, "size", size);
          controller.close();
        },

        cancel() {
          console.log(prefixstr, "Log retrieval cancelled");
        },
      });

      return new Response(body, {
        headers: {
          "content-type": "application/octet-stream",
        },
      });
    } else if (key.startsWith("v1/watch")) {
      // MARK: /v1/watch SSE
      let logFormat = url.searchParams.get("fmt");
      logFormat = logFormat === null ? "text" : logFormat;
      logFormat = ["text", "html"].includes(logFormat) ? logFormat : "text";

      const headers = { "Content-Type": "text/event-stream" };
      const stream = kv.watch([[LATEST_LOG_KEY]]).getReader();
      const body = new ReadableStream({
        async start(controller) {
          while (true) {
            if ((await stream.read()).done) {
              return;
            }

            const obj = await kv.get(["logs/latest"]);
            const data = obj.value as IndexRecord;
            if (data === null) {
              continue;
            }

            if (logFormat === "html") {
              const fdata = await renderHtmlToString(
                <div>
                  <p class="">Log uploaded: {data.prefix.join("/")}</p>
                  <p class="font-bold">Size: {data.size}</p>
                </div>,
              );
              controller.enqueue(
                new TextEncoder().encode(
                  `event: logupload\ndata: ${fdata}\n\n`,
                ),
              );
            } else if (logFormat === "text") {
              const fdata = data.prefix.join("/") + " " + data.size;
              controller.enqueue(
                new TextEncoder().encode(
                  `event: logupload\ndata: ${fdata}\n\n`,
                ),
              );
            } else {
              console.log("Invalid log format", logFormat);
              controller.close();
            }
          }
        },

        cancel() {
          stream.cancel();
        },
      });
      return new Response(body, { headers });
    } else {
      // MARK: Invalid request
      return new Response("Invalid request", { status: 404 });
    }
  },
};

// MARK: SSR setup
async function ssr(ns: Node<unknown>): Promise<string> {
  const body = await renderHtmlToString(ns);
  const { html, css } = extract(body);
  return html.replace("</head>", `<style data-twind>${css}</style></head>`);
}

// MARK: HTML App
function app(): Node<unknown> {
  return (
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1, shrink-to-fit=no"
        />
        <title>Buck2 Logs</title>
      </head>
      <body class="font-sans">
        <div>
          <h2 class="text(2xl blue-500)">Log uploads</h2>
          <div
            hx-ext="sse"
            sse-connect="http://localhost:8000/v1/watch?fmt=html"
          >
            <div sse-swap="logupload" hx-swap="afterbegin">
            </div>
          </div>
        </div>
        <script
          src="https://unpkg.com/htmx.org@2.0.0"
          integrity="sha384-wS5l5IKJBvK6sPTKa2WZ1js3d947pvWXbPJ1OmWfEuxLgeHcEbjUUA5i9V5ZkpCw"
          crossorigin="anonymous"
        >
        </script>
        <script
          src="https://unpkg.com/htmx-ext-sse@2.0.0/sse.js"
          integrity="sha384-6BQ0r6BgBJ1HfQB7E0C7K6AP8k83MKLid3v0xFpx8W2e4hh4B9mCeEOmXq2XGs5P"
          crossorigin="anonymous"
        >
        </script>
      </body>
    </html>
  );
}
