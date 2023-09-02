use binary_layout::prelude::*;

define_layout!(save, LittleEndian, {
    header: super::header::NestedView,
    character_1: character::NestedView,
    character_2: character::NestedView,
    character_3: character::NestedView,
    character_4: character::NestedView,
    character_5: character::NestedView,
    character_6: character::NestedView,
    character_7: character::NestedView,
    character_8: character::NestedView,
    character_9: character::NestedView,
    character_10: character::NestedView,
    general: super::general::NestedView
});

// Should total 768 bytes
define_layout!(header, LittleEndian, {
    block_1: [u8; 768]
});

// Should total 2621456 bytes
define_layout!(character, LittleEndian, {
    checksum: [u8; 16],
    block_1: [u8; 2621440],
    // block_1: [u8; 56159],
    // class: u8, // This doesn't align with my testing
    // // I don't believe there is a consistent index where the character classes are stored
    // // This begs the question of how the game finds a character's starting class at all
    // block_2: [u8; 2565280],
});

// Should total 393232 bytes
define_layout!(general, LittleEndian, {
    checksum: [u8; 16],
    block_1: [u8; 4],
    steam_id: u64,
    block_2: [u8; 6482],
    character_1: character_select::NestedView,
    character_2: character_select::NestedView,
    character_3: character_select::NestedView,
    character_4: character_select::NestedView,
    character_5: character_select::NestedView,
    character_6: character_select::NestedView,
    character_7: character_select::NestedView,
    character_8: character_select::NestedView,
    character_9: character_select::NestedView,
    character_10: character_select::NestedView,
    block_3: [u8; 380858]
});

// Should total 588 bytes
define_layout!(character_select, LittleEndian, {
    name: [u8; 32],
    block_1: [u8; 2],
    level: u16,
    block_2: [u8; 2],
    play_time: u32,
    block_3: [u8; 546]
});
