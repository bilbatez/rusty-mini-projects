use std::io;
use std::process;

fn main() {
    greeting();
    loop {
        println!("Please input choose an operator [*/+-]!!");
        let operator = ask_operator_from_user();

        println!("Please input the first number!!");
        let first = ask_input_number_from_user();

        println!("Please input the second number!!");
        let second = ask_input_number_from_user();

        let result = calculate(operator, first, second);
        println!("Result is: {}\n", result);
        check_if_special_number(result);

        println!("Try Again? [y/n]");
        let mut try_again = String::from("");
        io::stdin()
            .read_line(&mut try_again)
            .expect("Failed to read line!");
        if try_again.chars().nth(0).unwrap().to_ascii_lowercase() != 'y' {
            if_blank_exit(&String::from(""));
        }
    }
}

fn greeting() {
    println!(r#"
 _____ _                 _
/  ___(_)               | |
\ `--. _ _ __ ___  _ __ | | ___
 `--. \ | '_ ` _ \| '_ \| |/ _ \
/\__/ / | | | | | | |_) | |  __/
\____/|_|_| |_| |_| .__/|_|\___|
                  | |
                  |_|
 _____       _            _       _
/  __ \     | |          | |     | |
| /  \/ __ _| | ___ _   _| | __ _| |_ ___  _ __
| |    / _` | |/ __| | | | |/ _` | __/ _ \| '__|
| \__/\ (_| | | (__| |_| | | (_| | || (_) | |
 \____/\__,_|_|\___|\__,_|_|\__,_|\__\___/|_|
------------------------------------------------
"#);
    println!("Created by: Albert JT\n\n")
}

fn ask_operator_from_user() -> char {
    loop {
        let mut cur = String::from("");
        io::stdin()
            .read_line(&mut cur)
            .expect("Failed to read line");
        cur.pop();

        if_blank_exit(&cur);

        let count = cur.chars().count();
        let value = cur.chars().nth(0).unwrap();
        if count > 1 || count == 0 || check_operator(value) == false {
            println!("Please choose between '*', '/', '+', or '-'")
        } else {
            break value;
        }
    }
}

fn check_operator(value: char) -> bool {
    value == '*' || value == '/' || value == '+' || value == '-'
}

fn ask_input_number_from_user() -> isize {
    loop {
        let mut cur = "".to_string();
        io::stdin()
            .read_line(&mut cur)
            .expect("Failed to read line!");

        if_blank_exit(&cur);

        match cur.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please input a number!!")
        };
    }
}

fn if_blank_exit(value: &String) {
    if value.is_empty() {
        println!("Bye-bye!!");
        process::exit(0);
    }
}

fn calculate(operator: char, val1: isize, val2: isize) -> isize {
    if operator == '*' {
        val1 * val2
    } else if operator == '/' {
        val1 / val2
    } else if operator == '+' {
        val1 + val2
    } else {
        val1 - val2
    }
}

fn check_if_special_number(value: isize) {
    match value {
        5 => vendetta(),
        8 => lucky_number(),
        221 => baker_street(),
        405 => star_wars(),
        666 => summon_prince_of_darkness(),
        1948 => big_brother_is_watching(),
        9000 => its_over_9000(),
        _ => println!("")
    }
}

fn vendetta() {
    println!("V for Vendetta");
    println!(r#"
─────█─▄▀█──█▀▄─█─────
────▐▌──────────▐▌────
────█▌▀▄──▄▄──▄▀▐█────
───▐██──▀▀──▀▀──██▌───
──▄████▄──▐▌──▄████▄──
    "#);
}

fn lucky_number() {
    println!("A lucky cat approaches you!");
    println!(r#"=^._.^= ∫"#);
    println!("You will be lucky today!!");
}

fn baker_street() {
    println!(r#"
      ,_
 ,'  `\,_
 |_,-'_)
 /##c '\  (
' |'  -{{.  )
  /\__-' \[]
 /`-_`\
 '     \
    "#);
    println!("Elementary.")
}

fn star_wars() {
    println!("May the force be with you!");
    println!(r#"
               __
.-.__      \ .-.  ___  __
|_|  '--.-.-(   \/\;;\_\.-._______.-.
(-)___     \ \ .-\ \;;\(   \       \ \
 Y    '---._\_((Q)) \;;\\ .-\     __(_)
 I           __'-' / .--.((Q))---'    \,
 I     ___.-:    \|  |   \'-'_          \
 A  .-'      \ .-.\   \   \ \ '--.__     '\
 |  |____.----((Q))\   \__|--\_      \     '
    ( )        '-'  \_  :  \-' '--.___\
     Y                \  \  \       \(_)
     I                 \  \  \         \,
     I                  \  \  \          \
     A                   \  \  \          '\
     |                    \  \__|           '
                           \_:.  \
                             \ \  \
                              \ \  \
                               \_\_|
    "#);
}

fn summon_prince_of_darkness() {
    println!(r#"
            ______
       .d$$$******$$$$c.
    .d$P"            "$$c
   $$$$$.           .$$$*$.
 .$$ 4$L*$$.     .$$Pd$  '$b
 $F   *$. "$$e.e$$" 4$F   ^$b
d$     $$   z$$$e   $$     '$.
$P     `$L$$P` `"$$d$"      $$
$$     e$$F       4$$b.     $$
$b  .$$" $$      .$$ "4$b.  $$
$$e$P"    $b     d$`    "$$c$F
'$P$$$$$$$$$$$$$$$$$$$$$$$$$$
 "$c.      4$.  $$       .$$
  ^$$.      $$ d$"      d$P
    "$$c.   `$b$F    .d$P"
      `4$$$c.$$$..e$$P"
          `^^^^^^^`
       "#);
    println!("You have summoned the prince of darkness...");
    println!("My God... what have you done...");
}

fn big_brother_is_watching() {
    println!("Big Brother is watching...");
    println!(r#"
               _ . - = - . _
       . "  \  \   /  /  " .
     ,  \                 /  .
   . \   _,.--~=~"~=~--.._   / .
  ;  _.-"  / \ !   ! / \  "-._  .
 / ,"     / ,` .---. `, \     ". \
/.'   `~  |   /:::::\   |  ~`   '.\
\`.  `~   |   \:::::/   | ~`  ~ .'/
 \ `.  `~ \ `, `~~~' ,` /   ~`.' /
  .  "-._  \ / !   ! \ /  _.-"  .
   ./    "=~~.._  _..~~=`"    \.
     ,/         ""          \,
       . _/             \_ .
          " - ./. .\. - "
    "#);
}

fn its_over_9000() {
    println!("It's over 9000!!!");
    println!(r#"
                    `\-.   `
                      \ `.  `
                       \  \ |
              __.._    |   \.
       ..---~~     ~ . |    Y
         ~-.          `|    |
            `.               `~~--.
              \                    ~.
               \                     \__. . -- -  .
         .-~~~~~      ,    ,            ~~~~~~---...._
      .-~___        ,'/  ,'/ ,'\          __...---~~~
            ~-.    /._\_( ,(/_. 7,-.    ~~---...__
           _...>-  P""6=`_/"6"~   6)    ___...--~~~
            ~~--._ \`--') `---'   9'  _..--~~~
                  ~\ ~~/_  ~~~   /`-.--~~
                    `.  ---    .'   \_
                      `. " _.-'     | ~-.,-------._
                  ..._../~~   ./       .-'    .-~~~-.
            ,--~~~ ,'...\` _./.----~~.'/    /'       `-
        _.-(      |\    `/~ _____..-' /    /      _.-~~`.
       /   |     /. ^---~~~~       ' /    /     ,'  ~.   \
      (    /    (  .           _ ' /'    /    ,/      \   )
      (`. |     `\   - - - - ~   /'      (   /         .  |
       \.\|       \            /'        \  |`.           /
       /.'\\      `\         /'           ~-\         .  /\
      /,   (        `\     /'                `.___..-      \
     | |    \         `\_/'                  //      \.     |
     | |     |                             /' |       |     |
    "#);
}