use rand::Rng;
use std::io;

// Enum to represent character classes (warrior, rogue, ranger, mage)
#[derive(PartialEq, Debug)]
enum CharacterClass {
    WARRIOR,
    ROGUE,
    RANGER,
    MAGE,
    // add classes as needed
}

#[derive(Debug)]
enum AttackType {
    STANDARD,
    RANGED,
    // add more if needed, perhaps damage types (fire, ice, nature, poison, etc.)
}

#[derive(Debug)]
struct Character {
    name: String,
    health: i32,
    attack: i32,
    defense: i32,
    level: i32,
    experience: i32,
    character_class: CharacterClass,
    attack_type: AttackType,
}

impl Character {
    // Constructor for initializing default values
    fn new() -> Self {
        Character {
            name: String::new(),
            health: 0,
            attack: 0,
            defense: 0,
            level: 1,
            experience: 0,
            character_class: CharacterClass::WARRIOR, // default values, change as needed
            attack_type: AttackType::STANDARD,
        }
    }

    // Function to display character information
    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Class: {:?}", self.character_class);
        println!("Attack Type: {:?}", self.attack_type);
        println!("Health: {}", self.health);
        println!("Attack: {}", self.attack);
        println!("Defense: {}", self.defense);
        println!("Level: {}", self.level);
        println!("Experience: {}", self.experience);
        println!();
    }
}

fn create_character() -> Character {
    let mut character = Character::new();

    println!("Enter character name:");
    io::stdin().read_line(&mut character.name).expect("Failed to read line");
    character.name = character.name.trim().to_string();

    // Randomize character classes
    let class_choice = rand::thread_rng().gen_range(0..4); // number obviously needs to be changed if more classes are added
    match class_choice {
        0 => {
            character.character_class = CharacterClass::WARRIOR;
            character.attack_type = AttackType::STANDARD;
        }
        1 => {
            character.character_class = CharacterClass::ROGUE;
            character.attack_type = AttackType::STANDARD;
        }
        2 => {
            character.character_class = CharacterClass::RANGER;
            character.attack_type = AttackType::RANGED;
        }
        3 => {
            character.character_class = CharacterClass::MAGE;
            character.attack_type = AttackType::RANGED;
        }
        // add more classes as needed
        _ => {}
    }

    character.health = rand::thread_rng().gen_range(30..=40); // random number between 30 and 40
    character.defense = rand::thread_rng().gen_range(8..=15); // random number between 8 and 15

    // set attack based on class
    if character.character_class == CharacterClass::RANGER || character.character_class == CharacterClass::MAGE {
        character.attack = rand::thread_rng().gen_range(8..=12); // random number between 8 and 12
    } else {
        // standard attack for warriors and rogues
        character.attack = rand::thread_rng().gen_range(12..=18); // random number between 12 and 18
    }

    character
}

fn ask_user_if_liked(character_name: &str) -> bool {
    let mut response = String::new();
    println!("Do you like the character {}? (y/n):", character_name);
    io::stdin().read_line(&mut response).expect("Failed to read line");
    let response = response.trim().to_lowercase();
    response == "y"
}

fn display_character_details(character: &Character) {
    println!("Name: {}", character.name);
    println!("Health: {}", character.health);
    println!("Attack: {}", character.attack);
    println!("Defense: {}", character.defense);
    println!();

    // Display the character's class
    let class_name = match character.character_class {
        CharacterClass::WARRIOR => "Warrior",
        CharacterClass::ROGUE => "Rogue",
        CharacterClass::RANGER => "Ranger",
        CharacterClass::MAGE => "Mage",
    };
    println!("Class: {}", class_name);
    println!();
}

fn main() {
    let mut characters = Vec::new();

    for _ in 0..3 {
        let mut new_character = Character::new();
        let mut user_likes_character = false;

        while !user_likes_character {
            new_character = create_character();

            // create a function to display character details
            display_character_details(&new_character);

            user_likes_character = ask_user_if_liked(&new_character.name);
        }

        characters.push(new_character);
    }

    println!("All characters created:");
    for (i, character) in characters.iter().enumerate() {
        println!("Character {}:", i + 1);
        println!("Name: {}", character.name);
        println!("Health: {}", character.health);
        println!("Attack: {}", character.attack);
        println!("Defense: {}", character.defense);
        println!();
    }
}
