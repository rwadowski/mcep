use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::data::ToByteUnit;
use database::establish_connection;
use definition::schema::app_definitions::dsl::app_definitions;

pub fn delete_definition(id: i32) -> Result<(), String> {
    use definition::schema::app_definitions;

    let _ = diesel::delete(app_definitions.filter(app_definitions::id.eq(id))).execute(&mut establish_connection());
    Ok(())
}