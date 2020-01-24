use super::utils;

//#![allow(dead_code)]
#[derive(Clone,Debug)]
struct Item
{
    t: &'static str,
    cost: i32,
    damage: Option<i32>,
    timer: Option<i32>,
    mana: Option<i32>,
    hp: Option<i32>,
    armor: Option<i32>
}

type SpellsType = utils::HashMapFnv<String, Item>;

fn inc_timer(cur_spells:&SpellsType) -> SpellsType
{
    let mut spells : SpellsType = fastmap!();
    for (t, sp) in cur_spells.iter() {
        match sp.timer
        {
            Some(tm) => {
                let tmr = tm - 1;
                if tmr >= 0 {
                    let mut newsp = sp.clone();
                    newsp.timer = Some(tmr);
                    spells.insert(t.to_owned(), newsp);
                }
            },
            None => ()
        };
    }
    spells
}

fn get_spells_res(cur_spells:&SpellsType) -> Item
{
    let mut ret = Item {t: "ret", hp: Some(0), mana: Some(0), armor: Some(0), damage: Some(0), cost:0, timer:None};
    for sp in cur_spells.values() {
        match sp.damage {
            None => (),
            Some(x) => ret.damage = Some(ret.damage.unwrap() + x)
        };
        match sp.hp {
            None => (),
            Some(x) => ret.hp = Some(ret.hp.unwrap() + x)
        };
        match sp.armor {
            None => (),
            Some(x) => ret.armor = Some(ret.armor.unwrap() + x)
        };
        match sp.mana {
            None => (),
            Some(x) => ret.mana = Some(ret.mana.unwrap() + x)
        };
    }
    ret
}

fn get_spell_res(sp:&Item) -> Item
{
    let mut ret = Item {t: "ret", hp: Some(0), mana: Some(0), armor: Some(0), damage: Some(0), cost:0, timer:None};
    if sp.timer.is_none() {
        match sp.damage {
            None => (),
            Some(x) => ret.damage = Some(ret.damage.unwrap() + x)
        };
        match sp.hp {
            None => (),
            Some(x) => ret.hp = Some(ret.hp.unwrap() + x)
        };
        match sp.armor {
            None => (),
            Some(x) => ret.armor = Some(ret.armor.unwrap() + x)
        };
        match sp.mana {
            None => (),
            Some(x) => ret.mana = Some(ret.mana.unwrap() + x)
        };
    }
    ret
}

fn apply_spells(sp:&Item, me:&Item, boss:&Item, apply_hp_cost:bool, hp_turn_cost:i32) -> (Item, Item)
{
    let mut new_me = me.clone();
    let mut new_boss = boss.clone();

    new_me.hp = Some(me.hp.unwrap() + sp.hp.unwrap_or(0));
    new_me.mana = Some(me.mana.unwrap() + sp.mana.unwrap_or(0));
    new_me.armor = Some(me.armor.unwrap() + sp.armor.unwrap_or(0));

    new_boss.hp = Some(boss.hp.unwrap() - sp.damage.unwrap_or(0));

    if apply_hp_cost && hp_turn_cost > 0 {
        new_me.hp = Some(new_me.hp.unwrap() + hp_turn_cost);
    }

    (new_me, new_boss)
}

fn run_game(me:&Item, boss:&Item, next_spell:&Item, cur_spells:&SpellsType, cur_spend:i32, depth:usize, min_mana_win:&mut i32, hp_turn_cost:i32, spells:&Vec<Item>) 
{
    if cur_spend > *min_mana_win {
        return;
    }

    if hp_turn_cost > 0 {
        if me.hp.unwrap() <= 1 {
            //print(indent, ' @ player loses', 'boss =', boss['hp'])
            return;
        }
    }

    let mut new_cur_spells = inc_timer(cur_spells);
    match new_cur_spells.get(next_spell.t) {
        None => (),
        Some(s) => {
            match s.timer {
                None => (),
                Some(t) => {
                    if t > 0 {
                        return;
                    }
                }
            }
        }
    }

    let mut spell_res = get_spells_res(&new_cur_spells);

    let (mut new_me, mut new_boss) = apply_spells(&spell_res, me, boss, true, hp_turn_cost);

    // check for boss death
    if new_boss.hp.unwrap_or(0) <= 0 {
        if cur_spend < *min_mana_win {
            *min_mana_win = cur_spend;
            println!("new min {}", cur_spend);
        }
        return;
    }

    new_me.mana = Some(new_me.mana.unwrap() - next_spell.cost);
    let new_cur_spend = cur_spend + next_spell.cost;

    if new_me.mana.unwrap() < 0 {
        return;
    }

    if next_spell.timer.is_some() {
        new_cur_spells.insert(next_spell.t.to_owned(), next_spell.clone());
    }
    spell_res = get_spell_res(next_spell);

    let r2 = apply_spells(&spell_res, &new_me, &new_boss, false, hp_turn_cost);
    new_me = r2.0;
    new_boss = r2.1;

    // check for boss death
    if new_boss.hp.unwrap_or(0) <= 0 {
        if new_cur_spend < *min_mana_win {
            *min_mana_win = new_cur_spend;
            println!("new min {}", new_cur_spend);
        }
        return;
    }

    // start boss turn
    new_cur_spells = inc_timer(&new_cur_spells);
    spell_res = get_spells_res(&new_cur_spells);

    let r3 = apply_spells(&spell_res, &new_me, &new_boss, false, hp_turn_cost);
    new_me = r3.0;
    new_boss = r3.1;

    // check for boss death
    if new_boss.hp.unwrap_or(0) <= 0 {
        if new_cur_spend < *min_mana_win {
            *min_mana_win = new_cur_spend;
            println!("new min {}", new_cur_spend);
        }
        return;
    }

    new_me.hp = Some( new_me.hp.unwrap() - std::cmp::max(1, new_boss.damage.unwrap_or(0) - new_me.armor.unwrap_or(0)));

    // check for player death
    if new_me.hp.unwrap_or(0) <= 0 {
        return;
    }

    // loop through spells not in new_cur_spells, call run_game for each
    for next_sp in spells.iter() {
        run_game(&mut new_me, &mut new_boss, &next_sp, &new_cur_spells, new_cur_spend, depth + 1, min_mana_win, hp_turn_cost, spells);
    }
}

pub fn _run()
{
    let spells : Vec<Item> = vec![
        Item {t: "Poison", cost: 173, damage: Some(3), timer: Some(6), armor: None, hp:None, mana:None},
        Item {t: "Recharge", cost: 229, mana: Some(101), timer: Some(5), armor: None, hp:None, damage:None},
        Item {t: "Sheild", cost: 113, armor: Some(7), timer: Some(6), hp:None, mana:None, damage:None},
        Item {t: "Magic Missle", cost: 53, damage: Some(4), armor: None, hp:None, mana:None, timer:None},
        Item {t: "Drain", cost: 73, damage: Some(2), hp: Some(2), armor: None, mana:None, timer:None}
    ];

    let mut me = Item {t: "me", hp: Some(50), mana: Some(500), armor: Some(0), damage: Some(0), cost:0, timer:None};
    let mut boss = Item {t: "boss", hp: Some(55), mana: None, armor: Some(0), damage: Some(8), cost:0, timer:None};
    let mut hp_turn_cost = 0;

    let mut min_mana_win = std::i32::MAX;

    for next_sp in spells.iter() {
        println!("starting with {}", next_sp.t);
        let cur_spells = fastmap!();
        run_game(&mut me, &mut boss, &next_sp, &cur_spells, 0, 0, &mut min_mana_win, hp_turn_cost, &spells);
    }

    println!("day22-1: {}", min_mana_win);
}