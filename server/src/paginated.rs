use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use uuid::Uuid;

pub trait Paginate: Sized {
  fn paginate(self) -> Paginated<Self>;
}

impl<T> Paginate for T {
  fn paginate(self) -> Paginated<Self> {
    Paginated {
      query: self,
      limit: None,
      after_cursor: None,
      before_cursor: None,
    }
  }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
  query: T,
  limit: Option<i64>,
  after_cursor: Option<Uuid>,
  before_cursor: Option<Uuid>,
}

impl<T> Paginated<T> {
  pub fn limit(self, limit: i64) -> Self {
    Paginated {
      limit: Some(limit),
      ..self
    }
  }

  pub fn after(self, after_cursor: Uuid) -> Self {
    Paginated {
      after_cursor: Some(after_cursor),
      ..self
    }
  }

  pub fn before(self, before_cursor: Uuid) -> Self {
    Paginated {
      before_cursor: Some(before_cursor),
      ..self
    }
  }
}

impl<T: Query> Query for Paginated<T> {
  type SqlType = T::SqlType;
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
  T: QueryFragment<Pg>,
{
  fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
    // out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
    self.query.walk_ast(out.reborrow())?;
    // out.push_sql(") t LIMIT ");
    // out.push_bind_param::<BigInt, _>(&self.per_page)?;
    // out.push_sql(" OFFSET ");
    // let offset = (self.page - 1) * self.per_page;
    // out.push_bind_param::<BigInt, _>(&offset)?;
    Ok(())
  }
}
