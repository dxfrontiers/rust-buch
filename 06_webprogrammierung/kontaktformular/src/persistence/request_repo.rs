use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::model::{ContactRequest, NewContactRequest};
use crate::persistence::DbCon;
use crate::schema::requests;
use diesel::connection::Connection;
use diesel::result::Error;
use diesel::QueryResult;
use diesel::{self, prelude::*};
use crate::schema::requests::dsl::{requests as all_requests};


/**
    Returns all ContactRequests in the database.
    If there is a Database Error of any kind, we map it to its string representation
 */
pub async fn get_all(con: &DbCon) -> Result<Vec<ContactRequest>, Error> {
    let result = con.run(|c| {
        requests::table
            .load::<ContactRequest>(c)
    }).await;
    return result;
}

/**

*/
pub async fn insert(con: &DbCon, new_request: NewContactRequest) -> Result<ContactRequest, String> {
    use diesel::prelude::*;
    use diesel::select;

    let result = con.run(move |c| {
        /*
            We explicitly start a transaction here, since SQLite does not support returning the inserted
            elements. On a real database, you can use `get_result` (commented out below) to get a Result of type
            `Result<ContactRequest, Error>` here.
         */
        let query_result = c.transaction::<_, Error, _>(|| {
            diesel::insert_into(requests::table)
                .values(&new_request)
                //.get_result(&con.0);
                .execute(c)?;

            requests::table
                .order(requests::id.desc())
                .limit(1)
                .load(c)?
                .into_iter()
                .rev()
                .next()
                .ok_or(Error::NotFound)
        });
        query_result.map_err(|e| e.to_string())
    }).await;
    return result;
}

/* // Would we only pg, the obove function could be condensed to this
pub async fn insert_pg_only(con: &DbCon, new_request: NewContactRequest) -> Result<ContactRequest, String> {
    use diesel::prelude::*;
    use diesel::select;
    con.run(move |c| {
        diesel::insert_into(requests::table)
            .values(&new_request)
            .get_result(c)
            .map_err(|e| e.to_string())
    }).await
}
*/

/**

*/
pub async fn get_by_id(con: &DbCon, request_id: i32) -> Result<Option<ContactRequest>,String> {
    con.run(move |c| {
        requests::table
            .find(request_id)
            .first(c)
            .map(|x| Some(x))         // TODO Das ergibt so keinen sinn
            .map_err(|e| e.to_string())
    }).await
}

/**

*/
pub async fn delete_by_id(con: &DbCon, request_id: i32) -> Result<usize, String> {
    let result = con.run(move |c| {
        diesel::delete(requests::table.find(request_id))
            .execute(c)
            .map(|num| num)
            .map_err(|e| e.to_string())
    }).await;
    return result;

}

/**

*/
pub async fn update(con: &DbCon, id: i32, updated_request: ContactRequest) -> Result<ContactRequest, String> {
    if updated_request.id != id {
        return Err("The id of a request cannot be changed".to_string())
    }

    /*
    If we used only postgres, the code of this method could be condensed to the following
    diesel::update(requests::table.find(updated_request.id))
        .set(updated_request)
        .get_result(&con.0)
        .map_err(|e| e.to_string())
     */

    use diesel::prelude::*;
    use diesel::select;

    con.run(|c| {
        /*
            We explicitly start a transaction here, since SQlite does not support returning the inserted
            elements. On a real database, you can use `get_result` (commented out below) to get a Result of type
            `Result<ContactRequest, Error>` here.
         */
        let query_result = c.transaction::<_, Error, _>(|| {
            diesel::update(requests::table.find(updated_request.id))
                .set(updated_request)
                //.get_result(&con.0);
                .execute(c)?;

            requests::table
                .order(requests::id.desc())
                .limit(1)
                .load(c)?
                .into_iter()
                .rev()
                .next()
                .ok_or(Error::NotFound)
        });
        query_result.map_err(|e| e.to_string())
    }).await
}
