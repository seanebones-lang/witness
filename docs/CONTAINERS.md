# Container deployment

Witness is a local provenance prototype. Its write APIs provide **no
authentication or authorization** (see [INSTALL.md](../INSTALL.md)), so the
deployment question is not "how do we expose it" but "how do we run it
inside a boundary that matches that fact". This recipe runs the API server
in a container with the smallest surface the prototype can manage:

- an unprivileged user (UID 10001) — the process is never root;
- a read-only root filesystem — the only writable state is a dedicated
  volume at `/data` holding the SQLite database;
- all Linux capabilities dropped, `no-new-privileges` set;
- the port published on the **host loopback interface only**.

The container is a deployment boundary, not access control: anyone who can
reach the port can still write records. Keep the loopback publication, or
put the service on a container network with no published port at all and
reach it from a sibling container. Do not publish the port on a LAN or
public interface until the write APIs have authentication.

## Build and run

```sh
docker compose up -d --build
curl http://127.0.0.1:8080/
```

Or without compose:

```sh
docker build -t witness:local .
docker volume create witness-data
docker run -d --name witness \
  --user 10001:10001 --read-only --cap-drop ALL \
  --security-opt no-new-privileges:true \
  --tmpfs /tmp \
  -e HOST=0.0.0.0 -e PORT=8080 -e DATABASE_URL=sqlite:///data/witness.db \
  -v witness-data:/data \
  -p 127.0.0.1:8080:8080 \
  witness:local
```

`HOST=0.0.0.0` is the bind address *inside the container's network
namespace*; it makes the server reachable through the published port and
from sibling containers, nowhere else. Host exposure is decided by the
`-p 127.0.0.1:8080:8080` publication, which is loopback-only. Running the
binary bare (no container), keep the default `HOST=127.0.0.1`.

Records persist across container replacement because the database lives on
the `witness-data` volume:

```sh
docker compose down        # the volume, and the records, remain
docker compose up -d       # the same store is reopened and re-verified
```

## Verify the boundary

```sh
docker compose exec witness id
# uid=10001(witness) gid=10001(witness) ...

docker compose exec witness sh -c 'touch /usr/local/bin/x'
# Read-only file system

docker compose exec witness sh -c 'ls /data'
# witness.db (and its -wal/-shm companions while running)
```

From the host, the API answers on loopback and is not bound on any external
interface:

```sh
curl -s http://127.0.0.1:8080/ >/dev/null && echo loopback-ok
ss -ltn | grep 8080        # shows 127.0.0.1:8080, never 0.0.0.0:8080
```

## What this does not change

- **No authentication.** The write APIs remain unauthenticated; integrity
  signatures still only make later tampering detectable. The container
  limits *where* the API can be reached from, not *who* can call it once
  reached. Authenticated write access is a product change, tracked in
  INSTALL.md's limitations, not something a deployment recipe can grant.
- **No record-level isolation.** One container holds one store. Tenant
  isolation does not exist in the prototype and this recipe does not add it.
- **Backups are the volume's job.** Snapshot or copy the `witness-data`
  volume with the container stopped (or use SQLite's online backup through
  the API host) — the recipe ships no backup tooling.
