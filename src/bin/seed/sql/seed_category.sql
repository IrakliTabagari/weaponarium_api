INSERT INTO categories (name, parent_id)
SELECT $1, $2
WHERE NOT EXISTS (
    SELECT 1
    FROM categories
    WHERE name = $1
      AND parent_id IS NOT DISTINCT FROM $2
);
