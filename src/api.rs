use super::roots::*;
use super::vowels::*;
use super::wordbuf::WordBuffer;

fn check_input_domain(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ')
}

pub fn words(s: &str) -> Option<WordBuffer> {
    //  FIXME: Proper punctuation handling.
    let s = s.replace(',', "");
    let s = s.replace('.', "");
    let s = s.replace('?', "");
    let s = s.replace('!', "");

    let words = s.split(' ').map(word).collect::<Option<Vec<_>>>()?;
    let mut words = words.into_iter();
    let mut head = words.next().unwrap();
    for trailing_word in words {
        head.append(trailing_word);
    }
    Some(head)
}

pub fn word(s: &str) -> Option<WordBuffer> {
    let mut chars = s.chars().into_iter().collect::<Vec<char>>();

    (check_input_domain(s) && !chars.is_empty()).then_some(())?;

    let mut w = WordBuffer::default();

    chars.insert(0, ' ');
    chars.push(' ');

    chars.as_slice().windows(3).for_each(|c| {
        if c[1] == 'q' && c[2] == 'u' {
            w.vowel(vowel_qu())
        } else if !(c[0] == 'q' && c[1] == 'u') {
            match c[1].to_ascii_lowercase() {
                'n' => w.consonant(root_n()),
                'm' => w.consonant(root_m()),
                'p' => w.consonant(root_p()),
                'b' => w.consonant(root_b()),
                'v' => w.consonant(root_v()),
                'w' => w.consonant(root_w()),
                'x' => w.consonant(root_x()),
                'z' => w.consonant(root_z()),
                's' => w.consonant(root_s()),
                't' => w.consonant(root_t()),
                'd' => w.consonant(root_d()),
                'r' => w.consonant(root_r()),
                'h' => w.consonant(root_h()),
                'c' => w.consonant(root_c()),
                'j' => w.consonant(root_j()),
                'k' => w.consonant(root_k()),
                'g' => w.consonant(root_g()),
                'l' => w.consonant(root_l()),
                'f' => w.consonant(root_f()),
                'y' => w.consonant(root_y()),

                'a' => w.vowel(vowel_a()),
                'e' => w.vowel(vowel_e()),
                'i' => w.vowel(vowel_i()),
                'o' => w.vowel(vowel_o()),
                'u' => w.vowel(vowel_u()),

                'q' => w.vowel(vowel_q()),
                _ => (),
            }
        }
    });
    Some(w)
}
