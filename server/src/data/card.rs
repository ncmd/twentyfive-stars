use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, StratagemCard, Wave, ID};
use async_graphql::FieldResult;

#[async_graphql::Union]
#[derive(Clone, Debug)]
pub struct Card(BattleCard, CharacterCard, StratagemCard);
