import { debounce } from "jsr:@std/async/debounce";
import { parseArgs } from "jsr:@std/cli/parse-args";
import { exists } from "jsr:@std/fs/exists";
import { toReadableStream } from "jsr:@std/io/to-readable-stream";

const flags = parseArgs(Deno.args, {
  default: { "host": "http://localhost:8000" },
  string: ["host"],
  boolean: ["help"],
});

let watchDir = String(flags._[0] || ".");
if (!(await exists(watchDir))) {
  console.error("Directory does not exist:", watchDir);
  Deno.exit(1);
} else {
  // go ahead and create buck-out/v2/log immediately if it isn't there and begin
  // watching that so we can pick up logs as soon as possible
  watchDir = `${watchDir}/buck-out/v2/log`;
  if (await exists(watchDir)) {
    console.log("Found buck-out/ log directory:", watchDir);
  } else {
    console.log("Creating buck-out/ log directory:", watchDir);
    await Deno.mkdir(watchDir, { recursive: true });
  }
}

console.log("Uploading logs to", flags.host);

const uploadLog = debounce(async (event: Deno.FsEvent) => {
  const file = event.paths[0];
  const logFormat = "pb-zst"; // FIXME (aseipp): this should be configurable
  if (!file.endsWith("_events.pb.zst")) {
    return;
  }

  const basename = file.split("/").pop();
  if (basename === undefined) {
    return;
  }

  if (event.kind === "access") {
    // log path looks like: 20240622-211843_build_07d6980a-fb21-4073-8d98-e72f876cdb16_events.pb.zst
    // we want to parse out these parts:
    // - 20240622-211843 (date)
    // - build (type)
    // - 07d6980a-fb21-4073-8d98-e72f876cdb16 (uuid)
    const parts = basename?.split("_");
    const timestamp = parts[0];
    const logType = parts[1];
    const uuid = parts[2];

    const f = await Deno.open(file);
    const sz = (await f.stat()).size;

    console.log("Log file discovered:", sz, timestamp, logType, uuid);
    const req = new Request(`${flags.host}/v1/upload`, {
      method: "PUT",
      headers: {
        "content-type": "application/octet-stream",
        "content-length": `${sz}`,

        "x-timestamp": timestamp,
        "x-type": logType,
        "x-uuid": uuid,
        "x-format": logFormat,
      },
      body: toReadableStream(f),
    });

    const resp = await fetch(req);
    const respBody = await resp.text();
    console.log("Upload response:", resp.status, respBody);
  }
}, 200);

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

let watcher = Deno.watchFs(watchDir);
do {
  console.log("Watching directory", watchDir);
  for await (const event of watcher) {
    if (event.kind === "remove" && event.paths[0] === watchDir) {
      console.log("Directory removed, restarting watcher...");

      // loop until the directory is recreated
      let backoff = 5; // milliseconds, exponential
      while (!(await exists(watchDir))) {
        await sleep(backoff);
        backoff *= 2;
        if (backoff > 500) {
          backoff = 500;
        }
      }

      watcher = Deno.watchFs(watchDir);
      break; // to do-while
    }

    uploadLog(event);
  }
} while (true);
