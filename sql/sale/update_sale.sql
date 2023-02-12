UPDATE sale 
SET drug_id = $1, name = $2, number = $3, money = $4, total = $5, sale_time = $6, code = $7, spec = $8, manu_address = $9
WHERE sale_id = $10;

-- UPDATE table_name
-- SET column1 = value1, column2 = value2...., columnN = valueN
-- WHERE [condition];