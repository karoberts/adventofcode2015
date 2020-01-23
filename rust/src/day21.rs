#![allow(dead_code)]
struct Item
{
    t: &'static str,
    cost: i32,
    damage: Option<i32>,
    armor: Option<i32>
}

fn run_game(my_hp_st:i32, my_damage:i32, my_armor:i32, boss_hp_st:i32, boss_damage:i32, boss_armor:i32) -> bool
{
    let mut my_hp = my_hp_st;
    let mut boss_hp = boss_hp_st;
    loop {
        boss_hp -= std::cmp::max(1, my_damage - boss_armor);
        if boss_hp <= 0 {
            return true;
        }
        my_hp -= std::cmp::max(1, boss_damage - my_armor);
        if my_hp <= 0 {
            return false;
        }
    }
}

pub fn _run()
{
    let weapons : Vec<Item> = vec![
        Item {t: "Dagger", cost: 8, damage: Some(4), armor: None},
        Item {t: "Shortsword", cost: 10, damage: Some(5), armor: None},
        Item {t: "Warhammer", cost: 25, damage: Some(6), armor: None},
        Item {t: "Longsword", cost: 40, damage: Some(7), armor: None},
        Item {t: "Greataxe", cost: 74, damage: Some(8), armor: None}
    ];

    let armors : Vec<Item> = vec![
        Item {t:"Leather", cost: 13, damage: None, armor: Some(1)},
        Item {t:"Chainmail", cost: 31, damage: None, armor: Some(2)},
        Item {t:"Splintmail", cost: 53, damage: None, armor: Some(3)},
        Item {t:"Bandedmail", cost: 75, damage: None, armor: Some(4)},
        Item {t:"Platemail", cost: 102, damage: None, armor: Some(5)}
    ];

    let rings : Vec<Item> = vec![
        Item {t:"Damage +1", cost: 25 , damage: Some(1), armor: Some(0)},
        Item {t:"Damage +2", cost: 50 , damage: Some(2), armor: Some(0)},
        Item {t:"Damage +3", cost:100 , damage: Some(3), armor: Some(0)},
        Item {t:"Defense +1", cost: 20, damage: Some(0), armor: Some(1)},
        Item {t:"Defense +2", cost: 40, damage: Some(0), armor: Some(2)},
        Item {t:"Defense +3", cost: 80, damage: Some(0), armor: Some(3)}
    ];

    let mut mincost = std::i32::MAX;
    let mut maxcost = 0;
    for w in 0..weapons.len() {
        let my_weapon = weapons[w].damage.unwrap();
        for a in -1..armors.len() as i32 {
            let my_armor = if a == -1 {0} else {armors[a as usize].armor.unwrap()};
            for r1 in -1..rings.len() as i32 {
                let my_ring1 = if r1 == -1 {(0,0)} else {(rings[r1 as usize].damage.unwrap(), rings[r1 as usize].armor.unwrap())};
                for r2 in -1..rings.len() as i32 {
                    let my_ring2 = if r2 == -1 {(0,0)} else {(rings[r2 as usize].damage.unwrap(), rings[r2 as usize].armor.unwrap())};

                    let wcost = weapons[w].cost;
                    let acost = if a == -1 {0} else {armors[a as usize].cost};
                    let r1cost = if r1 == -1 {0} else {rings[r1 as usize].cost};
                    let r2cost = if r2 == -1 {0} else {rings[r2 as usize].cost};
                    let cost = wcost + acost + r1cost + r2cost;

                    if run_game(100, my_weapon + my_ring1.0 + my_ring2.0, my_armor + my_ring1.1 + my_ring2.1, 104, 8, 1) {
                        if cost < mincost {
                            mincost = cost;
                        }
                    }
                    else {
                        if cost > maxcost {
                            maxcost = cost;
                        }
                    }
                }
            }
        }
    }

    println!("day21-1: {}", mincost);
    println!("day21-2: {}", maxcost);
}