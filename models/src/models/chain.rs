use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::errors::DBModelError;
use crate::schema::chains;
use crate::schema::chains::dsl::chains as all_chains;

#[derive(Debug, Queryable, Serialize)]
pub struct Chain {
    pub id: i32,
    pub chain_id: String,
    pub chain_name: String,
    pub icon_url: Option<String>,
    pub webisite: Option<String>,
    pub inserted_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Chain {
    pub fn query_all(conn: &PgConnection) -> Result<Vec<Chain>, DBModelError> {
        all_chains
            .order(chains::id.desc())
            .load::<Chain>(conn)
            .map_err(|e| e.into())
    }

    pub fn count_all(conn: &PgConnection) -> Result<i64, DBModelError> {
        all_chains.count().get_result(conn).map_err(|e| e.into())
    }

    pub fn find_by_chain_id(conn: &PgConnection, chain_id: String) -> Result<Chain, DBModelError> {
        all_chains
            .filter(chains::chain_id.eq(chain_id))
            .first::<Chain>(conn)
            .map_err(|e| e.into())
    }
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "chains"]
pub struct NewChain {
    pub chain_id: String,
    pub chain_name: String,
    pub inserted_at: NaiveDateTime,
}

impl NewChain {
    pub fn insert(new_chain: &NewChain, conn: &PgConnection) -> Result<usize, DBModelError> {
        diesel::insert_into(chains::table)
            .values(new_chain)
            .execute(conn)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::DBConfig,
        db::{BackendDB, Database},
        models::test_helpers::cleanup_db,
    };
    use chrono::Utc;
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn test_insert() {
        let mut db = BackendDB::new(DBConfig::default());
        let result = db.connect();
        assert_eq!(result, true);

        cleanup_db(&db.conn().unwrap());

        let data = NewChain {
            chain_id: "gaia".to_string(),
            chain_name: "gaia".to_string(),
            inserted_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        };

        let result = NewChain::insert(&data, &db.conn().unwrap()).unwrap();
        assert_eq!(result, 1);

        let count = Chain::count_all(&db.conn().unwrap()).unwrap();
        assert_eq!(count, 1i64);

        // duplicate chain info should occur errors.
        let result = NewChain::insert(&data, &db.conn().unwrap());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    #[serial]
    fn test_query() {
        let mut db = BackendDB::new(DBConfig::default());
        let result = db.connect();
        assert_eq!(result, true);

        cleanup_db(&db.conn().unwrap());

        let data = NewChain {
            chain_id: "gaia".to_string(),
            chain_name: "gaia".to_string(),
            inserted_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        };

        let result = NewChain::insert(&data, &db.conn().unwrap()).unwrap();
        assert_eq!(result, 1);

        let result = Chain::query_all(&db.conn().unwrap()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].chain_id, "gaia".to_string());
        assert_eq!(result[0].chain_name, "gaia".to_string());
        assert_eq!(result[0].updated_at, None);
    }
}
