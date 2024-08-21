update
    medicine_dict
set
    material_name = $1, material_alias = $2, purchasing_base = $3,
    purchasing_price = $4, selling_price = $5, traceability_code = $6,
    manufacturer = $7, storage_condition = $8, status = $9,
    update_by = $10, update_time = $11
where
    medicine_id = $12;
