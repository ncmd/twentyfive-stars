use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, StratagemCard, ID};
use crate::database_schema::cards_with_pageinfo;

#[derive(Identifiable, Queryable, Clone, PartialEq, Eq, Debug)]
#[table_name = "cards_with_pageinfo"]
pub struct Card {
  pub id: ID,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: ID,
  pub sort_order: i32,
  pub has_previous: bool,
  pub has_next: bool,
}

pub mod schema {
  use super::*;
  // use async_graphql::FieldResult;

  #[async_graphql::Interface(
    field(name = "id", type = "ID"),
    field(name = "tcg_id", type = "&str"),
    field(name = "rarity", type = "CardRarity"),
    field(name = "number", type = "&str"),
    field(name = "category", type = "CardCategory"),
    // TODO: enable once FieldResult is fixed
    // field(name = "wave", type = "FieldResult<Wave>", context)
  )]
  #[derive(Clone, Debug)]
  pub struct Card(BattleCard, CharacterCard, StratagemCard);
}
