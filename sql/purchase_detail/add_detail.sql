INSERT INTO purchase_detail0 (purchase, drug_id, number, name, sale_money, self_money)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING $table_fields;