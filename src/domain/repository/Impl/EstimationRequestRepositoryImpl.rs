use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::EstimateRequest::dsl::*;

/** コマンド構造体 */


/** クエリ構造体 */

pub struct EstimationRequestRepositoryImpl {}

impl EstimationRequestRepository for EstimationRequestRepositoryImpl {}