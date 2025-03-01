use wasm_bindgen::prelude::*;

struct Tree {
    nodes: [Node; 65],
}

#[derive(Copy, Clone)]
struct Node {
    pub val: &'static str,
    options: [(Option<&'static str>, usize); 3],
}

#[wasm_bindgen(getter_with_clone)]
pub struct JSNode {
    pub val: String,
    pub options: Vec<NodeOption>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct NodeOption(pub String, pub usize);

impl From<Node> for JSNode {
    fn from(node: Node) -> Self {
        let options = node
            .options
            .into_iter()
            .filter_map(|op| {
                if let Some(a) = op.0.map(|s| (s.to_string(), op.1)) {
                    Some(NodeOption(a.0, a.1))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Self {
            val: node.val.to_string(),
            options,
        }
    }
}

impl Node {
    const fn default() -> Self {
        Self {
            val: "",
            options: [(None, 0); 3],
        }
    }
}

macro_rules! node {
    ($val:literal) => {
        Node {
            val: $val,
            options: [(None, 0); 3],
        }
    };
    ($val:expr, ($op1:expr, $i1:expr)) => {
        Node {
            val: $val,
            options: [(Some($op1), $i1), (None, 0), (None, 0)],
        }
    };
    ($val:expr, ($op1:expr, $i1:expr), ($op2:expr, $i2:expr)) => {
        Node {
            val: $val,
            options: [(Some($op1), $i1), (Some($op2), $i2), (None, 0)],
        }
    };
    ($val:expr, ($op1:expr, $i1:expr), ($op2:expr, $i2:expr), ($op3:expr, $i3:expr)) => {
        Node {
            val: $val,
            options: [(Some($op1), $i1), (Some($op2), $i2), (Some($op3), $i3)],
        }
    };
}

const fn build_tree() -> Tree {
    let mut nodes = [Node::default(); 65];

    nodes[0] = node!(
        "Firstly, do you want to look good?",
        ("No, I want to wear mail.", 1),
        ("Yes", 11)
    );
    nodes[1] = node!(
        "Do you want a pet, or to be petted?",
        ("Neither, I want to be alone.", 2),
        ("I want a pet.", 3),
        ("Pet me", 6)
    );
    nodes[2] = node!("Hunter (MM)");
    nodes[3] = node!("Do you exist?", ("Yes", 4), ("No", 5));
    nodes[4] = node!("Hunter (BM)");
    nodes[5] = node!("Hunter (Survival)");
    nodes[6] = node!(
        "Do you want to make the ladies wet? (with water)",
        ("Hell yeah", 7),
        ("Nah", 8)
    );
    nodes[7] = node!("Shaman (Resto)");
    nodes[8] = node!(
        "Have you ever found something you felt a primal urge to destroy it?",
        ("No", 9),
        ("Yes", 10)
    );
    nodes[9] = node!("Shaman (Ele)");
    nodes[10] = node!("Shaman (Enh)");
    nodes[11] = node!(
        "Are you okay with wearing a skirt?",
        ("Fuck no", 12),
        ("It's a robe...", 13),
        ("I'd rather not...", 29)
    );
    nodes[12] = node!("Warrior (Arms");
    nodes[13] = node!(
        "Do you know this song: \"Caught here in a fiery blaze, won't lose my will to stay\"?",
        ("Nah", 14),
        ("Yes, I love Avenged Sevenfold", 28)
    );
    nodes[14] = node!(
        "Do you ultimately just want to feel appreciated?",
        ("Yeah :(", 15),
        ("No, I crave violence", 18)
    );
    nodes[15] = node!(
        "Is prevention better than cure?",
        ("Yes", 16),
        ("Yeah, but I want my party to notice I'm there :(", 17)
    );
    nodes[16] = node!("Priest (Disc)");
    nodes[17] = node!("Priest (Holy)");
    nodes[18] = node!(
        "Do you want to inflict your enemies with spells that do damage over time?",
        ("Not in particular", 19),
        ("Yes", 25)
    );
    nodes[19] = node!(
        "Do you want to summon some big friends?",
        ("Yes >:)", 20),
        ("No", 21),
        (
            "I want the ability to do so, but then I specifically want to choose a talent so that I can be lonely",
            24
        )
    );
    nodes[20] = node!("Warlock (Demonology)");
    nodes[21] = node!(
        "Do you still play with the candle on the table when you go to a restaurant?",
        ("Yes", 22),
        ("No", 23)
    );
    nodes[22] = node!("Mage (Fire)");
    nodes[23] = node!("Mage (Arcane)");
    nodes[24] = node!("Mage (Frost)");
    nodes[25] = node!("Do you want to be any good at it?", ("Yes", 26), ("No", 27));
    nodes[26] = node!("Warlock (Affliction)");
    nodes[27] = node!("Priest (Shadow)");
    nodes[28] = node!("Warlock (Destro)");
    nodes[29] = node!(
        "Be honest, have you died before?",
        ("I'm still standing!", 30),
        ("Well, there was that one time...", 60)
    );
    nodes[30] = node!(
        "Did you notice the Elton John reference?",
        ("Yes", 31),
        ("Nah", 32)
    );
    nodes[31] = node!("Paladin (Retribution)");
    nodes[32] = node!(
        "Do you feel a deep connection to nature?",
        ("I prefer astronomy (and chickens)", 33),
        ("Nah", 34),
        ("Yes", 55)
    );
    nodes[33] = node!("Druid (Balance)");
    nodes[34] = node!(
        "Would you rather cha cha slide or jump and glide?",
        ("Cha Cha Slide", 35),
        ("Jump + Glide", 52)
    );
    nodes[35] = node!(
        "Have you ever gamed while drunk?",
        ("Always", 36),
        ("Never", 37),
        ("A few times I guess", 38)
    );
    nodes[36] = node!("Monk (BM)");
    nodes[37] = node!("Paladin (Holy)");
    nodes[38] = node!("Are you a sadist?", ("No", 39), ("Yes", 51));
    nodes[39] = node!(
        "Are you a fan of the occasional sea shanty?",
        ("Not really", 40),
        ("Yes", 50)
    );
    nodes[40] = node!(
        "Do you want to kick people?",
        ("Not really", 41),
        ("Yes", 46),
        ("Yes, but I also want fast queue times", 49)
    );
    nodes[41] = node!("Protecc or Attacc?", ("Protecc", 42), ("Attacc", 45));
    nodes[42] = node!(
        "Do you want to use magic to heal and protect yourself?",
        ("That's what the healer is for!", 43),
        ("Sure", 44)
    );
    nodes[43] = node!("Warrior (Protection)");
    nodes[44] = node!("Paladin (Protection)");
    nodes[45] = node!("Warrior (Fury)");
    nodes[46] = node!(
        "Do you wear a lot of hoodies?",
        ("Yes", 47),
        ("The normal amount I guess?", 48)
    );
    nodes[47] = node!("Rogue (Assassin)");
    nodes[48] = node!("Monk (WW)");
    nodes[49] = node!("Monk (MW)");
    nodes[50] = node!("Rogue (Outlaw)");
    nodes[51] = node!("Rogue (Subtlety)");
    nodes[52] = node!(
        "Do you live by the mantra \"live fast, die young\"?",
        ("Yeah!", 53),
        ("No", 54)
    );
    nodes[53] = node!("Demon Hunter (Havoc)");
    nodes[54] = node!("Demon Hunter (Vengeance)");
    nodes[55] = node!(
        "Do you wanna grow some mushrooms?",
        ("Yeah bro", 56),
        ("Nah", 57)
    );
    nodes[56] = node!("Druid (Resto)");
    nodes[57] = node!(
        "Do you want people to kick you out of groups for no reason?",
        ("No", 58),
        ("LOL yeah", 59)
    );
    nodes[58] = node!("Druid (Guardian)");
    nodes[59] = node!("Druid (Feral)");
    nodes[60] = node!(
        "Did you ever have a phase where you were really into norse mythology?",
        ("Yes", 61)
    );
    nodes[61] = node!("Death Knight (Frost)");
    nodes[62] = node!("Are you okay?", ("Yep", 63), ("No :(", 64));
    nodes[63] = node!("Death Knight (Blood)");
    nodes[64] = node!("Death Knight (Unholy)");

    Tree { nodes }
}

const QUIZ_TREE: Tree = build_tree();

#[wasm_bindgen]
pub fn get_root() -> JSNode {
    QUIZ_TREE.nodes[0].into()
}

#[wasm_bindgen]
pub fn get_node(index: usize) -> JSNode {
    QUIZ_TREE.nodes[index].into()
}
