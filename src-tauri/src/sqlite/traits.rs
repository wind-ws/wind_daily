// use diesel::{r2d2::ConnectionManager, SqliteConnection, Insertable, QuerySource, Table, query_builder::IncompleteInsertStatement, RunQueryDsl, Connection, query_dsl::methods::ExecuteDsl, sqlite::Sqlite};
// use r2d2::PooledConnection;

// use crate::other::user::user_db::UserDb;



// pub trait GetDb {
//     fn get_sqlite_db()->PooledConnection<ConnectionManager<SqliteConnection>>{
//         UserDb.get_db().unwrap()
//     }
// }
// pub trait GetTable<T:Table>{
//     fn get_table()->T;
// }

// pub trait Insert<T,Q,Conn>
// where
//     Self :  GetDb + 
//             GetTable<T> + 
//             Insertable<Q> + 
//             ExecuteDsl<Conn> + 
//             Sized,
//     Q : QuerySource,
//     T : Table,
//     Conn: Connection<Backend= Sqlite>{

//     fn insert(self) where Self: Insertable<T>{
//         diesel::insert_into(Self::get_table())
//             .values(self)
//             .execute(&mut UserDb.get_db().unwrap())
//             .unwrap();
//     }
// }

// todo 有时间在看吧
