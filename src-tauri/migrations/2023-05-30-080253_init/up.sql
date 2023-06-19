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

-- json结构 版本表
create table mig_json_version (
    id      integer     not null primary key autoincrement,
    key     text        not null unique ,
    version integer     not null
);

-- 用户的 配置表
create table config (
    id   integer not null primary key autoincrement,
    key  text    not null unique,
    json text    not null
);

-- 用户的 界面状态表
-- 用于存储 App的各种各样的界面状态,例如 todo的顺序 , 列表的顺序 , 数据图 , 退出时的路径, 
create table state (
    id   integer not null primary key autoincrement,
    key  text    not null unique,
    json text    not null
);


-- |todo的根表 
create table todo (
    id          integer                 not null primary key autoincrement,
    
    "is"        boolean default false   not null,-- 是否完成todo
    is_visible  boolean default true    not null,-- 是否可见
    
    title       text                    not null,-- 标题
    priority    text                    
        check(priority in (
                '"Indifferent"', --无所谓啦级(灰色),做不做都可以或者说想做就做
                '"Ordinary"',    --一般般啦级(绿色),有资源(时间)在做
                '"Normal"',      --正常级(蓝色),需要完成,
                    --Normal优先Indifferent & Ordinary,即建议完成所有Normal后在进行优先级更低的
                    --Normal被优先Urgent & Haunted ,即建议 所有被优先 级 完成后在进行Normal级
                '"Urgent"',   --紧急级(黄色),放下爱情和理想,赶快去完成这个todo
                '"Haunted"'   --要命级(红色),当它出现后,会派出一只鬼去追杀你,直到你完成todo
            )
        )
        default 'Normal'                not null,-- 优先级
    father_id   integer                         ,-- 标记父亲id,若存在则说明它是孩子,null则没有父亲
    
    remind_time timestamp                       ,-- 提醒时间,提醒你需要做它啦~
    create_time timestamp               not null,-- 创建时间
    done_time   timestamp                        -- 完成时间
);



-- 任务表
-- 任务可以包含多个todo
-- 任务和todo相比,
--     待办(todo) 只是提醒你需要做的事,比如: 明天要 买胡萝卜,提醒收衣服
--     而任务可以是, 明天要买[胡萝卜,酱油,...](一个一个完成,虽然todo的父子关系也可以达到这个效果)
--     但是任务可以周期性生成todo 和更加丰富的数据表示
create table task (-- unstable
    id          integer                 not null primary key autoincrement

);


-- 习惯表
-- 用于记录养成习惯,和坚持的习惯或行为
-- 比如: 减肥(坚持运动),健康(每天一苹果,医生远离我),js(编程语言)
-- 你应该用它来记录你全部的 习惯 ,无论好坏,无论是否需要坚持
create table habit (-- unstable
    id          integer                 not null primary key autoincrement

);


-- 计划表
-- 一个极度全面的数据表, 包含 todo,task,habit,...
create table plan (-- unstable
    id          integer                 not null primary key autoincrement
    
);

