-- INSERT INTO drug (name, drug_id, drug_number, ingredient, character, major_function, specification, usage_dosage, adverse_reaction, taboo, matters_need_attention, store_up, expiry_date, produced_time, approval_number, manufacturing_enterprise)
-- VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
-- RETURNING $table_fields;

INSERT INTO drug (code, sptm_img, img, goods_type, trademark, goods_name, spec, note, price, ycg, manu_name, manu_address, qs, nw, description, gw, width, height, depth, gpc, gpc_type, keyword, img_list)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
RETURNING $table_fields;


    -- "code": "69********432", // 条形码
    -- "sptmImg": "", // 条码图片
    -- "img": "", // 图片
    -- "goodsType": "造纸原料和纸制品>>纸制品>>个人纸制品>>纸巾", // 商品分类
    -- "trademark": "清风", // 品牌 
    -- "goodsName": "清风原木纯品纸手帕", // 商品名称
    -- "spec": "迷你型", // 规格
    -- "note": "备注：经查，该厂商识别代码已在中国物品编码中心注册，但编码信息未按规定通报", // 备注信息
    -- "price": "", // 参考价格(单位:元)
    -- "ycg": "", // 原产地(可能无此参数信息)
    -- "manuName": "金红叶纸业集团有限公司", // 厂商
    -- "manuAddress": "江苏省苏州市苏州工业园区金胜路1号", //  厂商地址
    -- "qs": "",//生产许可证号
    -- "nw": "",//净重
    -- "description": "",//形态描述
    -- "gw": "",//毛重
    -- "width": "",//宽
    -- "hight": "",//高
    -- "depth": "",//深
    -- "gpc": "",//gpc分类代码
    -- "gpcType": "",//gpc分类名称    
    -- "keyword": "",//关键词
    -- "imgList": [] //