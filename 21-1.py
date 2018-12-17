
def run_game(my_hp, my_damage, my_armor, boss_hp, boss_damage, boss_armor):
    while True:
        
        boss_hp -= max(1, my_damage - boss_armor)
        if boss_hp <= 0:
            #print('boss loses', 'player =', my_hp)
            return True
        my_hp -= max(1, boss_damage - my_armor)
        if my_hp <= 0:
            #print('player loses', 'boss =', boss_hp)
            return False

weapons = []
weapons.append({'t': 'Dagger', 'cost': 8, 'damage': 4})
weapons.append({'t': 'Shortsword', 'cost': 10, 'damage': 5})
weapons.append({'t': 'Warhammer', 'cost': 25, 'damage': 6})
weapons.append({'t': 'Longsword', 'cost': 40, 'damage': 7})
weapons.append({'t': 'Greataxe', 'cost': 74, 'damage': 8})

armors = []
armors.append({'t': 'Leather', 'cost': 13, 'armor': 1})
armors.append({'t': 'Chainmail', 'cost': 31, 'armor': 2})
armors.append({'t': 'Splintmail', 'cost': 53, 'armor': 3})
armors.append({'t': 'Bandedmail', 'cost': 75, 'armor': 4})
armors.append({'t': 'Platemail', 'cost': 102, 'armor': 5})

rings = []
rings.append({'t':'Damage +1', 'cost': 25 , 'damage': 1, 'armor': 0})
rings.append({'t':'Damage +2', 'cost': 50 , 'damage': 2, 'armor': 0})
rings.append({'t':'Damage +3', 'cost':100 , 'damage': 3, 'armor': 0})
rings.append({'t':'Defense +1', 'cost': 20, 'damage': 0, 'armor': 1})
rings.append({'t':'Defense +2', 'cost': 40, 'damage': 0, 'armor': 2})
rings.append({'t':'Defense +3', 'cost': 80, 'damage': 0, 'armor': 3})

# boss
# Hit Points: 104
# Damage: 8
# Armor: 1

# my hp = 100

mincost = 9999999
for w in range(0, len(weapons)):
    my_weapon = weapons[w]['damage']
    for a in range(-1, len(armors)):
        my_armor = 0 if a == -1 else armors[a]['armor']
        for r1 in range(-1, len(rings)):
            my_ring1 = (0, 0) if r1 == -1 else (rings[r1]['damage'], rings[r1]['armor'])
            for r2 in range(-1, len(rings)):
                my_ring2 = (0, 0) if r2 == -1 else (rings[r2]['damage'], rings[r2]['armor'])

                if run_game(100, my_weapon + my_ring1[0] + my_ring2[0], my_armor + my_ring1[1] + my_ring2[1], 104, 8, 1):
                    wcost = weapons[w]['cost']
                    acost = 0 if a == -1 else armors[a]['cost']
                    r1cost = 0 if r1 == -1 else rings[r1]['cost']
                    r2cost = 0 if r2 == -1 else rings[r2]['cost']
                    cost = wcost + acost + r1cost + r2cost
                    if cost < mincost:
                        print('cost', cost, 'w', weapons[w], 'a', armors[a], 'r1', my_ring1, 'r2', my_ring2)
                        mincost = cost

print('mincost', mincost)