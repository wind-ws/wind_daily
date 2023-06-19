//! 实现 数据库中的json迁移

// todo wait 需要时去实现

use crate::other::user::user_db::UserDb;

use super::*;

/// SQL中的Text类型的 迁移包装类型, 服务Json
struct SqlJson<D>(D)
where D:Sized+
    MigData+
    MigUp+
    MigDown+
    MigGetSelf;

impl<D> Mig<D> for SqlJson<D>
where
    D:Sized+
        MigData+
        MigUp+
        MigDown+
        MigGetSelf {}

impl<D> MigVersion for SqlJson<D>
where
    D:Sized+
        MigData+
        MigUp+
        MigDown+
        MigGetSelf {
    fn now_version()->VersionMarkType {
        let db = UserDb.get_db().unwrap();

        todo!()
    }
}

// todo 为 SqlJson 实现 FromSql 和 ToSql

/// 获取 key ,用于 mig_json_version 表中的匹配
/// 被包装类型实现, 若是 更新或回溯关系 则要求相同key
trait SqlMigJsonKey{
    fn key()->&'static str;
}
