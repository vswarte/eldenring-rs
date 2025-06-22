#[repr(C)]
#[derive(Debug)]
/// Usually located immediately after the `WorldChrManDbg` singleton.
pub struct CharacterDebugFlags {
    /// prevents death by setting HP to 1 when they are less than 0
    pub no_dead: bool,
    unk1: bool,
    /// deals 9999999 damage on every hit
    pub exterminate: bool,
    /// prevents consumption of usable items
    pub no_goods_consume: bool,
    /// prevents stamina consumption
    pub no_stamina_consume: bool,
    /// prevents FP consumption
    pub no_fp_consume: bool,
    /// prevents durability loss (leftover from Dark Souls)
    pub no_item_damage: bool,
    /// prevents spell consumption (leftover from Dark Souls)
    pub no_spells_consume: bool,
    unk8: bool,
    unk9: bool,
    /// prevents death of enemies, same as `no_dead`
    pub enemy_no_dead: bool,
    /// does the same as `no_fp_consume`
    pub no_fp_consume2: bool,
    /// prevents enemies from being hit
    pub enemy_no_hit: bool,
    /// prevents enemies from attacking
    pub enemy_no_attack: bool,
    /// prevents enemies from pursuing the player
    pub enemy_no_pursuit: bool,
    /// prevents enemies from moving
    pub enemy_no_move: bool,
    unk10: bool,
    unk11: bool,
    /// same as `no_goods_consume` but for enemies (gives infinite heal flasks for npc invaders)
    pub enemy_no_goods_consume: bool,
    /// auto-parries all attacks for both player and enemies
    pub auto_parry: bool,
    /// disables enemy rendering
    pub enemy_no_draw: bool,
    /// replaces AOW attack animations with no FP versions
    pub no_fp_aow: bool,
    unk16: bool,
    unk17: bool,
    unk18: bool,
    unk19: bool,
}
