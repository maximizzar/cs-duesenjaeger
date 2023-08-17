use clearscreen;

fn main() {
    println!("{}", startup_screen());

    println!("{}", endscreen());
}

fn startup_screen() -> String {
    return "Thank you for checking out our text-adventure. Have fun playing".to_string()
        + cs_duesenjaeger().as_ref();
}
fn endscreen() -> String {
    return r" Thanks for playing. Have a nice day!".to_string()
        + cs_duesenjaeger().as_ref();
}
fn cs_duesenjaeger() -> String {
    return r##"
                ---------------------------------------------------            \\  \\             |""
                |                __|__                            |              .  \\            |   :
                |         --o--o--(_)--o--o--                     |              `   \\           |
                |            cs-duesenjaeger                      |              \\   \\          |    ;               +.
                |                                                 |                .   \\         |                   *._`-.
                |    --- Projektlead                              |                `    \\        |     :          .-*'  `. `.
                |      --- Lucas G.                               |               _\\    \\.__..--**--...L_   _.-*'      .'`*'
                |                                                 |               /  `*-._\\   -.       .-*"*+._       .'
                |    --- Development management                   |              :        ``*-._*.     \\      _J.   .'
                |      --- Marvin R.                              |          .-*'`*-.       ;     `.    \\    /   `.'
                |                                                 |      .-*'  _.-*'.     .-'       `-.  `-.:   _.'`-.
                |    --- Development                              |   +*' _.-*'      `..-'             `*-. `**'      `-.
                |      --- Maximilian K. x Kevin S. x Laurin L.   |    `*'          .-'      ._            `*-._         `.
                |                                                 |              .-'         `.`-.____..+-**""'         .*"`.
                ---------------------------------------------------         ._.-'          _.-*'':$$$;._$              /     `.
                                                                         .-'  `.      _.-*' `*-.__T$P   `"**--..__    :        `.
                                                                  .'..-'       \\_.-*'                            `"**--..___.-*'\
    "##.to_string()
}