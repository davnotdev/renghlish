#[derive(Default)]
pub struct WordBuffer {
    bufs: [String; 6],
    previous_was_vowel: bool,
    previous_was_barred: bool,
}

impl WordBuffer {
    pub fn get_rows(&self) -> &[String; 6] {
        &self.bufs
    }

    pub fn append(&mut self, other: WordBuffer) {
        self.advance();

        self.previous_was_vowel = false;
        self.previous_was_barred = false;

        for (self_buf, other_buf) in self.bufs.iter_mut().zip(other.bufs) {
            self_buf.push_str(&other_buf);
        }
    }

    pub fn vowel(&mut self, lanes: &[&str]) {
        self.advance();
        self.overwrite(lanes);
        self.draw_vowel_bar();
        self.previous_was_vowel = true;
        self.previous_was_barred = true;
    }

    pub fn consonant(&mut self, (lanes, can_go_under): (&[&str], bool)) {
        let mut still_barred = self.previous_was_barred;

        if !(can_go_under && self.previous_was_vowel) {
            still_barred = true;
            self.advance();
        }
        self.overwrite(lanes);

        if !can_go_under && self.previous_was_barred {
            self.draw_vowel_bar();
            still_barred = true;
        }
        if !still_barred {
            self.previous_was_barred = false;
        }

        self.previous_was_vowel = false;
    }

    fn advance(&mut self) {
        for buf in self.bufs.iter_mut() {
            //            123
            buf.push_str("   ");
        }
    }

    fn overwrite(&mut self, lanes: &[&str]) {
        for idx in 0..self.bufs.len() {
            let buf = &mut self.bufs[idx];
            if let Some(lane) = lanes.get(idx) {
                let lane_chars: Vec<char> = lane.chars().collect();
                if !lane_chars.is_empty() {
                    let mut popped = vec![];
                    for _ in 0..3 {
                        popped.insert(0, buf.pop());
                    }
                    for cidx in 0..3 {
                        if popped[cidx].unwrap() == ' ' {
                            buf.push(*lane_chars.get(cidx).unwrap_or(&' '));
                        } else {
                            buf.push(popped[cidx].unwrap());
                        }
                    }
                }
            }
        }
    }

    fn draw_vowel_bar(&mut self) {
        let bar_buf = &mut self.bufs[1];
        let chars: Vec<char> = bar_buf.chars().collect();
        let bar_length = if chars.len() >= 5 { 5 } else { 3 };
        let range = chars.len() - bar_length..chars.len();
        for _ in range.clone() {
            bar_buf.pop();
        }
        for cidx in range {
            if chars[cidx] != ' ' && chars[cidx] != '─' {
                bar_buf.push('┼');
            } else {
                bar_buf.push('─');
            }
        }
    }
}
