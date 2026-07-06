use crate::{Class::{Earth, Water, Fire, Air}, Skill::{Magnitude, HealingWave, FireBurst, Typhoon}};


//enums
enum Class {
    Water,
    Fire,
    Earth,
    Air,
}

enum EnemyType {
    Goblin,
    Orc,
    Skeleton,
    Zombie,
}

enum Skill {
    HealingWave,
    FireBurst,
    Magnitude,
    Typhoon,
}

//structs
struct Player {
    name: String,
    class: Class,
    hp: u32,
    atk: u32,
    def: u32,
    spd: u32,
    skill: Skill,
}

struct Enemy {
    name: String,
    species: EnemyType,
    hp: u32,
    atk: u32,
    def: u32,
    spd: u32,
}

//player methods
impl Player {
    fn new(name: String, class: Class) -> Self{
        match class {
            Class::Water => Player {name, class, hp:40, atk:33, def:35, spd:42, skill:HealingWave},
            Class::Fire => Player {name, class, hp:28, atk:50, def:24, spd:48, skill: FireBurst},
            Class::Earth => Player {name, class, hp:50, atk:31, def:45, spd:24, skill: Magnitude},
            Class::Air => Player {name, class, hp:30, atk:38, def:22, spd:60, skill: Typhoon},
        }
    }

    fn atk(&self) -> u32 {
        self.atk
    }

    fn take_dmg(&mut self, damage:u32) {
        let act_dmg = damage - (self.def / 2);
        if act_dmg <= 0 {
            self.hp -= 1;
        }
        self.hp -= act_dmg;
    }

}


//enemy methods
impl Enemy {
    fn new(isa: EnemyType) -> Self {
        match isa{
            EnemyType::Goblin => Enemy {name: "Goblin".to_string(),
            species: EnemyType::Goblin,
            hp: 24,
            atk: 30,
            def: 22,
            spd: 44,
            },

            EnemyType::Orc => Enemy {
            name: "Orc".to_string(),
            species: EnemyType::Orc,
            hp: 38,
            atk: 36,
            def: 30,
            spd: 16,
            },

            EnemyType::Skeleton => Enemy {
            name: "Skeleton".to_string(),
            species: EnemyType::Skeleton,
            hp: 26,
            atk: 34,
            def: 20,
            spd: 40,
            },

            EnemyType::Zombie => Enemy {
            name: "Zombie".to_string(),
            species: EnemyType::Zombie,
            hp: 42,
            atk: 26,
            def: 34,
            spd: 18,
            },
        }
    }

    fn atk(&self)-> u32 {
        self.atk
    }

    fn take_dmg(&mut self, damage: u32) {
        let act_dmg = damage - (self.def / 2);
        if act_dmg <= 0 {
            self.hp -= 1;
        }
        self.hp -= act_dmg;
    }
}


//main function
fn main() {
    let mut player = Player::new("River".to_string(), Water);

    let mut enemy = Enemy::new(EnemyType::Goblin);

    Enemy::take_dmg(&mut enemy, Player::atk(&player));

    println!("Enemy hp :{}", enemy.hp);

    Player::take_dmg(&mut player, Enemy::atk(&enemy));

    println!("River hp : {}", player.hp);
}