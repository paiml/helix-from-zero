// helix-from-zero — queries.hx
//
// 15 lesson queries laid out in order so each one can be demoed live
// in the matching video. Lesson IDs in comments map to the course
// outline (config/rde_c20_helix_from_zero.lua in course-studio).
//
// ┌──────────────────────────────────────────────────────────────────┐
// │  Map                                                              │
// │  ───                                                              │
// │  M2.1.3  AddN, AddE, RETURN  — addUser, addKnows, getUser         │
// │  M3.1.1  helix init layout  — listSchema (smoke test)             │
// │  M3.1.2  helix check        — typedReturnShape                    │
// │  M3.1.3  helix push dev     — countUsers                          │
// │  M4.1.1  Out, In, WHERE     — youngFriends, friendsOfFriends      │
// │  M4.1.2  SearchV            — searchEmbedding                     │
// │  M4.1.3  ShortestPath       — routeDijkstra, routeBFS             │
// │  M5.1.1  Doc → Embedding    — addDocAndEmbedding, hitToDocs       │
// │  M5.1.2  BM25 + vector      — hybridSearch                        │
// │  M5.1.3  Rust client demo   — listTopFilms                        │
// └──────────────────────────────────────────────────────────────────┘

// ===========================================================================
// M2.1.3 — AddN, AddE, RETURN
// ===========================================================================

QUERY addUser(name: String, age: U32, email: String) =>
    user <- AddN<User>({name: name, age: age, email: email})
    RETURN user

QUERY addKnows(from_id: ID, to_id: ID, since: I32) =>
    a <- N<User>(from_id)
    b <- N<User>(to_id)
    edge <- AddE<Knows>({since: since})::From(a)::To(b)
    RETURN edge

QUERY getUser(user_name: String) =>
    user <- N<User>({name: user_name})
    RETURN user

// ===========================================================================
// M3.1.1 — helix init layout (smoke test that schema is reachable)
// ===========================================================================

QUERY listSchema() =>
    users <- N<User>
    RETURN users

// ===========================================================================
// M3.1.2 — helix check (typed return shape)
// ===========================================================================

QUERY typedReturnShape(user_name: String) =>
    user <- N<User>({name: user_name})
    RETURN user::{name, age, email}

// ===========================================================================
// M3.1.3 — helix push dev (counter that proves the dev instance is live)
// ===========================================================================

QUERY countUsers() =>
    users <- N<User>
    RETURN users::COUNT

// ===========================================================================
// M4.1.1 — Out, In, WHERE, ORDER<Asc>, RANGE
// ===========================================================================

QUERY youngFriends(user_name: String, max_age: U32, limit: I64) =>
    user <- N<User>({name: user_name})
    friends <- user::Out<Knows>::WHERE(_::{age}::LT(max_age))
        ::ORDER<Asc>(_::{age})
        ::RANGE(0, limit)
    RETURN friends

QUERY friendsOfFriends(user_name: String) =>
    user <- N<User>({name: user_name})
    fof <- user::Out<Knows>::Out<Knows>
    RETURN fof

// ===========================================================================
// M4.1.2 — SearchV (top-k vector similarity)
// ===========================================================================

QUERY searchEmbedding(query_vec: [F64], k: I64) =>
    hits <- SearchV<Embedding>(query_vec, k)
    RETURN hits::{chunk, chunk_index, model}

// ===========================================================================
// M4.1.3 — ShortestPathDijkstras + ShortestPathBFS
// ===========================================================================

QUERY routeDijkstra(start: ID, end: ID) =>
    path <- N<Location>(start)
        ::ShortestPathDijkstras<Route>(_::{distance})
        ::To(end)
    RETURN path

QUERY routeBFS(start: ID, end: ID) =>
    path <- N<Location>(start)::ShortestPathBFS<Route>::To(end)
    RETURN path

QUERY routeWeightedDecay(start: ID, end: ID) =>
    // Weight = distance * 0.95^(days_since_update / 30)
    // Prefers fresher routes without dropping the distance signal.
    path <- N<Location>(start)
        ::ShortestPathDijkstras<Route>(
            MUL(_::{distance}, POW(0.95, DIV(_::{days_since_update}, 30)))
        )
        ::To(end)
    RETURN path

// ===========================================================================
// M5.1.1 — Doc → Embedding → Edge (the canonical RAG schema in motion)
// ===========================================================================

QUERY addDocAndEmbedding(
    title: String,
    content: String,
    source: String,
    chunk: String,
    chunk_index: I32,
    vec: [F64]
) =>
    doc <- AddN<Doc>({title: title, content: content, source: source})
    emb <- AddV<Embedding>(vec, {
        chunk: chunk,
        chunk_index: chunk_index,
        model: "mock-384"
    })
    AddE<EmbeddingOf>::From(doc)::To(emb)
    RETURN doc

QUERY hitToDocs(query_vec: [F64], k: I64) =>
    hits <- SearchV<Embedding>(query_vec, k)
    docs <- hits::In<EmbeddingOf>
    RETURN docs::{title, content, source}

// ===========================================================================
// M5.1.2 — Hybrid search: vector top-k narrowed by title prefix
// ===========================================================================
//
// Note: real BM25 + vector RRF runs through the helix_engine reranker,
// which composes BM25 and SearchV at the engine layer rather than in
// HelixQL. The query below shows the HelixQL-side fan-out: vector top-k
// followed by graph-walk back to docs and a title-keyword filter, which
// is the user-visible API the course demos against.

QUERY hybridSearch(query_vec: [F64], title_prefix: String, k: I64) =>
    hits <- SearchV<Embedding>(query_vec, k)
    candidates <- hits::In<EmbeddingOf>
    matches <- candidates::WHERE(_::{title}::EQ(title_prefix))
    RETURN matches::{title, content, source}

// ===========================================================================
// M5.1.3 — Rust client demo
// ===========================================================================

QUERY listTopFilms(genre: String, limit: I64) =>
    films <- N<Doc>::WHERE(_::{source}::EQ(genre))
        ::ORDER<Asc>(_::{title})
        ::RANGE(0, limit)
    RETURN films::{title, content, source}
