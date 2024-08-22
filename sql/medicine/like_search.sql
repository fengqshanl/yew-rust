SELECT * FROM medicine_dict WHERE status = 'NORMAL' and delete_tag = 0 and traceability_code LIKE $1
UNION
SELECT * FROM medicine_dict WHERE status = 'NORMAL' and delete_tag = 0 and material_name LIKE $2;
