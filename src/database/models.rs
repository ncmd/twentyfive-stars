use crate::database::schema::{battle_cards, cards, waves};
use crate::database::types::{CardCategory, CardRarity};
use crate::graphql_schema::Context;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, juniper::GraphQLObject)]
pub struct Wave {
  pub id: Uuid,
  pub tcg_id: String,
  pub name: String,
  pub released: chrono::NaiveDate,
}

// #[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
// pub struct Card {
//   pub id: Uuid,
//   pub tcg_id: String,
//   pub rarity: CardRarity,
//   pub number: String,
//   pub category: CardCategory,
//   pub wave_id: Uuid,
// }

pub trait GenericCard {
  fn id(&self) -> Uuid;

  fn wave(&self, context: &Context) -> Wave;
}

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct BattleCard {
  pub id: Uuid,
  pub card_id: Uuid,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: Uuid,
}

impl GenericCard for BattleCard {
  fn id(&self) -> Uuid {
    self.id
  }

  fn wave(&self, context: &Context) -> Wave {
    waves::table
      .inner_join(cards::table)
      .select((waves::id, waves::tcg_id, waves::name, waves::released))
      .first::<Wave>(&context.connection)
      .expect("Error loading wave")
  }
}

#[juniper::object(
  Context = Context
  interfaces = [&Card]
)]
impl BattleCard {
  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn card_id(&self) -> Uuid {
    self.card_id
  }

  pub fn tcg_id(&self) -> &str {
    self.tcg_id.as_str()
  }

  pub fn rarity(&self) -> &CardRarity {
    &self.rarity
  }

  pub fn number(&self) -> &str {
    self.number.as_str()
  }

  pub fn category(&self) -> &CardCategory {
    &self.category
  }

  pub fn wave(&self, context: &Context) -> Wave {
    waves::table
      .inner_join(cards::table)
      .select((waves::id, waves::tcg_id, waves::name, waves::released))
      .first::<Wave>(&context.connection)
      .expect("Error loading wave")
  }
}

pub enum Card {
  BattleCard(BattleCard),
}

impl Card {
  pub fn id(&self) -> Uuid {
    match *self {
      Card::BattleCard(BattleCard { id, .. }) => dbg!(id),
    }
  }

  pub fn tcg_id(&self) -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref tcg_id, .. }) => tcg_id.as_str(),
    }
  }

  pub fn rarity(&self) -> &CardRarity {
    match *self {
      Card::BattleCard(BattleCard { ref rarity, .. }) => rarity,
    }
  }

  pub fn number(&self) -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref number, .. }) => number.as_str(),
    }
  }

  pub fn category(&self) -> &CardCategory {
    match *self {
      Card::BattleCard(BattleCard { ref category, .. }) => category,
    }
  }

  pub fn wave(&self, context: &Context) -> Wave {
    match &*self {
      Card::BattleCard(card) => card.wave(context),
    }
  }
}

juniper::graphql_interface!(Card: Context |&self| {
  field id() -> Uuid {
    self.id()
  }

  field tcg_id() -> &str {
    self.tcg_id()
  }

  field rarity() -> &CardRarity {
    self.rarity()
  }

  field number() -> &str {
    self.number()
  }

  field category() -> &CardCategory {
    self.category()
  }

  field wave(&executor) -> Wave {
    self.wave(&executor.context())
  }

  instance_resolvers: |_| {
    &BattleCard => match *self { Card::BattleCard(ref card) => Some(card), _ => None },
      // BattleCard => {
      //   let results = battle_cards::table
      //   .inner_join(cards::table)
      //   .select((battle_cards::id, battle_cards::card_id, cards::tcg_id, cards::rarity, cards::number, cards::category, cards::wave_id))
      //   .first::<BattleCard>(&context.connection);
      //   match results {
      //     Ok(res) => Some(res),
      //     Err(_) => None
      //   }
      // }
      // &Droid => context.droids.get(&self.id),
  }
});
