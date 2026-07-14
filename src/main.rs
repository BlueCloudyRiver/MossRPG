use crate::{Class::{Earth, Water, Fire, Air}, Skill::{Magnitude, HealingWave, FireBurst, Typhoon}};
use std::io;

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

    fn take_dmg(&mut self, damage: u32) {
        if damage < self.def/2 {
            self.hp = self.hp.saturating_sub(1);
            return
        }

        let act_dmg = damage - (self.def / 2);
        self.hp = self.hp.saturating_sub(act_dmg);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn skill(&mut self) -> u32 {
        match self.skill{
            HealingWave => {
                self.hp += 15;
                println!("You dealt 30dmg and restored 15 hp");
                30
            }
            FireBurst => {
                println!("You dealt 40dmg + {}extra", self.atk/8);
                40 + self.atk/8
            }
            Magnitude => {
                self.def += 15;
                println!("You dealt 35dmg and buffed your defense");
                35
            }
            Typhoon => {
                self.spd += 10;
                println!("You dealt 30dmg and buffed your speed");
                30
            }
        }
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
        if damage < self.def/2 {
            self.hp = self.hp.saturating_sub(1);
            return
        }

        let act_dmg = damage - (self.def / 2);
        self.hp = self.hp.saturating_sub(act_dmg);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}


//main function
fn main() {
    let mut name = String::new();

    println!("Enter the name for your character");
    std::io::stdin()
        .read_line(&mut name)
        .unwrap();

    let mut input = String::new();

    println("
    Pick a class (Enter the number)
    1. Water
    2. Earth
    3. Air
    4. Fire
    ");
    
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();

    let class = &input.trim().to_string();

    let class = match class {
        "1" => Class::Water,
        "2" => Class::Earth,
        "3" => Class::Air,
        "4" => Class::Fire,
        _ => {
            println!("Invalid input, defaulting to Earth");
            Class::Earth
        },
    }

    let player = Player::new(name, class);

    println!("A wild goblin has appeared!");
    let mut enemy = Enemy::new(EnemyType::Goblin);

    while player.is_alive() && enemy.is_alive() {
        println!("===============");
        println!("Your hp: {}", player.hp);
        println!("Enemy's hp: {}", enemy.hp);

        println!("
        1. Attack
        2. Skill
        3. Defend
        ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim();
        match choice {
            "1" => enemy.take_dmg(player.atk()),
            "2" => enemy.take_dmg(player.skill()), 
            "3" => player.take_dmg(enemy.atk()),  
            _ => println!("Invalid input"),      
        }

    }
}