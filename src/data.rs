//! This module contains the dicts for replacing our Pinyin numerical
//!
//!
//! There are 4 classes of replacement, 1, 2, 3, 4 which correspond to
//! a length of 1-4 letters.  Pinyin does not change so hard-coding these dicts
//! is fine.  We don't need to put these in JSON files for that reason.
//!
//! When searching these dicts for matches it is super important you go in order from
//! bigger dict to smaller dict.  Otherwise you'll match on subsets of the longer sequences
//! and never convert the longer ones correctly.
//!
//! In the 4 dicts the key is what we look for in the source and the value is the
//! replacement that goes into the output.

use maplit::hashmap;
use std::collections::HashMap;

pub struct PinyinData {
    pub match_map1: HashMap<&'static str, &'static str>,
    pub match_map2: HashMap<&'static str, &'static str>,
    pub match_map3: HashMap<&'static str, &'static str>,
    pub match_map4: HashMap<&'static str, &'static str>,
}

impl PinyinData {
    pub fn new() -> PinyinData {
        PinyinData {
            match_map4: hashmap! {
                // iang
                "iang1"=> "iāng",
                "iang2"=> "iáng",
                "iang3"=> "iǎng",
                "iang4"=> "iàng",
                "Iang1"=> "Iāng",
                "Iang2"=> "Iáng",
                "Iang3"=> "Iǎng",
                "Iang4"=> "Iàng",
                // iong
                "iong1"=> "iōng",
                "iong2"=> "ióng",
                "iong3"=> "iǒng",
                "iong4"=> "iòng",
                "Iong1"=> "Iōng",
                "Iong2"=> "Ióng",
                "Iong3"=> "Iǒng",
                "Iong4"=> "Iòng",
                // uang
                "uang1"=> "uāng",
                "uang2"=> "uáng",
                "uang3"=> "uǎng",
                "uang4"=> "uàng",
                "Uang1"=> "Uāng",
                "Uang2"=> "Uáng",
                "Uang3"=> "Uǎng",
                "Uang4"=> "Uàng",
                // ueng
                "ueng1"=> "uēng",
                "ueng2"=> "uéng",
                "ueng3"=> "uěng",
                "ueng4"=> "uèng",
                "Ueng1"=> "Uēng",
                "Ueng2"=> "Uéng",
                "Ueng3"=> "Uěng",
                "Ueng4"=> "Uèng",
            },
            match_map3: hashmap! {
                // iao
                "iao1"=> "iāo",
                "iao2"=> "iáo",
                "iao3"=> "iǎo",
                "iao4"=> "iào",
                "Iao1"=> "Iāo",
                "Iao2"=> "Iáo",
                "Iao3"=> "Iǎo",
                "Iao4"=> "Iào",
                // uai
                "uai1"=> "uāi",
                "uai2"=> "uái",
                "uai3"=> "uǎi",
                "uai4"=> "uài",
                "Uai1"=> "Uāi",
                "Uai2"=> "Uái",
                "Uai3"=> "Uǎi",
                "Uai4"=> "Uài",
                // ang
                "ang1"=> "āng",
                "ang2"=> "áng",
                "ang3"=> "ǎng",
                "ang4"=> "àng",
                "Ang1"=> "Āng",
                "Ang2"=> "Áng",
                "Ang3"=> "Ǎng",
                "Ang4"=> "Àng",
                // eng
                "eng1"=> "ēng",
                "eng2"=> "éng",
                "eng3"=> "ěng",
                "eng4"=> "èng",
                "Eng1"=> "Ēng",
                "Eng2"=> "Éng",
                "Eng3"=> "Ěng",
                "Eng4"=> "Èng",
                // ian
                "ian1"=> "iān",
                "ian2"=> "ián",
                "ian3"=> "iǎn",
                "ian4"=> "iàn",
                "Ian1"=> "Iān",
                "Ian2"=> "Ián",
                "Ian3"=> "Iǎn",
                "Ian4"=> "Iàn",
                // ing
                "ing1"=> "īng",
                "ing2"=> "íng",
                "ing3"=> "ǐng",
                "ing4"=> "ìng",
                "Ing1"=> "Īng",
                "Ing2"=> "Íng",
                "Ing3"=> "Ǐng",
                "Ing4"=> "Ìng",
                // ong
                "ong1"=> "ōng",
                "ong2"=> "óng",
                "ong3"=> "ǒng",
                "ong4"=> "òng",
                "Ong1"=> "Ōng",
                "Ong2"=> "Óng",
                "Ong3"=> "Ǒng",
                "Ong4"=> "Òng",
                // uan
                "uan1"=> "uān",
                "uan2"=> "uán",
                "uan3"=> "uǎn",
                "uan4"=> "uàn",
                "Uan1"=> "Uān",
                "Uan2"=> "Uán",
                "Uan3"=> "Uǎn",
                "Uan4"=> "Uàn",
            },
            match_map2: hashmap! {
                // ia
                "ia1"=> "iā",
                "ia2"=> "iá",
                "ia3"=> "iǎ",
                "ia4"=> "ià",
                "Ia1"=> "Iā",
                "Ia2"=> "Iá",
                "Ia3"=> "Iǎ",
                "Ia4"=> "Ià",
                // ua
                "ua1"=> "uā",
                "ua2"=> "uá",
                "ua3"=> "uǎ",
                "ua4"=> "uà",
                "Ua1"=> "Uā",
                "Ua2"=> "Uá",
                "Ua3"=> "Uǎ",
                "Ua4"=> "Uà",
                // ao
                "ao1"=> "āo",
                "ao2"=> "áo",
                "ao3"=> "ǎo",
                "ao4"=> "ào",
                "Ao1"=> "Āo",
                "Ao2"=> "Áo",
                "Ao3"=> "Ǎo",
                "Ao4"=> "Ào",
                // ai
                "ai1"=> "āi",
                "ai2"=> "ái",
                "ai3"=> "ǎi",
                "ai4"=> "ài",
                "Ai1"=> "Āi",
                "Ai2"=> "Ái",
                "Ai3"=> "Ǎi",
                "Ai4"=> "Ài",
                // ie
                "ie1"=> "iē",
                "ie2"=> "ié",
                "ie3"=> "iě",
                "ie4"=> "iè",
                "Ie1"=> "Iē",
                "Ie2"=> "Ié",
                "Ie3"=> "Iě",
                "Ie4"=> "Iè",
                // ue
                "ue1"=> "uē",
                "ue2"=> "ué",
                "ue3"=> "uě",
                "ue4"=> "uè",
                "Ue1"=> "Uē",
                "Ue2"=> "Ué",
                "Ue3"=> "Uě",
                "Ue4"=> "Uè",
                // ei
                "ei1"=> "ēi",
                "ei2"=> "éi",
                "ei3"=> "ěi",
                "ei4"=> "èi",
                "Ei1"=> "Ēi",
                "Ei2"=> "Éi",
                "Ei3"=> "Ěi",
                "Ei4"=> "Èi",
                // ui
                "ui1"=> "uī",
                "ui2"=> "uí",
                "ui3"=> "uǐ",
                "ui4"=> "uì",
                "Ui1"=> "Uī",
                "Ui2"=> "Uí",
                "Ui3"=> "Uǐ",
                "Ui4"=> "Uì",
                // io
                "io1"=> "iō",
                "io2"=> "ió",
                "io3"=> "iǒ",
                "io4"=> "iò",
                "Io1"=> "Iō",
                "Io2"=> "Ió",
                "Io3"=> "Iǒ",
                "Io4"=> "Iò",
                // uo
                "uo1"=> "uō",
                "uo2"=> "uó",
                "uo3"=> "uǒ",
                "uo4"=> "uò",
                "Uo1"=> "Uō",
                "Uo2"=> "Uó",
                "Uo3"=> "Uǒ",
                "Uo4"=> "Uò",
                // ou
                "ou1"=> "ōu",
                "ou2"=> "óu",
                "ou3"=> "ǒu",
                "ou4"=> "òu",
                "Ou1"=> "Ōu",
                "Ou2"=> "Óu",
                "Ou3"=> "Ǒu",
                "Ou4"=> "Òu",
                // iu
                "iu1"=> "iū",
                "iu2"=> "iú",
                "iu3"=> "iǔ",
                "iu4"=> "iù",
                "Iu1"=> "Iū",
                "Iu2"=> "Iú",
                "Iu3"=> "Iǔ",
                "Iu4"=> "Iù",
                // an
                "an1"=> "ān",
                "an2"=> "án",
                "an3"=> "ǎn",
                "an4"=> "àn",
                "An1"=> "Ān",
                "An2"=> "Án",
                "An3"=> "Ǎn",
                "An4"=> "Àn",
                // en
                "en1"=> "ēn",
                "en2"=> "én",
                "en3"=> "ěn",
                "en4"=> "èn",
                "En1"=> "Ēn",
                "En2"=> "Én",
                "En3"=> "Ěn",
                "En4"=> "Èn",
                // er
                "er1"=> "ēr",
                "er2"=> "ér",
                "er3"=> "ěr",
                "er4"=> "èr",
                "Er1"=> "Ēr",
                "Er2"=> "Ér",
                "Er3"=> "Ěr",
                "Er4"=> "Èr",
                // in
                "in1"=> "īn",
                "in2"=> "ín",
                "in3"=> "ǐn",
                "in4"=> "ìn",
                "In1"=> "Īn",
                "In2"=> "Ín",
                "In3"=> "Ǐn",
                "In4"=> "Ìn",
                // un
                "un1"=> "ūn",
                "un2"=> "ún",
                "un3"=> "ǔn",
                "un4"=> "ùn",
                "Un1"=> "Ūn",
                "Un2"=> "Ún",
                "Un3"=> "Ǔn",
                "Un4"=> "Ùn",
                // vn
                "vn1"=> "ǖn",
                "vn2"=> "ǘn",
                "vn3"=> "ǚn",
                "vn4"=> "ǜn",
                "vn"=> "ün",
                "Vn1"=> "Ǖn",
                "Vn2"=> "Ǘn",
                "Vn3"=> "Ǚn",
                "Vn4"=> "Ǜn",
                "Vn"=> "Ün",
                // ve
                "ve1"=> "ǖe",
                "ve2"=> "ǘe",
                "ve3"=> "ǚe",
                "ve4"=> "ǜe",
                "ve"=> "üe",
                "Ve1"=> "Ǖe",
                "Ve2"=> "Ǘe",
                "Ve3"=> "Ǚe",
                "Ve4"=> "Ǜe",
                "Ve"=> "Üe",
            },
            match_map1: hashmap! {
                // a
                "a1"=> "ā",
                "a2"=> "á",
                "a3"=> "ǎ",
                "a4"=> "à",
                "A1"=> "Ā",
                "A2"=> "Á",
                "A3"=> "Ǎ",
                "A4"=> "À",
                // e
                "e1"=> "ē",
                "e2"=> "é",
                "e3"=> "ě",
                "e4"=> "è",
                "E1"=> "Ē",
                "E2"=> "É",
                "E3"=> "Ě",
                "E4"=> "È",
                // i
                "i1"=> "ī",
                "i2"=> "í",
                "i3"=> "ǐ",
                "i4"=> "ì",
                "I1"=> "Ī",
                "I2"=> "Í",
                "I3"=> "Ǐ",
                "I4"=> "Ì",
                // o
                "o1"=> "ō",
                "o2"=> "ó",
                "o3"=> "ǒ",
                "o4"=> "ò",
                "O1"=> "Ō",
                "O2"=> "Ó",
                "O3"=> "Ǒ",
                "O4"=> "Ò",
                // u
                "u1"=> "ū",
                "u2"=> "ú",
                "u3"=> "ǔ",
                "u4"=> "ù",
                "U1"=> "Ū",
                "U2"=> "Ú",
                "U3"=> "Ǔ",
                "U4"=> "Ù",
                // v
                "v1"=> "ǖ",
                "v2"=> "ǘ",
                "v3"=> "ǚ",
                "v4"=> "ǜ",
                "v"=> "ü",
                "V1"=> "Ǖ",
                "V2"=> "Ǘ",
                "V3"=> "Ǚ",
                "V4"=> "Ǜ",
                "V"=> "Ü",
            },
        }
    }
}

pub const VOWELS: &str = "AaEeIiOoUuVv";
pub const TONES: &str = "1234";

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_able_access_compare_match_map_key_value() {
        let pinyin_data = PinyinData::new();
        assert_eq!(pinyin_data.match_map1["iang1"], "iāng");
    }

    #[test]
    fn test_able_show_vowel_in_vowels_const() {
        assert!(VOWELS.contains("a"));
        assert!(VOWELS.contains('A'));
    }

    #[test]
    fn test_tone_1_in_tones_const() {
        assert!(TONES.contains("1"));
    }
}
