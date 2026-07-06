use crate::world::{
    Map,
    Pos,
};

fn cell_symbol(
    map: &Map,
    pos: Pos,
    player_pos: Pos,
) -> &'static str {
    if pos == player_pos {
        return " @ ";
    }
    let Some(room) = map.rooms.get(&pos) else {
        return "///";
    };
    if !room.visited {
        return " # ";
    }
    if pos == map.exit_pos {
        return " X ";
    }
    if room.enemy.is_some() {
        return " E ";
    }
    if room.npc.is_some() {
        return " N ";
    }
    if !room.items.is_empty() {
        return " ? ";
    }
    "   "
}

fn has_right_passage(
    map: &Map,
    r: i32,
    c: i32,
    max_c: i32,
) -> bool {
    c < max_c
        && map.rooms.contains_key(&(r, c))
        && map.rooms.contains_key(&(r, c + 1))
}

fn has_bottom_passage(
    map: &Map,
    r: i32,
    c: i32,
    max_r: i32,
) -> bool {
    r < max_r
        && map.rooms.contains_key(&(r, c))
        && map.rooms.contains_key(&(r + 1, c))
}

/// Draw the map as ASCII art showing only rooms within `view_dist` of the player.
pub fn draw_map(
    map: &Map,
    player_pos: Pos,
    view_dist: i32,
) -> String {
    let mut lines = Vec::new();

    let min_r = player_pos.0 - view_dist;
    let max_r = player_pos.0 + view_dist;
    let min_c = player_pos.1 - view_dist;
    let max_c = player_pos.1 + view_dist;

    let width = (max_c - min_c + 1) as usize;

    let mut top = "+".to_string();
    for _ in 0..width {
        top += "---+";
    }
    lines.push(top);

    for r in min_r..=max_r {
        let mut row_mid = "|".to_string();
        let mut row_bot = "+".to_string();

        for c in min_c..=max_c {
            let pos: Pos = (r, c);
            let cell = cell_symbol(map, pos, player_pos);

            let right = if has_right_passage(map, r, c, max_c) {
                " "
            } else {
                "|"
            };
            row_mid += cell;
            row_mid += right;

            let bot = if has_bottom_passage(map, r, c, max_r) {
                "   "
            } else {
                "---"
            };
            row_bot += bot;
            row_bot += "+";
        }

        lines.push(row_mid);
        lines.push(row_bot);
    }

    lines.push(String::new());
    lines.push(
        "@ You  X Exit  E Enemy  N NPC  ? Item  # Unexplored  /// Wall".into(),
    );
    lines.join("\n")
}
