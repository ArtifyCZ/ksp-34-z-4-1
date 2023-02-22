use text_io::scan;

fn first_bigger(x: u32, y: u32, z: u32) -> u32 {
    if (x - y) % z == 0 {
        return x;
    }

    x + z - (x - y) % z
}

fn has_seen(p: u32, f: u32, born: u32, died: u32) -> bool {
    if died < f {
        return false;
    }

    if born <= f && f <= died {
        return true;
    }

    // let next = born + ((born - f) % p);
    // let next = born + ((born - f) % p);
    // let next = born + ((born - f) % p);
    let next = first_bigger(born, f, p);

    if next <= died {
        return true;
    }

    return false;
}

// https://ksp.mff.cuni.cz/z/ulohy/34/zadani4.html#task-34-Z4-1
fn main() {
    let n: u32; // THE COUNT OF PEOPLE
    let p: u32; // THE COMET'S PERIOD
    let f: u32; // YEAR WHEN THE COMET ARRIVED FOR THE FIRST TIME
    scan!("{} {} {}", n, p, f);

    let mut count: u32 = 0; // COUNT OF PEOPLE WHO HASN'T SEEN THE COMET

    // COUNT THE N PEOPLE
    for _ in 0..n {
        let born: u32; // WHEN BORN
        let died: u32; // WHEN DIED
        scan!("{} {}", born, died);

        if !has_seen(p, f, born, died) {
            count += 1;
        }
    }

    println!("{}", count);
}
