type CanGoUnder = bool;
type Return = (&'static [&'static str], CanGoUnder);

#[rustfmt::skip]
pub fn root_n() -> Return {
    (
    &["",
      "",
      "",
      "┌─┐",
      "└─┘"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_m() -> Return {
    (
    &["",
      "",
      "",
      "┌─┐",
      "└─┘",
      "───"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_p() -> Return {
    (
    &["┌─╮",
      "│",  
      "│",  
      "│",  
      "│", 
      "│"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_b() -> Return {
    (
    &["┌─╮",
      "│",  
      "│",  
      "│",  
      "│", 
      "│",
      "┴──"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_v() -> Return {
    (
    &["",
      "",  
      "",  
      "",  
      "╲╱"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_w() -> Return {
    (
    &["",
      "",  
      "",  
      "",  
      " ╲╱", 
      "───"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_x() -> Return {
    (
    &["",
      "",  
      "",  
      "",  
      "╶┼╴"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_z() -> Return {
    (
    &["",
      "",  
      "",  
      "",  
      "╶┼╴",
      "───"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_s() -> Return {
    (
    &["",
      "",  
      " ╭",  
      "─┼─",  
      "─╯"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_t() -> Return {
    (
    &["",
      "",  
      " ╮",  
      "─┼─",  
      " ╰╴"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_d() -> Return {
    (
    &["",
      "",  
      " ╮",  
      "─┼─",  
      " ├─",
      " ╰╴"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_r() -> Return {
    (
    &["  │",
      "  │",
      "  │",
      "  │",  
      "╭─╯",  
      "│"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_h() -> Return {
    (
    &["  │",
      "  │",  
      "  │",  
      "╭─╯",  
      "│",
      "┴──"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_c() -> Return {
    (
    &["",
      "",  
      "╭─╴",  
      "│",  
      "╰──"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_j() -> Return {
    (
    &["",
      "",  
      "",  
      "",  
      " │",
      "─┴─"],
    true,
    )
}

#[rustfmt::skip]
pub fn root_k() -> Return {
    (
    &["──╮",
      "  │",  
      "  │",  
      "──┤",  
      "  │"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_g() -> Return {
    (
    &["──╮",
      "  │",  
      "  │",  
      "──┤",  
      "  │",
      "──┴"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_l() -> Return {
    (
    &["  │",
      "  │",  
      "  │",  
      "  │",  
      "──╯"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_f() -> Return {
    (
    &["│",
      "│",  
      "│",  
      "├──",  
      "│",
      "│"],
    false,
    )
}

#[rustfmt::skip]
pub fn root_y() -> Return {
    (
    &["  │",
      "  │",  
      "  │",  
      "──┤",  
      "  │",
      "  │"],
    false,
    )
}
