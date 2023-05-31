-- Your SQL goes here

-- 例子表 
create table example (
    id INT primary key     not null,
    name           TEXT    not null,
    age            INT     not null,
    address        CHAR(50),
    salary         REAL
);

-- 用户的 配置表
create table config
(
    id   INTEGER not null
        primary key autoincrement,
    key  TEXT    not null
        unique,
    json TEXT    not null
);

-- 用户的 界面状态表
-- 用于存储 App的各种各样的界面状态,例如 todo的顺序 , 列表的顺序 , 数据图 , 退出时的路径, 
create table state
(
    id   INTEGER not null
        primary key autoincrement,
    key  TEXT    not null
        unique,
    json TEXT    not null
);

-- 暂时todo表, 未来十有八九会发生变化
create table todo
(
    id          integer            not null
        primary key autoincrement,
    "is"        blob/*整错类型了,应该是bool*/ default false not null,-- 是否完成todo
    title       text               not null,-- 标题
    create_time text               not null,-- 创建时间
    done_time   text                        -- 完成时间
);


