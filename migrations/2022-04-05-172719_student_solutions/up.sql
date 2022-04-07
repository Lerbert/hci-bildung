-- we control this in application logic now
DROP TRIGGER sheet_changed ON sheets;
DROP FUNCTION update_changed_timestamp();

CREATE TABLE solutions (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    sheet_id uuid REFERENCES sheets ON UPDATE CASCADE ON DELETE SET NULL,
    sheet_version TIMESTAMPTZ NOT NULL,
    owner_id INTEGER NOT NULL REFERENCES users ON UPDATE CASCADE ON DELETE RESTRICT,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    changed TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    trashed TIMESTAMPTZ NULL,
    content JSONB NOT NULL,
    UNIQUE (sheet_id, sheet_version, owner_id)
);

-- delete all trashed solutions older than 30 days
CREATE FUNCTION delete_expired_solutions()
RETURNS VOID
LANGUAGE sql
AS $$
    DELETE FROM solutions
    WHERE trashed IS NOT NULL AND trashed < NOW() - INTERVAL '30 days';
$$;

-- convert all sheets to custom model

CREATE FUNCTION convert_tiptap_mark_to_custom(mark JSONB, parentNode JSONB)
RETURNS JSONB
LANGUAGE sql
AS $$
    SELECT jsonb_build_object(
        'type', mark->'type'
    ) || CASE mark->>'type'
            WHEN 'gap' THEN jsonb_build_object('solution', coalesce(parentNode->>'text', ''), 'answer', '')
            WHEN 'latex' THEN jsonb_build_object('source', coalesce(parentNode->>'text', ''))
            ELSE jsonb_build_object()
        END
$$;

CREATE FUNCTION convert_tiptap_node_to_custom(node JSONB)
RETURNS JSONB
LANGUAGE sql
AS $$
    SELECT jsonb_build_object(
        'type', node->'type',
        'content', array_to_json((
            SELECT coalesce(array_agg(convert_tiptap_node_to_custom(c)), ARRAY[]::JSONB[])
            FROM jsonb_array_elements(node->'content') as r(c)
        )),
        'marks', array_to_json((
            SELECT coalesce(array_agg(convert_tiptap_mark_to_custom(c, node)), ARRAY[]::JSONB[])
            FROM jsonb_array_elements(node->'marks') as r(c)
        ))
    ) || CASE node->>'type'
            WHEN 'audio' THEN jsonb_build_object('source', node->'attrs'->>'source', 'mimetype', node->'attrs'->>'mimetype')
            WHEN 'codeBlock' THEN jsonb_build_object('language', coalesce(node->'attrs'->>'language', 'plain'))
            WHEN 'heading' THEN jsonb_build_object('level', node->'attrs'->>'level')
            WHEN 'multipleChoiceAnswer' THEN jsonb_build_object('solution', coalesce((node->'attrs'->>'checked')::boolean, FALSE), 'answer', FALSE)
            WHEN 'text' THEN jsonb_build_object('text', node->>'text')
            ELSE jsonb_build_object()
        END
$$;

UPDATE sheets SET tiptap = convert_tiptap_node_to_custom(tiptap);
ALTER TABLE sheets RENAME COLUMN tiptap TO content;

DROP FUNCTION convert_tiptap_node_to_custom(JSONB);
DROP FUNCTION convert_tiptap_mark_to_custom(JSONB, JSONB);
