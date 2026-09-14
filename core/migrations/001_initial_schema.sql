-- Create nodes table
CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    cid TEXT NOT NULL,
    epistemic_type TEXT NOT NULL CHECK (epistemic_type IN ('observed', 'inferred', 'generated')),
    payload TEXT NOT NULL,
    author TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    parents TEXT NOT NULL DEFAULT '[]',
    labels TEXT NOT NULL DEFAULT '[]',
    signature TEXT,
    domain TEXT,
    source_uri TEXT
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_nodes_epistemic_type ON nodes(epistemic_type);
CREATE INDEX IF NOT EXISTS idx_nodes_domain ON nodes(domain);
CREATE INDEX IF NOT EXISTS idx_nodes_timestamp ON nodes(timestamp);
CREATE INDEX IF NOT EXISTS idx_nodes_author ON nodes(author);

-- Create narrative_diffs table for tracking edits
CREATE TABLE IF NOT EXISTS narrative_diffs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inference_id TEXT NOT NULL,
    old_version TEXT NOT NULL,
    new_version TEXT NOT NULL,
    changed_fields TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    editor TEXT NOT NULL,
    FOREIGN KEY (inference_id) REFERENCES nodes(id)
);

CREATE INDEX IF NOT EXISTS idx_diffs_inference_id ON narrative_diffs(inference_id);
CREATE INDEX IF NOT EXISTS idx_diffs_timestamp ON narrative_diffs(timestamp);

-- Create edges table for explicit graph traversal
CREATE TABLE IF NOT EXISTS edges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    from_id TEXT NOT NULL,
    to_id TEXT NOT NULL,
    edge_type TEXT NOT NULL CHECK (edge_type IN ('supports', 'derives', 'generates', 'references', 'contradicts', 'updates')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (from_id) REFERENCES nodes(id),
    FOREIGN KEY (to_id) REFERENCES nodes(id)
);

CREATE INDEX IF NOT EXISTS idx_edges_from ON edges(from_id);
CREATE INDEX IF NOT EXISTS idx_edges_to ON edges(to_id);
CREATE INDEX IF NOT EXISTS idx_edges_type ON edges(edge_type);