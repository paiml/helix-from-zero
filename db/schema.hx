// helix-from-zero — schema.hx
//
// Three schemas, one file. Each schema is referenced by name from
// queries.hx. Field types and INDEX hints come straight out of the
// HelixQL surface that the course teaches.
//
// M2 — Users + Knows (the simplest possible graph for the AddN/AddE
//   walkthrough; intentionally tiny so the lesson can fit in 6 minutes)
// M4 — Locations + Route (weighted graph for ShortestPathDijkstras +
//   ShortestPathBFS; route Properties carry the cost terms)
// M5 — Doc + Embedding + EmbeddingOf (the canonical RAG schema —
//   document holds the source text, vector holds the embedding,
//   the typed edge connects them so SearchV hits can walk back to
//   their docs)

// ===========================================================================
// M2: Users + Knows — the simplest possible graph
// ===========================================================================

N::User {
    INDEX name: String,
    age: U32,
    email: String DEFAULT "unknown@example.com"
}

E::Knows {
    From: User,
    To: User,
    Properties: {
        since: I32
    }
}

// ===========================================================================
// M4: Locations + Routes — weighted graph for Dijkstra and BFS demos
// ===========================================================================

N::Location {
    INDEX name: String,
    traffic_factor: F64 DEFAULT 1.0,
    popularity: F64 DEFAULT 0.5
}

E::Route {
    From: Location,
    To: Location,
    Properties: {
        distance: F64,
        days_since_update: F64,
        bandwidth: F64,
        reliability: F64
    }
}

// ===========================================================================
// M5: Doc + Embedding + EmbeddingOf — the canonical RAG schema
// ===========================================================================

N::Doc {
    INDEX title: String,
    content: String,
    source: String DEFAULT "unknown"
}

V::Embedding {
    chunk: String,
    chunk_index: I32,
    model: String DEFAULT "mock-384"
}

E::EmbeddingOf {
    From: Doc,
    To: Embedding
}
