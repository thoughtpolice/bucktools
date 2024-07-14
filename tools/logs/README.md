# logging tools

These are tools for dealing with Buck2 logs, located under `buck-out/v2/log`.

Notably, there is a Deno server that can have log files uploaded into it and
serve them back later, as well as a tool for watching `buck-out/` directories
and uploading logs on demand.

## `upload` tool

TBD.

## `server` tool

TBD.

## using it with `buck2`

It doesn't work with upstream yet.

The server exposes an endpoint `/v1/get/$UUID` that allows you to download a log
file. You can then put it in a `.pb.zst` file under `buck-out/v2/log` and run
`buck2 log --trace-id $UUID` in order to view it.

To make this transparent, you need to make a small patch to Buck2
[somewhere around here](https://github.com/facebook/buck2/blob/main/app/buck2_client/src/commands/log/options.rs#L93)
in order to download logs from a URL when you run `buck2 log --trace-id ...`.
Hopefully this can be upstreamed somehow.
