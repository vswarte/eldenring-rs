use pelite::Pod;

#[repr(C)]
#[derive(Debug, Pod)]
/// Usually located immediately after the `WorldChrManDbg` singleton.
pub struct CharacterDebugFlags {
    /// prevents death by setting HP to 1 when they are less than 0
    pub no_dead: u8,
    unk1: u8,
    /// activates same effect as the `Transient Curse` item from Dark Souls 1
    pub transient_curse_active: u8,
    /// prevents consumption of usable items
    pub no_goods_consume: u8,
    /// prevents stamina consumption
    pub no_stamina_consume: u8,
    /// prevents FP consumption
    pub no_fp_consume: u8,
    /// prevents durability loss (leftover from Dark Souls)
    pub no_item_damage: u8,
    /// prevents spell consumption (leftover from Dark Souls)
    pub no_spells_consume: u8,
    unk8: u8,
    unk9: u8,
    /// prevents death of enemies, same as `no_dead`
    pub enemy_no_dead: u8,
    /// does the same as `no_fp_consume`
    pub no_fp_consume2: u8,
    /// prevents enemies from being hit
    pub enemy_no_hit: u8,
    /// prevents enemies from attacking
    pub enemy_no_attack: u8,
    /// prevents enemies from pursuing the player
    pub enemy_no_pursuit: u8,
    /// prevents enemies from moving
    pub enemy_no_move: u8,
    unk10: u8,
    unk11: u8,
    /// same as `no_goods_consume` but for enemies (gives infinite heal flasks for npc invaders)
    pub enemy_no_goods_consume: u8,
    /// auto-parries all attacks for both player and enemies
    pub auto_parry: u8,
    /// disables enemy rendering
    pub enemy_no_draw: u8,
    /// replaces AOW attack animations with no FP versions
    pub no_fp_aow: u8,
    unk16: u8,
    unk17: u8,
    unk18: u8,
    unk19: u8,
}
