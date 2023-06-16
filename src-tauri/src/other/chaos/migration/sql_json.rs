//! 实现 数据库中的json迁移


use super::*;


struct SqlJson<D:MigVersionData>(D);


