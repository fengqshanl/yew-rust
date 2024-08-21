INSERT INTO medicine_dict (
    material_name, material_alias, purchasing_base,
    purchasing_price, selling_price, traceability_code,
    manufacturer, storage_condition, status,
    create_by, create_time
) VALUES (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
);
