
def inc_timer(cur_spells):
    spells = {}
    for t, sp in cur_spells.items():
        if 'timer' in sp:
            tmr = sp['timer'] - 1
            if tmr >= 0:
                spells[t] = dict(sp)
                spells[t]['timer'] = tmr
    return spells

def copy_cur_spells(d):
    {x:dict(y) for x,y in d.items()}

def get_spells_res(cur_spells, indent):
    global output

    ret = {'damage': 0, 'hp': 0, 'armor': 0, 'mana': 0}
    for sp in cur_spells.values():
        if 'damage' in sp:
            ret['damage'] += sp['damage']
            if output:
                print(indent, '{} provides {} damage, timer now {}'.format(sp['t'], sp['damage'], sp['timer']))
        if 'hp' in sp:
            ret['hp'] += sp['hp']
            if output:
                print(indent, '{} provides {} hp, timer now {}'.format(sp['t'], sp['hp'], sp['timer']))
        if 'armor' in sp:
            ret['armor'] += sp['armor']
            if output:
                print(indent, '{} provides {} armor, timer now {}'.format(sp['t'], sp['armor'], sp['timer']))
        if 'mana' in sp:
            ret['mana'] += sp['mana']
            if output:
                print(indent, '{} provides {} mana, timer now {}'.format(sp['t'], sp['mana'], sp['timer']))
    return ret

def get_spell_res(sp, indent):
    ret = {'damage': 0, 'hp': 0, 'armor': 0, 'mana': 0}

    if 'timer' not in sp:
        if 'damage' in sp:
            ret['damage'] += sp['damage']
            if output:
                print(indent, '{} provides {} damage'.format(sp['t'], sp['damage']))
        if 'hp' in sp:
            ret['hp'] += sp['hp']
            if output:
                print(indent, '{} provides {} hp'.format(sp['t'], sp['hp']))
        if 'armor' in sp:
            ret['armor'] += sp['armor']
            if output:
                print(indent, '{} provides {} armor'.format(sp['t'], sp['armor']))
        if 'mana' in sp:
            ret['mana'] += sp['mana']
            if output:
                print(indent, '{} provides {} mana'.format(sp['t'], sp['mana']))

    return ret

def apply_spells(sp, me, boss):
    new_me = dict(me)
    new_boss = dict(boss)

    new_me['hp'] += sp['hp']
    new_me['mana'] += sp['mana']
    new_me['armor'] = sp['armor']

    new_boss['hp'] -= sp['damage']

    return (new_me, new_boss)

def run_game(me, boss, next_spell, cur_spells, cur_spend, depth):
    global metawins
    global min_mana_win
    global outputLoss
    global hp_turn_cost

    indent = '  ' * depth

    if cur_spend > min_mana_win:
        return

    # start player turn
    if output:
        print(indent, '-- Player turn --')
        print(indent, '- Player has {} hit points, {} armor, {} mana'.format(me['hp'], me['armor'], me['mana']))
        print(indent, '- Boss has {} hit points'.format(boss['hp']))

    if hp_turn_cost > 0:
        if output:
            print(indent, 'Player loses', hp_turn_cost, 'hp')
        me = dict(me)
        me['hp'] -= hp_turn_cost

        if me['hp'] <= 0:
            if output:
                print(indent, ' @ player loses', 'boss =', boss['hp'])
            return

    new_cur_spells = inc_timer(cur_spells)
    spell_res = get_spells_res(new_cur_spells, indent)

    (new_me, new_boss) = apply_spells(spell_res, me, boss)

    # check for boss death
    if new_boss['hp'] <= 0:
        if output or outputLoss:
            print(indent, ' ! boss loses', 'player =', me['hp'], 'spend =', cur_spend)
        manawins.append(cur_spend)
        if cur_spend < min_mana_win:
            min_mana_win = cur_spend
            #print(indent, ' ! boss loses', 'player =', me['hp'], 'spend =', cur_spend)
            print('new min', cur_spend)
        return

    if output:
        print(indent, 'Player casts', next_spell['t'])
    new_me['mana'] -= next_spell['cost']
    cur_spend += next_spell['cost']

    if new_me['mana'] < 0:
        if output:
            print(indent, ' @ player loses, no mana', 'boss =', new_boss['hp'], 'player =', new_me['hp'])
        return

    if 'timer' in next_spell:
        new_cur_spells[next_spell['t']] = dict(next_spell)
    spell_res = get_spell_res(next_spell, indent)

    (new_me, new_boss) = apply_spells(spell_res, new_me, new_boss)

    # check for boss death
    if new_boss['hp'] <= 0:
        if output or outputLoss:
            print(indent, ' ! boss loses', 'player =', new_me['hp'], 'spend =', cur_spend)
        manawins.append(cur_spend)
        if cur_spend < min_mana_win:
            min_mana_win = cur_spend
            #print(indent, ' ! boss loses', 'player =', me['hp'], 'spend =', cur_spend)
            print('new min', cur_spend)
        return

    # start boss turn
    if output:
        print(indent, '-- Boss turn --')
        print(indent, '- Player has {} hit points, {} armor, {} mana'.format(new_me['hp'], new_me['armor'], new_me['mana']))
        print(indent, '- Boss has {} hit points'.format(new_boss['hp']))

    new_cur_spells = inc_timer(new_cur_spells)
    spell_res = get_spells_res(new_cur_spells, indent)

    (new_me, new_boss) = apply_spells(spell_res, new_me, new_boss)

    # check for boss death
    if new_boss['hp'] <= 0:
        if output or outputLoss:
            print(indent, ' ! boss loses', 'player =', new_me['hp'], 'spend =', cur_spend)
        manawins.append(cur_spend)
        if cur_spend < min_mana_win:
            min_mana_win = cur_spend
            #print(indent, ' ! boss loses', 'player =', me['hp'], 'spend =', cur_spend)
            print('new min', cur_spend)
        return

    if output:
        print(indent, 'Boss attacked for {} damage!'.format(new_boss['damage'] - new_me['armor']))
    new_me['hp'] -= max(1, new_boss['damage'] - new_me['armor'])

    # check for player death
    if new_me['hp'] <= 0:
        if output:
            print(indent, ' @ player loses', 'boss =', new_boss['hp'])
        return

    # loop through spells not in new_cur_spells, call run_game for each
    for next_sp in spells:
        if next_sp['t'] in new_cur_spells:
            continue

        run_game(new_me, new_boss, next_sp, new_cur_spells, cur_spend, depth + 1)


spells = []
spells.append({'t': 'Poison', 'cost': 173, 'damage': 3, 'timer': 6, 'otimer': 6})
spells.append({'t': 'Recharge', 'cost': 229, 'mana': 101, 'timer': 5, 'otimer': 5})
spells.append({'t': 'Magic Missle', 'cost': 53, 'damage': 4})
spells.append({'t': 'Drain', 'cost': 73, 'damage': 2, 'hp': 2})
spells.append({'t': 'Sheild', 'cost': 113, 'armor': 7, 'timer': 6, 'otimer': 6})

manawins = []
min_mana_win = 999999999

# boss
# Hit Points: 55
# Damage: 8

# my hp = 50
# mana = 500

# test 1
# me = {'hp': 10, 'mana': 250, 'armor': 0, 'damage': 0}
# boss = {'hp': 13, 'mana': 0, 'armor': 0, 'damage': 8}

# test 2
# me = {'hp': 10, 'mana': 250, 'armor': 0, 'damage': 0}
# boss = {'hp': 14, 'mana': 0, 'armor': 0, 'damage': 8}

# part 1
me = {'hp': 50, 'mana': 500, 'armor': 0, 'damage': 0}
boss = {'hp': 55, 'mana': 0, 'armor': 0, 'damage': 8}
hp_turn_cost = 0

# part 2
hp_turn_cost = 1

output = False
outputLoss = False

for next_sp in spells:
    print('starting with', next_sp)
    run_game(me, boss, next_sp, {}, 0, 0)

#print('manawins', manawins)
print('manawins', min(manawins))