use crate::{classes::Class, races::Race};

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    race: Race,
    class: Class,
    level: u32,
    hit_points: u32,
    proficiency_bonus: i32,
    ability_scores: AbilityScores,
}

/**
# Ability Score Summary

## Strength
Measures
Natural athleticism, bodily power

Important for
Barbarian, Fighter, Paladin

Racial Increases
Mountain dwarf (+2), Dragonborn (+2), Half-orc (+2), Human (+1)

## Dexterity
Measures
Physical agility, reflexes, balance, poise

Important for
Monk, Ranger, Rogue

Racial Increases
Elf (+2), Halfling (+2), Forest gnome (+1), Human (+1)

## Constitution
Measures
Health, stamina, vital force

Important for
Everyone

Racial Increases
Dwarf (+2), Stout halfling (+1), Rock gnome (+1), Half-orc (+1), Human (+1)

## Intelligence
Measures
Mental acuity, information recall, analytical skill

Important for
Wizard

Racial Increases
High elf (+1), Gnome (+2), Tiefling (+1), Human (+1)

## Wisdom
Measures
Awareness, intuition, insight

Important for
Cleric, Druid

Racial Increases
Hill dwarf (+1), Wood elf (+1), Human (+1)

## Charisma
Measures
Confidence, eloquence, leadership

Important for
Bard, Sorcerer, Warlock

Racial Increases
Half-elf (+2), Drow (+1), Lightfoot halfling (+1), Dragonborn (+1), Tiefling (+2), Human (+1)
**/
#[derive(Debug, Clone)]
pub struct AbilityScores {
    strength: u32,
    dexterity: u32,
    constitution: u32,
    intelligence: u32,
    wisdom: u32,
    charisma: u32,
}
