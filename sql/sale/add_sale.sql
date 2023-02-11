INSERT INTO sale ( drug_id, name, number, money, total, sale_time, code, spec, manu_address)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING $table_fields;