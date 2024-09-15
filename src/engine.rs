use std::{collections::HashMap, ffi::CStr, fmt::{self}};

// Halo 1 Xbox Retail
const HALO_OBJECT_POOL_HEADER_ADDR: usize = 0x000B9370;
const HALO_PLAYER_POOL_HEADER_ADDR: usize = 0x00213C50;

const HALO_TAG_ARRAY_HEADER_ADDR: usize = 0x003A6024;
const HALO_TAG_HEADER_ADDR: usize = 0x003A6000; 

const HALO_PLAYER_GLOBALS_ADDR: usize = 0x00214E00;
const MAXIMUM_NUMBER_OF_LOCAL_PLAYERS: usize = 4;

// Halo 1 Xbox Max Objects
const HALO_OBJECT_MAX_POOL_ENTRIES: usize = 2048;
const HALO_PLAYER_MAX_POOL_ENTRIES: usize = 16;

// Sanity check constants
const AT_T_AT_D: u32 = 1681945664;
const DEAH: u32 = 1751474532;
const LIAT: u32 = 1952541036;
const RNCS: u32 = 1935896178;

// Halo Structs
#[repr(C)]
#[derive(Clone)]
pub struct DatumHandle(u32);

impl DatumHandle {
    pub fn new_from_index_id(index: u16, id: u16) -> DatumHandle {
        DatumHandle { 0: ((id as u32) << 16) | index as u32 } 
    }

    pub fn get_index(&self) -> u16 {
        let lower_word = self.0 & 0xFFFF;
        lower_word as u16
    }

    // Referred to some as the salt
    pub fn get_id(&self) -> u16 {
        let upper_word = (self.0 >> 16) & 0xFFFF;
        upper_word as u16
    }

    pub fn get_handle(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for DatumHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatumHandle")
         .field("Handle", &self.get_handle())
         .field("Index", &self.get_index())
         .field("ID", &self.get_id())
         .finish()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct PlayersGlobals {
    pub unknown_1: i32,
    pub local_players: [DatumHandle; MAXIMUM_NUMBER_OF_LOCAL_PLAYERS],
    pub local_dead_players: [DatumHandle; MAXIMUM_NUMBER_OF_LOCAL_PLAYERS],
    pub local_player_count: u16,
    pub double_speed_ticks_remaining: u16,
    pub are_all_dead: u8,
    pub input_disabled: u8,
    pub unk_tag_index: u16, // bsp index??
    pub respawn_failure: u16,
    pub teleported: u8,
    pub unk_flags: u8,
    pub combined_pvs: [u8; 0x40],
    pub combined_pvs_local: [u8; 0x40]
}

#[derive(Debug)]
#[repr(C)]
pub struct PlayerPoolHeader { // Same as the object pool header
    pub name: [u8; 32],
    pub max_objects: u16,
    pub object_table_size: u16,
    pub unknown_1: u32,
    pub signature: u32,
    pub next_object_index: u16,
    pub max_object_count: u16,
    pub current_objects: u16,
    pub next_object_id: u16,
    pub object_data_begin: [u8; 4]
}

#[derive(Debug)]
#[repr(C)]
pub struct PlayerPoolEntry {
    pub id: u16, // update rev github with this info
    pub local_player_index: u16,
    pub player_name: [u16; 12],
    pub unknown_1: [i32; 6],
    pub slave_unit_index: DatumHandle, // datum
    pub last_slave_unit_index: DatumHandle, // datum
    pub unknown_2: [u8; 150]
}

#[derive(Debug)]
#[repr(C)]
pub struct TagEntry {
    tag_class: u32,
    tag_class_secondary: u32,
    tag_class_tertiary: u32,
    tag_index: u32, // tag_id
    tag_path_ptr: [u8; 4],
    tag_data_ptr: [u8; 4],
    unknown_1: u32,
    unknown_2: u32
}

#[derive(Debug)]
#[repr(C)]
pub struct TagHeader {
    tag_array_ptr: [u8; 4],
    tag_index: u32,
    map_id: u32,
    tag_count: u32,
    vertex_count: u32,
    vertex_offset: u32,
    index_count: u32,
    index_offset: u32,
    model_data_size: u32,
    footer: u32 // tags backwards
}

#[derive(Debug)]
#[repr(C)]
pub struct ObjectPoolEntry {
    pub id: u16,
    pub unknown_1: u16, 
    pub unknown_2: u16,
    pub size: u16,
    pub object_address: [u8; 4]
}

#[derive(Debug)]
#[repr(C)]
pub struct ObjectPoolHeader {
    pub name: [u8; 32],
    pub max_objects: u16,
    pub object_table_size: u16,
    pub unknown_1: u32,
    pub signature: u32,
    pub next_object_index: u16,
    pub max_object_count: u16,
    pub current_objects: u16, // is this supposed to be here?
    pub next_object_id: u16,
    pub object_data_begin: [u8; 4] // 32-bit pointer of 3 bytes?
}

#[derive(Debug)]
#[repr(C)]
pub struct GameObject {
    pub header_head: u32,
    pub tag_id: u32, 
    pub ptr_a: u32,
    pub ptr_next_object: u32,
    pub ptr_previous_object: u32,
    pub header_tail: u32,
    pub tag_index: u32,
    pub flags: u32,
    pub padding_1: u32,
    pub position: [f32; 3]
}

// Application
#[derive(Debug)]
pub struct EngineSnapshot {
    pub object_pool_header: ObjectPoolHeader,
    pub object_pool_entries: Vec<Option<ObjectPoolEntry>>, 
    pub game_object_entries: Vec<Option<GameObject>>,
    pub player_pool_header: PlayerPoolHeader,
    pub player_globals: PlayersGlobals,
    pub player_pool_entries: Vec<Option<PlayerPoolEntry>>,
    pub tags: HashMap<u32, String>
}

impl EngineSnapshot {
    pub fn find_local_player_index_from_unit_index(&self, index: u16) -> Option<usize> {
        for (_, player_pool_entry) in self.player_pool_entries.iter().enumerate() {
            if let Some(player_pool_entry) = player_pool_entry {
                if player_pool_entry.slave_unit_index.get_index() == index {
                    return Some(player_pool_entry.local_player_index as usize);
                }
            }
        }

        None
    }

    pub fn find_next_object_datum_player(&self, object_datum: DatumHandle) -> Option<usize> {
        for (index, player_object_datum_handle) in self.player_globals.local_dead_players.iter().enumerate() {
            if player_object_datum_handle.get_index() == object_datum.get_index() {
                return Some(index);
            }
        }
            
        None
    }
}

pub fn build_snapshot(bytes: &[u8]) -> Option<EngineSnapshot> {
    // Headers
    let pool_header: ObjectPoolHeader = unsafe { std::ptr::read(bytes[HALO_OBJECT_POOL_HEADER_ADDR..].as_ptr() as *const _) };
    if pool_header.signature != AT_T_AT_D { return None; }

    let tag_header: TagHeader = unsafe { std::ptr::read(bytes[HALO_TAG_HEADER_ADDR..].as_ptr() as *const _) };
    if tag_header.footer != RNCS { return None; }

    let player_pool_header: PlayerPoolHeader = unsafe { std::ptr::read(bytes[HALO_PLAYER_POOL_HEADER_ADDR..].as_ptr() as *const _) };
    if player_pool_header.signature != AT_T_AT_D { return None; } 

    // TODO: Find a way to sanity check this data
    let player_globals: PlayersGlobals = unsafe { std::ptr::read(bytes[HALO_PLAYER_GLOBALS_ADDR..].as_ptr() as *const _) };

    let mut game_object_entries: Vec<_> = (0..HALO_OBJECT_MAX_POOL_ENTRIES).map(|_| None).collect();
    let mut object_pool_entries: Vec<_> = (0..HALO_OBJECT_MAX_POOL_ENTRIES).map(|_| None).collect();
    let mut player_pool_entries: Vec<_> = (0..HALO_PLAYER_MAX_POOL_ENTRIES).map(|_| None).collect();
    
    let object_data_begin = u32::from_le_bytes([pool_header.object_data_begin[0], pool_header.object_data_begin[1], pool_header.object_data_begin[2], 0x0]);
    for index in (0..pool_header.max_objects as usize).rev() {
        let index_ptr_addr =  object_data_begin as usize + (size_of::<ObjectPoolEntry>() * index);
        let pool_entry: ObjectPoolEntry = unsafe { std::ptr::read(bytes[index_ptr_addr..].as_ptr() as *const _) };

        let base_pointer = u32::from_le_bytes([pool_entry.object_address[0], pool_entry.object_address[1], pool_entry.object_address[2], 0x0]);

        if base_pointer != 0 && base_pointer >= 0x18 {
            let game_object_pointer = base_pointer as usize - 0x18;
            let game_object_slice = &bytes[game_object_pointer..];
            let game_object: GameObject = unsafe { std::ptr::read(game_object_slice.as_ptr() as *const _) };

            if game_object.header_head == DEAH && game_object.header_tail == LIAT {
                game_object_entries[index] = Some(game_object);
                object_pool_entries[index] = Some(pool_entry);
            }
        }
    }

    // Get tag index mappings to tag names
    let mut tag_index_to_str: HashMap<u32, String> = HashMap::new();
    let tag_array_base_ptr = u32::from_le_bytes([tag_header.tag_array_ptr[0], tag_header.tag_array_ptr[1], tag_header.tag_array_ptr[2], 0x0]);

    for index in 0..tag_header.tag_count as usize {
        let tag_entry_ptr =  tag_array_base_ptr as usize + (size_of::<TagEntry>() * index);
        let tag_entry: TagEntry = unsafe { std::ptr::read(bytes[tag_entry_ptr..].as_ptr() as *const _) };

        if !tag_index_to_str.contains_key(&tag_entry.tag_index) {
            let tag_path_ptr = u32::from_le_bytes([tag_entry.tag_path_ptr[0], tag_entry.tag_path_ptr[1], tag_entry.tag_path_ptr[2], 0x0]) as usize;
            unsafe { 
                if let Ok(value) = CStr::from_ptr(bytes[tag_path_ptr..].as_ptr() as *const _).to_str() {
                    tag_index_to_str.insert(tag_entry.tag_index, value.to_string());
                }
            }
        }
    }

    // Player Pool Entries
    let player_object_data_begin = u32::from_le_bytes([player_pool_header.object_data_begin[0], player_pool_header.object_data_begin[1], player_pool_header.object_data_begin[2], 0x0]);
    for index in (0..player_pool_header.max_objects as usize).rev() {
        let index_ptr_addr =  player_object_data_begin as usize + (size_of::<PlayerPoolEntry>() * index);
        let player_pool_entry: PlayerPoolEntry = unsafe { std::ptr::read(bytes[index_ptr_addr..].as_ptr() as *const _) };

        if player_pool_entry.id != 0 {
            player_pool_entries[index] = Some(player_pool_entry);
        }
    }

    Some(EngineSnapshot {
        object_pool_header: pool_header,
        object_pool_entries: object_pool_entries,
        game_object_entries: game_object_entries,
        player_pool_header: player_pool_header,
        player_pool_entries: player_pool_entries,
        player_globals: player_globals,
        tags: tag_index_to_str 
    })
}
