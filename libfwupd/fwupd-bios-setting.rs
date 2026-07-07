// Copyright 2025 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1-or-later

// The type of BIOS setting.
// Since: 1.8.4
enum FwupdBiosSettingKind {
    // BIOS setting type is unknown.
    Unknown,
    // BIOS setting that has enumerated possible values.
    Enumeration,
    // BIOS setting that is an integer.
    Integer,
    // BIOS setting that accepts a string.
    String,
}

// The flags available for BIOS settings.
// Since: 2.1.7
#[derive(ToString(enum), FromString(enum))]
enum FwupdBiosSettingFlags {
    // No flags set.
    None = 0,
    // Setting is considered safe and useful to show to end users.
    UserFriendly = 1 << 0,
}
