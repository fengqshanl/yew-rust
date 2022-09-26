-- INSERT INTO drug (name, drug_id, drug_number, ingredient, character, major_function, specification, usage_dosage, adverse_reaction, taboo, matters_need_attention, store_up, expiry_date, produced_time, approval_number, manufacturing_enterprise)
-- VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
-- RETURNING $table_fields;

INSERT INTO drug (name, drug_id, drug_number, usage_dosage, matters_need_attention, a_b_classify)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING $table_fields;