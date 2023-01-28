INSERT INTO sale ( drug_id, drug_name, sale_number, money, total, sale_time)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING $table_fields;