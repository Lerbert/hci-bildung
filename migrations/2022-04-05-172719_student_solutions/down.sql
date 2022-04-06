-- convert custom model back to tiptap

CREATE FUNCTION convert_custom_mark_to_tiptap(mark JSONB)
RETURNS JSONB
LANGUAGE sql
AS $$
    SELECT jsonb_build_object('type', mark->'type')
$$;

CREATE FUNCTION convert_custom_node_to_tiptap(node JSONB)
RETURNS JSONB
LANGUAGE sql
AS $$
    SELECT jsonb_strip_nulls(jsonb_build_object(
        'type', node->'type',
        'content', array_to_json((
            SELECT array_agg(convert_custom_node_to_tiptap(c))
            FROM jsonb_array_elements(node->'content') as r(c)
        )),
        'marks', array_to_json((
            SELECT array_agg(convert_custom_mark_to_tiptap(c))
            FROM jsonb_array_elements(node->'marks') as r(c)
        )))
    ) || CASE node->>'type'
            WHEN 'audio' THEN jsonb_build_object('attrs', jsonb_build_object('source', node->'source', 'mimetype', node->'mimetype'))
            WHEN 'codeBlock' THEN jsonb_build_object('attrs', jsonb_build_object('language', node->'language'))
            WHEN 'heading' THEN jsonb_build_object('attrs', jsonb_build_object('level', node->'level'))
            WHEN 'multipleChoiceAnswer' THEN jsonb_build_object('attrs', jsonb_build_object('checked', node->'solution'))
            WHEN 'text' THEN jsonb_build_object('text', node->>'text')
            ELSE jsonb_build_object()
        END
$$;

UPDATE sheets SET tiptap = convert_custom_node_to_tiptap(tiptap);

DROP FUNCTION convert_custom_node_to_tiptap(JSONB);
DROP FUNCTION convert_custom_mark_to_tiptap(JSONB);

DROP FUNCTION delete_expired_solutions;

DROP TABLE solutions;

CREATE FUNCTION update_changed_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.changed = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER sheet_changed BEFORE UPDATE ON sheets
FOR EACH ROW EXECUTE PROCEDURE update_changed_timestamp();
