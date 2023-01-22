INSERT INTO purchase (per_id, money, kind, in_time)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;