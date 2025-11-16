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
    "acidic",
    "aged",
    "agile",
    "airy",
    "amber",
    "ancient",
    "angry",
    "animated",
    "aqua",
    "aquamarine",
    "arctic",
    "aromatic",
    "atomic",
    "autumn",
    "azure",
    "balanced",
    "balmy",
    "beige",
    "black",
    "blazing",
    "blue",
    "bold",
    "bouncy",
    "breezy",
    "bright",
    "brilliant",
    "brisk",
    "brittle",
    "bronze",
    "brown",
    "bubbling",
    "bubbly",
    "buoyant",
    "buttery",
    "buzzy",
    "calm",
    "candid",
    "caramel",
    "celestial",
    "charcoal",
    "cheerful",
    "cheery",
    "chewy",
    "chilly",
    "chrome",
    "citrus",
    "citrusy",
    "clean",
    "clear",
    "clever",
    "cloudless",
    "cloudy",
    "cobalt",
    "cold",
    "colorful",
    "compact",
    "cooked",
    "cool",
    "copper",
    "coral",
    "cream",
    "creamy",
    "crimson",
    "crisp",
    "crumbly",
    "crunchy",
    "crusty",
    "crystal",
    "curious",
    "curvy",
    "daring",
    "dashing",
    "dazzling",
    "deft",
    "dense",
    "dew",
    "dim",
    "downy",
    "dreamy",
    "dry",
    "dusky",
    "dusty",
    "dynamic",
    "eager",
    "earthy",
    "ebony",
    "electric",
    "emerald",
    "energetic",
    "fearless",
    "feathery",
    "fierce",
    "fiery",
    "flaky",
    "flavorful",
    "fleet",
    "fluffy",
    "foggy",
    "fragrant",
    "fresh",
    "frosty",
    "gentle",
    "giant",
    "gilded",
    "gleaming",
    "gleeful",
    "glimmering",
    "glinting",
    "glittering",
    "glossy",
    "glowing",
    "gold",
    "golden",
    "gooey",
    "grand",
    "gray",
    "green",
    "gritty",
    "happy",
    "hazel",
    "heavy",
    "herbaceous",
    "heroic",
    "honeyed",
    "hot",
    "huge",
    "humming",
    "icy",
    "immediate",
    "indigo",
    "intrepid",
    "ivory",
    "jade",
    "jazzy",
    "juicy",
    "keen",
    "lavender",
    "lemon",
    "light",
    "lime",
    "lithe",
    "little",
    "lively",
    "lucid",
    "lukewarm",
    "luminous",
    "lustrous",
    "magenta",
    "magnetic",
    "maroon",
    "massive",
    "mellow",
    "merry",
    "mighty",
    "milky",
    "misty",
    "moldy",
    "mushy",
    "navy",
    "new",
    "nimble",
    "noble",
    "noisy",
    "ochre",
    "old",
    "olive",
    "oozy",
    "optimistic",
    "orange",
    "pearl",
    "peppery",
    "peppy",
    "perfumed",
    "perky",
    "petite",
    "pink",
    "plucky",
    "plum",
    "polar",
    "polished",
    "primal",
    "prism",
    "pristine",
    "pungent",
    "pure",
    "purple",
    "quick",
    "quiet",
    "radiant",
    "rainy",
    "rapid",
    "raw",
    "red",
    "restless",
    "ripe",
    "roaring",
    "rosy",
    "round",
    "ruby",
    "rustling",
    "rusty",
    "saffron",
    "salty",
    "sandy",
    "savory",
    "scalding",
    "scarlet",
    "sepia",
    "serene",
    "shadowy",
    "shimmering",
    "shiny",
    "silent",
    "silken",
    "silky",
    "silver",
    "sleek",
    "sleepy",
    "slender",
    "slippery",
    "small",
    "smelly",
    "smoky",
    "smooth",
    "snappy",
    "snowy",
    "soggy",
    "solar",
    "solid",
    "soothing",
    "sparkling",
    "sparkly",
    "speedy",
    "spiced",
    "spicy",
    "spirited",
    "sprightly",
    "sprinting",
    "spry",
    "square",
    "stale",
    "steadfast",
    "steamy",
    "stellar",
    "sticky",
    "stinky",
    "stormy",
    "succulent",
    "sunlit",
    "sunny",
    "sweet",
    "sweltering",
    "swift",
    "syrupy",
    "tangy",
    "tart",
    "teal",
    "teeny",
    "tender",
    "thundering",
    "tidy",
    "tiny",
    "toasty",
    "tropical",
    "turquoise",
    "twinkling",
    "vast",
    "vibrant",
    "violet",
    "vivid",
    "warm",
    "whimsical",
    "whirring",
    "white",
    "wide",
    "wild",
    "wintry",
    "witty",
    "wrinkly",
    "yellow",
    "zealous",
    "zesty",
    "zippy",
];

const FOOD_WORDS: WordLists = WordLists {
    nouns: &[
        "acai",
        "almond",
        "amberjack",
        "anchovy",
        "apple",
        "apricot",
        "artichoke",
        "arugula",
        "asparagus",
        "avocado",
        "bacon",
        "bagel",
        "banana",
        "barracuda",
        "basil",
        "bass",
        "beef",
        "beet",
        "bilberry",
        "biscuit",
        "black cod",
        "blackberry",
        "blackcurrant",
        "blueberry",
        "bluefin",
        "bonito",
        "boysenberry",
        "bread",
        "breadfruit",
        "brisket",
        "broccoli",
        "broccolini",
        "brownie",
        "brussels",
        "bun",
        "butterfish",
        "cabbage",
        "cake",
        "candy",
        "cantaloupe",
        "caramel",
        "carrot",
        "cashew",
        "catfish",
        "cauliflower",
        "celery",
        "cereal",
        "chard",
        "cherry",
        "chicken",
        "chipotle",
        "churro",
        "clams",
        "clementine",
        "cloudberry",
        "coconut",
        "cod",
        "collard",
        "cookie",
        "couscous",
        "cranberry",
        "croissant",
        "cucumber",
        "currant",
        "curry",
        "cuttlefish",
        "date",
        "dewberry",
        "doughnut",
        "dragonfruit",
        "duck",
        "dumpling",
        "durian",
        "edamame",
        "eel",
        "eggplant",
        "elderberry",
        "falafel",
        "feijoa",
        "fennel",
        "fig",
        "fingerlime",
        "flounder",
        "fondue",
        "garlic",
        "ginger",
        "goji",
        "gooseberry",
        "granola",
        "grape",
        "grapefruit",
        "grouper",
        "guava",
        "halibut",
        "ham",
        "hazelnut",
        "herring",
        "honey",
        "honeydew",
        "huckleberry",
        "jackfruit",
        "jelly",
        "jujube",
        "kale",
        "kimchi",
        "kingfish",
        "kiwi",
        "kiwifruit",
        "kumquat",
        "lamb",
        "lasagna",
        "leek",
        "lemon",
        "lentil",
        "lettuce",
        "lime",
        "lingonberry",
        "lobster",
        "longan",
        "loquat",
        "lychee",
        "mackerel",
        "mahi mahi",
        "mandarin",
        "mango",
        "mangosteen",
        "marionberry",
        "marlin",
        "marshmallow",
        "miracleberry",
        "miso",
        "mochi",
        "muffin",
        "mulberry",
        "mussels",
        "mutton",
        "nectarine",
        "noodle",
        "nutmeg",
        "octopus",
        "okra",
        "olive",
        "omelet",
        "onion",
        "orange",
        "oyster",
        "pancake",
        "papaya",
        "parsnip",
        "passionfruit",
        "pasta",
        "peach",
        "peanut",
        "pear",
        "pepper",
        "perch",
        "persimmon",
        "pickle",
        "pie",
        "pike",
        "pineapple",
        "pistachio",
        "pizza",
        "plantain",
        "plum",
        "pollock",
        "pomegranate",
        "pomelo",
        "pork",
        "potato",
        "prawn",
        "pretzel",
        "prune",
        "quinoa",
        "radish",
        "raisin",
        "ramen",
        "raspberry",
        "redcurrant",
        "risotto",
        "rockfish",
        "rutabaga",
        "sablefish",
        "salami",
        "salmon steak",
        "salmonberry",
        "salsa",
        "sardine",
        "satsuma",
        "sausage",
        "scallion",
        "scallop",
        "sesame",
        "shallot",
        "shrimp",
        "snapper",
        "sole",
        "sorbet",
        "soy",
        "spaghetti",
        "spinach",
        "squash",
        "squid",
        "starfruit",
        "steak",
        "steelhead",
        "stew",
        "strawberry",
        "sturgeon",
        "sugarapple",
        "sundae",
        "sushi",
        "taco",
        "tamarind",
        "tangerine",
        "tilapia",
        "toffee",
        "tomato",
        "truffle",
        "tuna steak",
        "turbot",
        "turkey",
        "turnip",
        "veal",
        "venison",
        "waffle",
        "walnut",
        "watermelon",
        "waxapple",
        "whitefish",
        "wintermelon",
        "yam",
        "yogurt",
        "youngberry",
        "yumberry",
        "zinfandel",
        "zucchini",
    ],
};

const SCIFI_WORDS: WordLists = WordLists {
    nouns: &[
        "android",
        "anomaly",
        "aperture",
        "asteroid",
        "asteroid belt",
        "astral plane",
        "astronaut",
        "aurora",
        "beacon",
        "binary star",
        "black hole",
        "blaster",
        "blue giant",
        "capsule",
        "citadel",
        "comet",
        "constellation",
        "cosmic dust",
        "cosmic ray",
        "cosmos",
        "cruiser",
        "cyborg",
        "dark energy",
        "dark matter",
        "deep space",
        "deep space probe",
        "droid",
        "dwarf planet",
        "eclipse",
        "engine",
        "enigma",
        "event horizon",
        "exoplanet",
        "falcon",
        "frontier",
        "fusion",
        "galaxy",
        "gamma ray",
        "gas giant",
        "gaseous mass",
        "globular cluster",
        "gravity well",
        "heliosphere",
        "hovercraft",
        "hyperdrive",
        "hypergiant",
        "ice giant",
        "interstellar medium",
        "ion",
        "ion storm",
        "kepler",
        "kuiper belt",
        "laser cannon",
        "launch window",
        "launchpad",
        "light speed",
        "lunar base",
        "magnetar",
        "magnetosphere",
        "mass driver",
        "meteor",
        "meteor shower",
        "meteor storm",
        "meteorite",
        "microgravity",
        "module",
        "mothership",
        "nebula",
        "neutron",
        "nova",
        "observatory",
        "open cluster",
        "orbital platform",
        "orbiter",
        "outpost",
        "phantom",
        "phase",
        "photon",
        "photon belt",
        "pioneer",
        "planetary nebula",
        "planetfall",
        "plasma",
        "portal",
        "probe",
        "protoplanet",
        "protostar",
        "pulsar",
        "quantum",
        "quasar",
        "radio telescope",
        "ranger",
        "reactor",
        "red dwarf",
        "red giant",
        "ring system",
        "rocket",
        "rogue planet",
        "satellite",
        "scout",
        "ship",
        "shuttle",
        "singularity",
        "solar flare",
        "solar wind",
        "solstice",
        "space elevator",
        "space probe",
        "space station",
        "space telescope",
        "space-time",
        "spectrum",
        "speeder",
        "star",
        "star chart",
        "star cluster",
        "star forge",
        "star gate",
        "star map",
        "starbase",
        "starlight",
        "starship",
        "station",
        "stellar nursery",
        "supergiant",
        "supernova",
        "terrestrial planet",
        "thruster",
        "transponder",
        "transporter",
        "triple star",
        "ufo",
        "vector",
        "warp",
        "wayfinder",
        "waypoint",
        "white dwarf",
        "wing",
        "wormhole",
        "xenobot",
        "xenon",
        "zenith",
        "zephyr",
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
