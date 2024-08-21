select * from system_dict where dict_status = 0 and dict_type = $1 order by dict_order asc;
