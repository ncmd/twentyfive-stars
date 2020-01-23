use crate::database::schema::waves;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, juniper::GraphQLObject)]
pub struct Wave {
  pub id: Uuid,
  pub tcg_id: String,
  pub name: String,
  pub released: chrono::NaiveDate,
}
