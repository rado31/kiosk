#![allow(dead_code)]

use egui::Color32;

// Ant Design Default Theme Colors
// A clean, professional blue-based light color scheme

// ============================================================================
// BACKGROUND COLORS
// ============================================================================

/// Darkest background - used for dimmed/inactive areas
pub const BG_DIM: Color32 = Color32::from_rgb(245, 245, 245);
/// Primary background - main canvas/page background
pub const BG: Color32 = Color32::from_rgb(255, 255, 255);
/// Alias for BG
pub const BG_0: Color32 = Color32::from_rgb(255, 255, 255);
/// Elevated surface - cards, panels (level 1)
pub const BG_1: Color32 = Color32::from_rgb(255, 255, 255);
/// Elevated surface - popups, dropdowns (level 2)
pub const BG_2: Color32 = Color32::from_rgb(250, 250, 250);
/// Elevated surface - tooltips, modals (level 3)
pub const BG_3: Color32 = Color32::from_rgb(245, 245, 245);
/// Elevated surface - highest elevation (level 4)
pub const BG_4: Color32 = Color32::from_rgb(240, 240, 240);
/// Subtle background highlight
pub const BG_5: Color32 = Color32::from_rgb(230, 230, 230);
/// Light background variant
pub const BG_LIGHT: Color32 = Color32::from_rgb(245, 245, 245);

// ============================================================================
// FOREGROUND / TEXT COLORS
// ============================================================================

/// Primary text color - soft black for light gray backgrounds
pub const FG: Color32 = Color32::from_rgb(38, 38, 38);
/// Secondary/muted text - rgba(0, 0, 0, 0.65)
pub const FG_MUTED: Color32 = Color32::from_rgb(89, 89, 89);
/// Disabled text - rgba(0, 0, 0, 0.25)
pub const FG_DISABLED: Color32 = Color32::from_rgb(191, 191, 191);
/// Placeholder text - rgba(0, 0, 0, 0.45)
pub const FG_PLACEHOLDER: Color32 = Color32::from_rgb(140, 140, 140);

// ============================================================================
// GREY SCALE
// ============================================================================

/// Grey for borders, dividers
pub const GREY_0: Color32 = Color32::from_rgb(217, 217, 217);
/// Grey for secondary elements
pub const GREY_1: Color32 = Color32::from_rgb(191, 191, 191);
/// Grey for tertiary elements
pub const GREY_2: Color32 = Color32::from_rgb(140, 140, 140);

// ============================================================================
// PRIMARY COLORS (Blue-based)
// ============================================================================

/// Primary brand color - main actions, links (#1677ff)
pub const PRIMARY: Color32 = Color32::from_rgb(22, 119, 255);
/// Primary hover state (#4096ff)
pub const PRIMARY_HOVER: Color32 = Color32::from_rgb(64, 150, 255);
/// Primary active/pressed state (#0958d9)
pub const PRIMARY_ACTIVE: Color32 = Color32::from_rgb(9, 88, 217);
/// Primary background (subtle) (#e6f4ff)
pub const PRIMARY_BG: Color32 = Color32::from_rgb(230, 244, 255);
/// Primary border (#91caff)
pub const PRIMARY_BORDER: Color32 = Color32::from_rgb(145, 202, 255);

// ============================================================================
// SECONDARY COLORS (Cyan)
// ============================================================================

/// Secondary color - alternative actions (#13c2c2)
pub const SECONDARY: Color32 = Color32::from_rgb(19, 194, 194);
/// Secondary hover state (#36cfc9)
pub const SECONDARY_HOVER: Color32 = Color32::from_rgb(54, 207, 201);
/// Secondary active state (#08979c)
pub const SECONDARY_ACTIVE: Color32 = Color32::from_rgb(8, 151, 156);
/// Secondary background (subtle) (#e6fffb)
pub const SECONDARY_BG: Color32 = Color32::from_rgb(230, 255, 251);
/// Secondary border (#87e8de)
pub const SECONDARY_BORDER: Color32 = Color32::from_rgb(135, 232, 222);

// ============================================================================
// ACCENT COLORS
// ============================================================================

/// Blue accent (#1677ff)
pub const BLUE: Color32 = Color32::from_rgb(22, 119, 255);
pub const BLUE_HOVER: Color32 = Color32::from_rgb(64, 150, 255);
pub const BLUE_ACTIVE: Color32 = Color32::from_rgb(9, 88, 217);
pub const BLUE_BG: Color32 = Color32::from_rgb(230, 244, 255);

/// Purple accent (#722ed1)
pub const PURPLE: Color32 = Color32::from_rgb(114, 46, 209);
pub const PURPLE_HOVER: Color32 = Color32::from_rgb(146, 84, 222);
pub const PURPLE_ACTIVE: Color32 = Color32::from_rgb(83, 29, 171);
pub const PURPLE_BG: Color32 = Color32::from_rgb(249, 240, 255);

/// Orange accent (#fa8c16)
pub const ORANGE: Color32 = Color32::from_rgb(250, 140, 22);
pub const ORANGE_HOVER: Color32 = Color32::from_rgb(255, 169, 64);
pub const ORANGE_ACTIVE: Color32 = Color32::from_rgb(212, 107, 8);
pub const ORANGE_BG: Color32 = Color32::from_rgb(255, 247, 230);

/// Yellow accent (#fadb14)
pub const YELLOW: Color32 = Color32::from_rgb(250, 219, 20);
pub const YELLOW_HOVER: Color32 = Color32::from_rgb(255, 236, 61);
pub const YELLOW_ACTIVE: Color32 = Color32::from_rgb(212, 177, 6);
pub const YELLOW_BG: Color32 = Color32::from_rgb(255, 255, 230);

// ============================================================================
// SEMANTIC COLORS
// ============================================================================

/// Success - confirmations, positive feedback (#52c41a)
pub const SUCCESS: Color32 = Color32::from_rgb(82, 196, 26);
pub const SUCCESS_HOVER: Color32 = Color32::from_rgb(115, 209, 61);
pub const SUCCESS_ACTIVE: Color32 = Color32::from_rgb(56, 158, 13);
pub const SUCCESS_BG: Color32 = Color32::from_rgb(246, 255, 237);
pub const SUCCESS_BORDER: Color32 = Color32::from_rgb(183, 235, 143);

/// Warning - caution, attention needed (#faad14)
pub const WARNING: Color32 = Color32::from_rgb(250, 173, 20);
pub const WARNING_HOVER: Color32 = Color32::from_rgb(255, 197, 61);
pub const WARNING_ACTIVE: Color32 = Color32::from_rgb(212, 136, 6);
pub const WARNING_BG: Color32 = Color32::from_rgb(255, 251, 230);
pub const WARNING_BORDER: Color32 = Color32::from_rgb(255, 229, 143);

/// Error/Danger - errors, destructive actions (#ff4d4f)
pub const ERROR: Color32 = Color32::from_rgb(255, 77, 79);
pub const ERROR_HOVER: Color32 = Color32::from_rgb(255, 120, 117);
pub const ERROR_ACTIVE: Color32 = Color32::from_rgb(217, 54, 56);
pub const ERROR_BG: Color32 = Color32::from_rgb(255, 242, 240);
pub const ERROR_BORDER: Color32 = Color32::from_rgb(255, 163, 158);

/// Info - informational messages (#1677ff)
pub const INFO: Color32 = Color32::from_rgb(22, 119, 255);
pub const INFO_HOVER: Color32 = Color32::from_rgb(64, 150, 255);
pub const INFO_ACTIVE: Color32 = Color32::from_rgb(9, 88, 217);
pub const INFO_BG: Color32 = Color32::from_rgb(230, 244, 255);
pub const INFO_BORDER: Color32 = Color32::from_rgb(145, 202, 255);

// ============================================================================
// BORDER COLORS
// ============================================================================

/// Default border (#d9d9d9)
pub const BORDER: Color32 = Color32::from_rgb(217, 217, 217);
/// Border on hover
pub const BORDER_HOVER: Color32 = Color32::from_rgb(64, 150, 255);
/// Focused border
pub const BORDER_FOCUS: Color32 = Color32::from_rgb(22, 119, 255);
/// Disabled border
pub const BORDER_DISABLED: Color32 = Color32::from_rgb(217, 217, 217);

// ============================================================================
// SHADOW / OVERLAY
// ============================================================================

/// Shadow color (for drop shadows)
pub const SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 30);
/// Overlay color (for modals, drawers)
pub const OVERLAY: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 115);

// ============================================================================
// SPECIAL
// ============================================================================

/// Link color (#1677ff)
pub const LINK: Color32 = Color32::from_rgb(22, 119, 255);
/// Link hover (#69b1ff)
pub const LINK_HOVER: Color32 = Color32::from_rgb(105, 177, 255);
/// Selection background
pub const SELECTION_BG: Color32 = Color32::from_rgba_premultiplied(22, 119, 255, 40);
/// Highlight background
pub const HIGHLIGHT_BG: Color32 = Color32::from_rgba_premultiplied(250, 173, 20, 30);
/// Pure white
pub const WHITE: Color32 = Color32::WHITE;
/// Pure black
pub const BLACK: Color32 = Color32::BLACK;

// ============================================================================
// COMPONENT-SPECIFIC (Convenience aliases)
// ============================================================================

/// Button primary background
pub const BTN_PRIMARY_BG: Color32 = PRIMARY;
pub const BTN_PRIMARY_HOVER: Color32 = PRIMARY_HOVER;
pub const BTN_PRIMARY_ACTIVE: Color32 = PRIMARY_ACTIVE;
pub const BTN_PRIMARY_TEXT: Color32 = WHITE;

/// Button light background
pub const BTN_BG_LIGHT: Color32 = BG_3;

/// Button default (secondary style)
pub const BTN_DEFAULT_BG: Color32 = WHITE;
pub const BTN_DEFAULT_HOVER: Color32 = BG_2;
pub const BTN_DEFAULT_ACTIVE: Color32 = BG_3;
pub const BTN_DEFAULT_TEXT: Color32 = FG;

/// Button success (green)
pub const BTN_GREEN: Color32 = SUCCESS;
pub const BTN_GREEN_HOVER: Color32 = SUCCESS_HOVER;
pub const BTN_GREEN_ACTIVE: Color32 = SUCCESS_ACTIVE;

/// Button danger (red)
pub const BTN_RED: Color32 = ERROR;
pub const BTN_RED_HOVER: Color32 = ERROR_HOVER;
pub const BTN_RED_ACTIVE: Color32 = ERROR_ACTIVE;

/// Input fields
pub const INPUT_BG: Color32 = WHITE;
pub const INPUT_BORDER: Color32 = BORDER;
pub const INPUT_FOCUS_BORDER: Color32 = PRIMARY;
pub const INPUT_PLACEHOLDER: Color32 = FG_PLACEHOLDER;

/// Card component
pub const CARD_BG: Color32 = WHITE;
pub const CARD_BORDER: Color32 = BORDER;

/// Header/Navigation
pub const HEADER_BG: Color32 = WHITE;
pub const NAV_ITEM_ACTIVE: Color32 = PRIMARY;
pub const NAV_ITEM_HOVER_BG: Color32 = BG_3;

/// Table/List
pub const TABLE_HEADER_BG: Color32 = BG_3;
pub const TABLE_ROW_HOVER: Color32 = BG_2;
pub const TABLE_BORDER: Color32 = BORDER;

/// Divider
pub const DIVIDER: Color32 = BORDER;
