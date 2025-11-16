#![forbid(unsafe_code)]

use core::cell::RefCell;
use core::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

thread_local! {
    static GLOBAL_RNG: RefCell<TinyRng> = RefCell::new(TinyRng::seed_from_entropy());
}

static ENTROPY_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Randomly select an adjective + food word and return them in Title Case (e.g. `Shiny Mango`).
pub fn random_food_name() -> String {
    random_name(&FOOD_WORDS)
}

/// Randomly select an adjective + sci-fi word and return them in Title Case (e.g. `Nebulous Rocket`).
pub fn random_scifi_name() -> String {
    random_name(&SCIFI_WORDS)
}

/// Return the raw adjective + noun pair for the food generator.
pub fn random_food_words() -> NamePair {
    random_pair(&FOOD_WORDS)
}

/// Return the raw adjective + noun pair for the sci-fi generator.
pub fn random_scifi_words() -> NamePair {
    random_pair(&SCIFI_WORDS)
}

/// Deterministic generator that can be seeded manually for reproducible output.
#[derive(Clone)]
pub struct NameGenerator {
    rng: TinyRng,
}

impl NameGenerator {
    /// Create a generator that is automatically seeded with best-effort entropy.
    pub fn new() -> Self {
        Self {
            rng: TinyRng::seed_from_entropy(),
        }
    }

    /// Create a generator from a fixed 64-bit seed.
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: TinyRng::from_seed(seed),
        }
    }

    /// Get a food-themed adjective + noun pair.
    pub fn food_words(&mut self) -> NamePair {
        select_pair(&FOOD_WORDS, &mut self.rng)
    }

    /// Get a sci-fi-themed adjective + noun pair.
    pub fn scifi_words(&mut self) -> NamePair {
        select_pair(&SCIFI_WORDS, &mut self.rng)
    }

    /// Convenience helper that returns a formatted food name (Title Case with a space).
    pub fn food_name(&mut self) -> String {
        self.food_words().title_case()
    }

    /// Convenience helper that returns a formatted sci-fi name (Title Case with a space).
    pub fn scifi_name(&mut self) -> String {
        self.scifi_words().title_case()
    }
}

impl Default for NameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Raw adjective + noun pair.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NamePair {
    pub adjective: &'static str,
    pub noun: &'static str,
}

impl NamePair {
    /// Render the pair as `Titlecase Titlecase`.
    pub fn title_case(&self) -> String {
        let mut text = String::with_capacity(self.adjective.len() + self.noun.len() + 1);
        push_title_case(self.adjective, &mut text);
        text.push(' ');
        push_title_case(self.noun, &mut text);
        text
    }
}

fn random_name(list: &WordLists) -> String {
    random_pair(list).title_case()
}

fn random_pair(list: &WordLists) -> NamePair {
    GLOBAL_RNG.with(|rng| select_pair(list, &mut *rng.borrow_mut()))
}

fn select_pair(words: &WordLists, rng: &mut TinyRng) -> NamePair {
    let adjective = ADJECTIVES[rng.index(ADJECTIVES.len())];
    let noun = words.nouns[rng.index(words.nouns.len())];
    NamePair { adjective, noun }
}

fn push_title_case(word: &str, buf: &mut String) {
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        for ch in first.to_uppercase() {
            buf.push(ch);
        }
        for c in chars {
            for lower in c.to_lowercase() {
                buf.push(lower);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct TinyRng {
    state: u64,
}

impl TinyRng {
    fn seed_from_entropy() -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        let extra = ENTROPY_COUNTER.fetch_add(0x9E37, Ordering::Relaxed);
        Self::from_seed(time ^ extra ^ extra.rotate_left(32))
    }

    fn from_seed(seed: u64) -> Self {
        let state = if seed == 0 { 0x4d595df4d0f33173 } else { seed };
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn index(&mut self, upper: usize) -> usize {
        let bound = upper as u64;
        if bound == 0 {
            return 0;
        }
        (self.next_u64() % bound) as usize
    }
}

struct WordLists {
    nouns: &'static [&'static str],
}

const ADJECTIVES: &[&str] = &[
    "agile",
    "amber",
    "atomic",
    "bold",
    "brilliant",
    "bubbly",
    "calm",
    "candid",
    "cheerful",
    "citrus",
    "clever",
    "crimson",
    "crisp",
    "curious",
    "daring",
    "dazzling",
    "dynamic",
    "eager",
    "electric",
    "emerald",
    "fearless",
    "fiery",
    "gentle",
    "gilded",
    "gleaming",
    "glimmering",
    "glowing",
    "golden",
    "grand",
    "hazel",
    "heroic",
    "icy",
    "immediate",
    "intrepid",
    "jazzy",
    "keen",
    "lively",
    "lucid",
    "luminous",
    "magnetic",
    "mellow",
    "mighty",
    "misty",
    "noble",
    "optimistic",
    "peppy",
    "polar",
    "primal",
    "prism",
    "quick",
    "quiet",
    "radiant",
    "rapid",
    "restless",
    "rosy",
    "serene",
    "shiny",
    "silent",
    "silver",
    "sleek",
    "snappy",
    "solar",
    "spirited",
    "spry",
    "steadfast",
    "stellar",
    "swift",
    "tidy",
    "vibrant",
    "vivid",
    "whimsical",
    "wild",
    "witty",
    "zealous",
    "zesty",
];

const FOOD_WORDS: WordLists = WordLists {
    nouns: &[
        "acai",
        "almond",
        "apple",
        "apricot",
        "avocado",
        "bagel",
        "basil",
        "biscuit",
        "bread",
        "broccoli",
        "brownie",
        "bun",
        "cabbage",
        "cake",
        "candy",
        "cantaloupe",
        "caramel",
        "carrot",
        "cashew",
        "cereal",
        "cherry",
        "chipotle",
        "churro",
        "coconut",
        "cookie",
        "couscous",
        "croissant",
        "curry",
        "doughnut",
        "dumpling",
        "edamame",
        "falafel",
        "fig",
        "fondue",
        "ginger",
        "granola",
        "grape",
        "guava",
        "hazelnut",
        "honey",
        "jelly",
        "kale",
        "kimchi",
        "kiwi",
        "lasagna",
        "lemon",
        "lentil",
        "lime",
        "mango",
        "marshmallow",
        "miso",
        "mochi",
        "muffin",
        "nectarine",
        "noodle",
        "nutmeg",
        "olive",
        "omelet",
        "pancake",
        "papaya",
        "pasta",
        "peach",
        "peanut",
        "pepper",
        "pickle",
        "pie",
        "pineapple",
        "pistachio",
        "pizza",
        "plum",
        "pretzel",
        "quinoa",
        "raisin",
        "ramen",
        "risotto",
        "salsa",
        "sesame",
        "sorbet",
        "soy",
        "spaghetti",
        "squash",
        "stew",
        "sundae",
        "sushi",
        "taco",
        "tamarind",
        "tangerine",
        "toffee",
        "tomato",
        "truffle",
        "udon",
        "waffle",
        "walnut",
        "yogurt",
    ],
};

const SCIFI_WORDS: WordLists = WordLists {
    nouns: &[
        "anomaly",
        "android",
        "aperture",
        "asteroid",
        "astronaut",
        "aurora",
        "beacon",
        "blaster",
        "capsule",
        "citadel",
        "comet",
        "constellation",
        "cosmos",
        "cruiser",
        "cyborg",
        "droid",
        "eclipse",
        "engine",
        "enigma",
        "falcon",
        "frontier",
        "fusion",
        "galaxy",
        "hovercraft",
        "hyperdrive",
        "ion",
        "kepler",
        "launchpad",
        "magnetar",
        "meteor",
        "module",
        "mothership",
        "nebula",
        "neutron",
        "nova",
        "observatory",
        "orbiter",
        "outpost",
        "phantom",
        "phase",
        "photon",
        "pioneer",
        "plasma",
        "portal",
        "probe",
        "quasar",
        "quantum",
        "ranger",
        "reactor",
        "rocket",
        "satellite",
        "scout",
        "ship",
        "shuttle",
        "singularity",
        "solstice",
        "speeder",
        "spectrum",
        "star",
        "starbase",
        "starship",
        "station",
        "starlight",
        "thruster",
        "transponder",
        "transporter",
        "ufo",
        "vector",
        "warp",
        "wayfinder",
        "waypoint",
        "wing",
        "wormhole",
        "xenobot",
        "xenon",
        "zephyr",
        "zenith",
        "zircon",
        "zodiac",
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_case_formats_correctly() {
        let pair = NamePair {
            adjective: "shiny",
            noun: "mango",
        };
        assert_eq!(pair.title_case(), "Shiny Mango");
    }

    #[test]
    fn combinations_exceed_minimums() {
        assert!(ADJECTIVES.len() * FOOD_WORDS.nouns.len() >= 1000);
        assert!(ADJECTIVES.len() * SCIFI_WORDS.nouns.len() >= 1000);
    }

    #[test]
    fn seeded_generator_is_deterministic() {
        let mut one = NameGenerator::from_seed(42);
        let mut two = NameGenerator::from_seed(42);

        for _ in 0..10 {
            assert_eq!(one.food_words(), two.food_words());
            assert_eq!(one.scifi_words(), two.scifi_words());
        }
    }

    #[test]
    fn global_functions_return_title_case() {
        let food = random_food_name();
        let scifi = random_scifi_name();

        assert!(food.chars().next().unwrap().is_uppercase());
        assert!(scifi.chars().next().unwrap().is_uppercase());
        assert!(food.contains(' '));
        assert!(scifi.contains(' '));
    }
}
