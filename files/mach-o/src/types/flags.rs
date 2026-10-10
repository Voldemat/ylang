pub struct Flags(pub u32);

impl Flags {
    pub const NO_UNDEFS: u32 = 1 << 0;
    pub const INCR_LINK: u32 = 1 << 1;
    pub const DYLD_LINK: u32 = 1 << 2;
    pub const BIND_AT_LOAD: u32 = 1 << 3;
    pub const PREBOUND: u32 = 1 << 4;
    pub const SPLIT_SEGS: u32 = 1 << 5;
    pub const LAZY_INIT: u32 = 1 << 6;
    pub const TWO_LEVEL: u32 = 1 << 7;
    pub const FORCE_FLAT: u32 = 1 << 8;
    pub const NO_MULTI_DEFS: u32 = 1 << 9;
    pub const NO_FIX_PREBINDING: u32 = 1 << 10;
    pub const PREBINDABLE: u32 = 1 << 11;
    pub const ALL_MODS_BOUND: u32 = 1 << 12;
    pub const SUBSECTIONS_VIA_SYMBOLS: u32 = 1 << 13;
    pub const CANONICAL: u32 = 1 << 14;
    pub const WEAK_DEFINES: u32 = 1 << 15;
    pub const BINDS_TO_WEAK: u32 = 1 << 16;
    pub const ALLOW_STACK_EXECUTION: u32 = 1 << 17;
    pub const ROOT_SAFE: u32 = 1 << 18;
    pub const SETGID_SAFE: u32 = 1 << 19;
    pub const NO_REEXPORTED_DYLIBS: u32 = 1 << 20;
    pub const PIE: u32 = 1 << 21;
    pub const DEAD_STRIPPABLE_DYLIB: u32 = 1 << 22;
    pub const HAS_TLV_DESCRIPTORS: u32 = 1 << 23;
    pub const NO_HEAP_EXECUTION: u32 = 1 << 24;
    pub const APP_EXTENSION_SAFE: u32 = 1 << 25;
    pub const NLIST_OUTOFSYNC_DYLDINFO: u32 = 1 << 26;
    pub const SIM_SUPPORT: u32 = 1 << 27;
    pub const DYLD_SHAREABLE: u32 = 1 << 31;

    pub fn is_no_undefs(&self) -> bool {
        (self.0 & Self::NO_UNDEFS) != 0
    }

    pub fn is_incr_link(&self) -> bool {
        (self.0 & Self::INCR_LINK) != 0
    }

    pub fn is_dyld_link(&self) -> bool {
        (self.0 & Self::DYLD_LINK) != 0
    }

    pub fn is_bind_at_load(&self) -> bool {
        (self.0 & Self::BIND_AT_LOAD) != 0
    }

    pub fn is_prebound(&self) -> bool {
        (self.0 & Self::PREBOUND) != 0
    }

    pub fn is_split_segs(&self) -> bool {
        (self.0 & Self::SPLIT_SEGS) != 0
    }

    pub fn is_lazy_init(&self) -> bool {
        (self.0 & Self::LAZY_INIT) != 0
    }

    pub fn is_two_level(&self) -> bool {
        (self.0 & Self::TWO_LEVEL) != 0
    }

    pub fn is_force_flat(&self) -> bool {
        (self.0 & Self::FORCE_FLAT) != 0
    }

    pub fn is_no_multi_defs(&self) -> bool {
        (self.0 & Self::NO_MULTI_DEFS) != 0
    }

    pub fn is_no_fix_prebinding(&self) -> bool {
        (self.0 & Self::NO_FIX_PREBINDING) != 0
    }

    pub fn is_prebindable(&self) -> bool {
        (self.0 & Self::PREBINDABLE) != 0
    }

    pub fn is_all_mods_bound(&self) -> bool {
        (self.0 & Self::ALL_MODS_BOUND) != 0
    }

    pub fn is_subsections_via_symbols(&self) -> bool {
        (self.0 & Self::SUBSECTIONS_VIA_SYMBOLS) != 0
    }

    pub fn is_canonical(&self) -> bool {
        (self.0 & Self::CANONICAL) != 0
    }

    pub fn is_weak_defines(&self) -> bool {
        (self.0 & Self::WEAK_DEFINES) != 0
    }

    pub fn is_binds_to_weak(&self) -> bool {
        (self.0 & Self::BINDS_TO_WEAK) != 0
    }

    pub fn is_allow_stack_execution(&self) -> bool {
        (self.0 & Self::ALLOW_STACK_EXECUTION) != 0
    }

    pub fn is_root_safe(&self) -> bool {
        (self.0 & Self::ROOT_SAFE) != 0
    }

    pub fn is_setgid_safe(&self) -> bool {
        (self.0 & Self::SETGID_SAFE) != 0
    }

    pub fn is_no_reexported_dylibs(&self) -> bool {
        (self.0 & Self::NO_REEXPORTED_DYLIBS) != 0
    }

    pub fn is_pie(&self) -> bool {
        (self.0 & Self::PIE) != 0
    }

    pub fn is_dead_strippable_dylib(&self) -> bool {
        (self.0 & Self::DEAD_STRIPPABLE_DYLIB) != 0
    }

    pub fn is_has_tlv_descriptors(&self) -> bool {
        (self.0 & Self::HAS_TLV_DESCRIPTORS) != 0
    }

    pub fn is_no_heap_execution(&self) -> bool {
        (self.0 & Self::NO_HEAP_EXECUTION) != 0
    }

    pub fn is_app_extension_safe(&self) -> bool {
        (self.0 & Self::APP_EXTENSION_SAFE) != 0
    }

    pub fn is_nlist_outofsync_dyldinfo(&self) -> bool {
        (self.0 & Self::NLIST_OUTOFSYNC_DYLDINFO) != 0
    }

    pub fn is_sim_support(&self) -> bool {
        (self.0 & Self::SIM_SUPPORT) != 0
    }

    pub fn is_dyld_shareable(&self) -> bool {
        (self.0 & Self::DYLD_SHAREABLE) != 0
    }
}

