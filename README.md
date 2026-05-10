# helix-from-zero

Companion repo for **HelixDB from Zero**, course 20 in the Coursera **Rust for Data Engineering** specialization (Pragmatic AI Labs · Noah Gift). Five modules walk a Rust-fluent learner from `helix init` through writing a first `.hx` schema, deploying it locally, and finally calling typed HelixQL queries from a static Rust binary — all against the same project you'll see in the videos.

## What HelixDB is

[HelixDB](https://github.com/HelixDB/helix-db) is an open-source graph + vector database written in Rust (AGPL-3.0, LMDB-backed). It collapses the typical RAG stack — Postgres for relational rows, Qdrant or Pinecone for vectors, Neo4j or pgvector for the graph — into one engine with one query language ([HelixQL](https://docs.helix-db.com/documentation/hql/hql)).

The course is the from-zero counterpart: you write the `.hx` files, run them through `helix check`, push to a local instance with `helix push dev`, and watch the typed Rust client return the rows you asked for.

## Project layout

```
helix-from-zero/
├── helix.toml             # HelixDB project config (name + queries dir + local.dev port)
├── db/
│   ├── schema.hx          # N::User, V::Embedding, E::EmbeddingOf, ... (3 schemas, 1 file)
│   └── queries.hx         # 15 lesson queries, one per video in the course
├── assets/                # hero.svg + hero.png will land here in iteration 4
└── README.md
```

The 15 lesson queries in `db/queries.hx` are commented with their lesson IDs so each video has a target it can demo against in real time.

## Lesson map

| Module | Lesson | Query in queries.hx | What it shows |
| --- | --- | --- | --- |
| **M1 — Why HelixDB** | 1.1.1, 1.1.2, 1.1.3 | (conceptual — no live demo) | One engine for graph + vector + KV; the Helix stack; vs the Postgres + Qdrant + Neo4j stack |
| **M2 — Schema + first queries** | 2.1.1, 2.1.2 | (schema-only) | `N::`, `V::`, `E::` shape; `INDEX`, field types, `DEFAULT` |
| | 2.1.3 | `addUser`, `addKnows`, `getUser` | First QUERY — `AddN`, `AddE`, `RETURN` |
| **M3 — helix-cli** | 3.1.1 | `listSchema` | `helix init` and the project layout (this repo) |
| | 3.1.2 | `typedReturnShape` | `helix check` and `helix compile` — type-safe pre-deploy |
| | 3.1.3 | `countUsers` | `helix push dev` — local instance and HTTP endpoints |
| **M4 — Graph + vector** | 4.1.1 | `youngFriends`, `friendsOfFriends` | `Out`, `In`, `WHERE`, `ORDER<Asc>`, `RANGE` |
| | 4.1.2 | `searchEmbedding` | `SearchV` — top-k vector similarity |
| | 4.1.3 | `routeDijkstra`, `routeBFS`, `routeWeightedDecay` | `ShortestPathDijkstras` + `ShortestPathBFS` with weight expressions |
| **M5 — Hybrid RAG + Rust client** | 5.1.1 | `addDocAndEmbedding`, `hitToDocs` | `Doc` → `Embedding` → `Edge` traversal |
| | 5.1.2 | `hybridSearch` | BM25 + vector fusion via the built-in reranker |
| | 5.1.3 | `listTopFilms` | Typed Rust client — Helix queries from a static binary |

## Running it locally

You need the [Helix CLI](https://github.com/HelixDB/helix-db) installed:

```bash
curl -sSL "https://install.helix-db.com" | bash
```

Then, from the repo root:

```bash
helix check                # validate schema.hx + queries.hx (M3.1.2)
helix compile              # compile the queries to a workspace artifact
helix push dev             # deploy to a local instance on port 6969 (M3.1.3)
```

Every `QUERY name(args) =>` in `db/queries.hx` becomes an HTTP endpoint at `http://localhost:6969/<name>`. The `Makefile` wraps these as `make check`, `make compile`, `make push` once the CLI is on PATH.

## Schema in one paragraph

`db/schema.hx` carries three independent sub-schemas so the lessons stay tight:

* **Users + Knows** for the M2 AddN/AddE walkthrough — the smallest possible graph that's still recognisable as a graph.
* **Locations + Routes** for the M4 Dijkstra/BFS demos — `Route` carries `distance`, `days_since_update`, `bandwidth`, and `reliability`, which is exactly enough surface to compose weighted shortest-path queries against.
* **Doc + Embedding + EmbeddingOf** for the M5 RAG demos — the canonical pattern where a `SearchV` hit walks back to its source document via the typed edge.

## License

Dual-licensed under MIT or Apache-2.0. HelixDB itself is AGPL-3.0; this companion repo links no Helix code, only its CLI and HTTP API, so the MIT/Apache choice on this code stays clean.

## Credit

Course author: [Noah Gift](https://github.com/noahgift) · [Pragmatic AI Labs](https://paiml.com).
HelixDB is built by the [HelixDB team](https://github.com/HelixDB).
