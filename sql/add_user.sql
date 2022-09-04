INSERT INTO company (id, name, age)
VALUES ($1, $2, $3)
RETURNING $table_fields;
