table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    battle_cards (id) {
        id -> Int4,
        external_id -> Uuid,
        card_id -> Int4,
        title -> Varchar,
        #[sql_name = "type"]
        type_ -> Battle_type,
        faction -> Nullable<Faction>,
        stars -> Nullable<Int4>,
        icons -> Array<Battle_icon>,
        attack_modifier -> Nullable<Int4>,
        defense_modifier -> Nullable<Int4>,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    cards (id) {
        id -> Int4,
        external_id -> Uuid,
        tcg_id -> Varchar,
        rarity -> Card_rarity,
        number -> Varchar,
        category -> Card_category,
        wave_id -> Int4,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    character_modes (id) {
        id -> Int4,
        external_id -> Uuid,
        card_id -> Int4,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        faction -> Faction,
        traits -> Array<Character_trait>,
        #[sql_name = "type"]
        type_ -> Mode_type,
        stars -> Int4,
        health -> Nullable<Int4>,
        attack -> Nullable<Int4>,
        defense -> Nullable<Int4>,
        attack_modifier -> Nullable<Int4>,
        defense_modifier -> Nullable<Int4>,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    stratagem_cards (id) {
        id -> Int4,
        external_id -> Uuid,
        card_id -> Int4,
        title -> Varchar,
        requirement -> Varchar,
        faction -> Nullable<Faction>,
        stars -> Int4,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    waves (id) {
        id -> Int4,
        external_id -> Uuid,
        tcg_id -> Varchar,
        name -> Varchar,
        released -> Date,
    }
}

joinable!(battle_cards -> cards (card_id));
joinable!(cards -> waves (wave_id));
joinable!(character_modes -> cards (card_id));
joinable!(stratagem_cards -> cards (card_id));

allow_tables_to_appear_in_same_query!(
    battle_cards,
    cards,
    character_modes,
    stratagem_cards,
    waves,
    cards_with_pageinfo
);

// Custom views
table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    cards_with_pageinfo (id) {
        id -> Int4,
        external_id -> Uuid,
        tcg_id -> Varchar,
        rarity -> Card_rarity,
        number -> Varchar,
        category -> Card_category,
        wave_id -> Int4,
        has_previous -> Bool,
        has_next -> Bool,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    battle_cards_with_pageinfo (id) {
        id -> Int4,
        external_id -> Uuid,
        tcg_id -> Varchar,
        rarity -> Card_rarity,
        number -> Varchar,
        category -> Card_category,
        wave_id -> Int4,
        has_previous -> Bool,
        has_next -> Bool,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    character_cards_with_pageinfo (id) {
        id -> Int4,
        external_id -> Uuid,
        tcg_id -> Varchar,
        rarity -> Card_rarity,
        number -> Varchar,
        category -> Card_category,
        wave_id -> Int4,
        has_previous -> Bool,
        has_next -> Bool,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    use crate::data::{BattleType as Battle_type, BattleIcon as Battle_icon, Faction, CardRarity as Card_rarity, CardCategory as Card_category, CharacterTrait as Character_trait, ModeType as Mode_type};

    stratagem_cards_with_pageinfo (id) {
        id -> Int4,
        external_id -> Uuid,
        tcg_id -> Varchar,
        rarity -> Card_rarity,
        number -> Varchar,
        category -> Card_category,
        wave_id -> Int4,
        has_previous -> Bool,
        has_next -> Bool,
    }
}

joinable!(cards_with_pageinfo -> waves (wave_id));
