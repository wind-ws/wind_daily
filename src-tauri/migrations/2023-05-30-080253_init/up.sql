-- Your SQL goes here

-- 例子表 
create table example (
    id          integer   not null primary key autoincrement,
    text        text      not null,
    real        real      not null,
    blob        blob      not null,
    integer     integer   not null,
    boolean     boolean   not null,
    timestamp   timestamp not null
);

-- 用户的 配置表
create table config
(
    id   integer not null primary key autoincrement,
    key  text    not null unique,
    json text    not null
);

-- 用户的 界面状态表
-- 用于存储 App的各种各样的界面状态,例如 todo的顺序 , 列表的顺序 , 数据图 , 退出时的路径, 
create table state
(
    id   integer not null primary key autoincrement,
    key  text    not null unique,
    json text    not null
);


-- |todo的根表 
create table todo
(
    id          integer                 not null primary key autoincrement,
    "is"        boolean default false   not null,-- 是否完成todo
    is_visible  boolean default true    not null,-- 是否可见
    title       text                    not null,-- 标题
    e_todo_type text    default "None"  not null,-- |todo的类型
    create_time timestamp               not null,-- 创建时间
    done_time   timestamp                        -- 完成时间
);

create table father_todo
(
    id              integer not null primary key autoincrement,
    todo_id         integer not null,-- 对应todo表
    father_todo_id  integer not null,-- 对应todo表
);