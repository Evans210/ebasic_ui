use ebasic_ui::*;
use std::io;

fn main() -> io::Result<()> {
    start()?;

    bughunt()?;

    end()?;
    Ok(())
}

fn bughunt() -> io::Result<()> {
    let mut str1 = "º";
    let mut a = ran_int(2, 20);
    let mut b = ran_int(2, 6);
    let c = ran_int(2, 20);
    let d = ran_int(2, 6);
    let mut x = ran_int(2, 20);
    let mut y = ran_int(2, 6);
    let mut k = 0;
    let mut h = "";
    let mut v = 1;
    let mut o = 0;
    let mut s = 0;
    let mut g = String::new();

    loop {
        if v == 1 {
            locate(c, d, "└")?;
        }

        if x == c && y == d && v == 1 {
            locate(4, 7, "SWORD")?;
            if g == "x" {
                locate(4, 6, "ATK+1")?;
                v = 2;
                str1 = "@";
            }
        }

        loop {
            g = getkey()?;

            if !g.is_empty() {
                break;
            }
        }

        if g == "a" || g == "Left" {
            if x != 2 {
                x -= 1;
                h = "Left";
            }
            if a != 21 {
                a += 1;
            }
        }

        if g == "d" || g == "Right" {
            if x != 20 {
                x += 1;
                h = "Right";
            }
            if a != 1 {
                a -= 1;
            }
        }

        if g == "w" || g == "Up" {
            if y != 2 {
                y -= 1;
                h = "Up";
            }
            if b != 7 {
                b += 1;
            }
        }

        if g == "s" || g == "Down" {
            if y != 6 {
                y += 1;
                h = "Down";
            }
            if b != 1 {
                b -= 1;
            }
        }

        if g == "c" {
            locate(1, 1, "PAUSE")?;
            locate(1, 2, "GOLD: ")?;
            locate(6, 2, &o.to_string())?;
            pause()?;
        }
        clr_text()?;

        locate(x, y, str1)?;

        if g == "x" && v == 2 {
            if h == "Up" {
                locate(x, y - 1, "↓")?;
                if a == x && b == y - 1 {
                    s = 1;
                }
            }

            if h == "Right" {
                locate(x + 1, y, "←")?;
                if a == x + 1 && b == y {
                    s = 1;
                }
            }

            if h == "Left" {
                locate(x - 1, y, "→")?;
                if a == x - 1 && b == y {
                    s = 1;
                }
            }

            if h == "Down" {
                locate(x, y + 1, "↑")?;
                if a == x && b == y + 1 {
                    s = 1;
                }
            }
        }

        locate(a, b, "¤")?;

        if x == a && y == b {
            k = 1;
        }

        if s == 1 {
            o += 1;
            a = ran_int(2, 20);
            b = ran_int(2, 6);
            s = 0;
        }

        locate(6, 2, "░")?;
        locate(3, 4, "░")?;
        locate(14, 4, "░")?;
        locate(7, 5, "░")?;
        locate(17, 3, "░")?;
        locate(10, 6, "░")?;
        locate(19, 6, "░")?;

        if !(k != 1 && g != "q") {
            break;
        }
    }

    locate(1, 1, "GAME OVER")?;
    Ok(())
}
