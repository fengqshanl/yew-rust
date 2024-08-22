update
    medicine_dict
set
    delete_by = $1, delete_time = $2, delete_tag = 1
where
    medicine_id = $3;
