// Learn more about F# at http://fsharp.org

open System

type Player = {
    activeSpells: (int * Spell) list;
    mana: int;
    health: int;
    armor: int;
}

and Boss = {
    health: int;
    damage: int;
}

and Spell = {
    name: string;
    cost: int;
    duration: int;
    instantEffect: (Player * Boss) -> (Player * Boss)
    ongoingEffect: (Player * Boss) -> (Player * Boss)
    endEffect: (Player * Boss) -> (Player * Boss)
}

let doBossDamage damage (player, boss: Boss) =
    player, { boss with health = boss.health - damage }

let modifyPlayerHealth health op (player: Player,boss) =
    { player with health = op player.health health }, boss

let modifyArmor armor op (player: Player,boss) =
    { player with armor = op player.armor armor }, boss

let modifyMana mana op (player: Player,boss) =
    { player with mana = op player.mana mana }, boss

let spells = [
    { name = "Magic Missile"; cost = 53; duration = 0; instantEffect = doBossDamage 4; endEffect = id; ongoingEffect = id; }
    { name = "Drain"; cost = 73; duration = 0; instantEffect = doBossDamage 2 >> modifyPlayerHealth 2 (+); endEffect = id; ongoingEffect = id; }
    { name = "Shield"; cost = 113; duration = 6; instantEffect = modifyArmor 7 (+); endEffect = modifyArmor 7 (-); ongoingEffect = id; }
    { name = "Poison"; cost = 173; duration = 6; instantEffect = id; endEffect = id; ongoingEffect = doBossDamage 3; }
    { name = "Recharge"; cost = 229; duration = 5; instantEffect = id; endEffect = id; ongoingEffect = modifyMana 101 (+); }
]

let addActiveSpell spell (player, boss) =
    if spell.duration = 0
    then player, boss
    else { player with activeSpells = (spell.duration, spell)::player.activeSpells }, boss

let applySpell (player,boss) (remain, spell) =
    let (player, boss) = spell.ongoingEffect (player,boss)
    let newRemain = remain - 1

    if newRemain <= 0
    then
        spell.endEffect (player, boss)
    else { player with activeSpells = (newRemain, spell)::player.activeSpells }, boss

let applySpellEffects player boss =
    player.activeSpells
    |> List.fold applySpell ({ player with activeSpells = [] }, boss)

let getValidSpells player =
    spells
    |> List.filter (fun s -> player.activeSpells |> List.exists (fun (_,a) -> a.name = s.name) |> not)
    |> List.filter (fun s -> player.mana >= s.cost)

let castSpell spell =
    modifyMana spell.cost (-) >> spell.instantEffect >> addActiveSpell spell

let handleRoundTwo isRoundTwo (player, boss) =
    if isRoundTwo
    then modifyPlayerHealth 1 (-) (player, boss)
    else (player, boss)


let doRound (player, boss) spell spentMana isRoundTwo =
    let (player, boss) = handleRoundTwo isRoundTwo (player, boss)
    if player.health <= 0
    then (player, boss, spentMana)
    else
        let (player, boss) = castSpell spell (player, boss)
        let spentMana = spentMana + spell.cost

        if boss.health <= 0
        then (player, boss, spentMana)
        else
            let (player, boss) = applySpellEffects player boss

            if boss.health <= 0
            then (player, boss, spentMana)
            else
                let damage = max 1 (boss.damage - player.armor)
                let (player, boss) = modifyPlayerHealth damage (-) (player, boss)

                if player.health <= 0
                then (player, boss, spentMana)
                else
                    let (player, boss) = applySpellEffects player boss
                    (player, boss, spentMana)

let rec findMinimumMana minimumMana spentMana (player: Player) boss isRoundTwo =
    if (spentMana - 100) > minimumMana
    then minimumMana
    else
    if boss.health <= 0
    then (min minimumMana spentMana)
    else
    if player.health <= 0
    then minimumMana
    else
        let validSpells = getValidSpells player
        if validSpells.IsEmpty
        then minimumMana
        else
            validSpells
            |> List.fold (fun minMana spell ->
                let (player, boss, spentMana) = doRound (player, boss) spell spentMana isRoundTwo

                if boss.health <= 0
                then (min minMana spentMana)
                else findMinimumMana minMana spentMana player boss isRoundTwo
            ) minimumMana

[<EntryPoint>]
let main argv =
    let player = {
        health = 50;
        mana = 500;
        armor = 0;
        activeSpells = [];
    }

    let boss = {
        health = 58;
        damage = 9;
    }

    let part1 = findMinimumMana Int32.MaxValue 0 player boss false
    printfn "Part 1 %d" part1
    let part2 = findMinimumMana Int32.MaxValue 0 player boss true
    printfn "Part 2 %d" part2
    0 // return an integer exit code
