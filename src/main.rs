use prompted::input;

fn main() {
    let filled = input!("number of rune slots filled already ").parse::<u32>().unwrap();
    let max_runes = input!("maximum number of runes to calculate for ").parse::<u32>().unwrap();
    let broken = input!("number of broken shards ").parse::<u32>().unwrap();
    let optimisation = input!("would you like to calculate chances for optimal broken shard usage (Y), or for immediately forging runes when possible (N)? ");
    let  wait;
    if optimisation == "Y" || optimisation == "y" {
        wait = -1;
    }
    else {
        wait = -2;
    }
    let mut continue_normally: String = "Y".into();
    if broken == 10 {
        let calc_broken = input!("calculate optimal time to use your current broken shards? (Y/N) ");
        if calc_broken == "Y" || calc_broken == "y"{
            let mut optimal_wait = 0;
            let mut max_value = 0.0;
            for i in 0..(9 - filled) {
                let value = calc(filled, true, 1f64 - (filled as f64 / 9f64), 0, 10, i as i32, max_runes) + calc(filled, false, filled as f64 / 9f64, 0, 10, i as i32, max_runes);
                if value > max_value {
                    max_value = value;
                    optimal_wait = i;
                }
            };
            println!("wait until you fill {} more slots before forging rune", optimal_wait);
            continue_normally = input!("continue to normal calculations? (Y/N)");
        }
    }
if continue_normally == "Y" || continue_normally == "y" {
    for k in 0..max_runes {
        let max_depth = k + 1;
        let chance = calc(filled, true, 1f64 - (filled as f64 / 9f64), 0, broken, wait, max_depth) + calc(filled, false, filled as f64 / 9f64, 0, broken, wait, max_depth);
        println!("{}% chance with {} runes", chance*100f64, max_depth);
    }
}
}

fn calc(mut current_filled: u32, success: bool, weight: f64, mut depth: u32, mut current_broken: u32, mut runes_to_wait: i32, max_depth: u32) -> f64 {
    if weight == 0.0 {
        return 0.0;
    }
    if current_broken == 10 {
        //println!("{success} {weight}");
        //println!("depth {depth}");
        if runes_to_wait == 0 || runes_to_wait == -2 {
            current_filled += 1;
            current_broken = 0;
            if current_filled == 9 {
                return weight;
            }
            else if depth == max_depth {
                return 0.0
            }
        }
        else if runes_to_wait == -1 {
            let mut optimal_wait = 0;
            let mut max_value = 0.0;
            for i in 0..(9 - current_filled) {
                if i+depth <= max_depth {
                    let value = calc(current_filled, true, 1f64 - (current_filled as f64 / 9f64), depth, current_broken, i as i32, max_depth) + calc(current_filled, false, current_filled as f64 / 9f64, depth, current_broken, i as i32, max_depth);
                    //println!("{i} {value}");
                    if value > max_value {
                        max_value = value;
                        optimal_wait = i;
                    }
                }
            };
            //println!("{}  ", optimal_wait);
            runes_to_wait = optimal_wait as i32;
            if runes_to_wait == 0 {
                current_filled += 1;
                current_broken = 0;
                if current_filled == 9 {
                    return weight;
                }
                else if depth == max_depth {
                    return 0.0
                }
            }
            else if depth == max_depth {
                return 0.0
            }
        }
    }
    depth += 1;
    if success {
        if runes_to_wait >= 0 {
            runes_to_wait -= 1;
        }
        current_filled += 1;
        if current_filled == 9 {
            weight
        } else if (depth == max_depth) && (current_broken != 10 || runes_to_wait > 0) {
            0.0
        } else {
            weight * (calc(current_filled, true, 1f64 - (current_filled as f64 / 9f64), depth, current_broken, runes_to_wait, max_depth) + calc(current_filled, false, current_filled as f64 / 9f64, depth, current_broken, runes_to_wait, max_depth))
        }
    } else {
        if current_broken != 10 {
            current_broken += 1;
        }
        if (depth == max_depth) && (current_broken != 10 || runes_to_wait > 0) {
            0.0
        } else {
            weight * (calc(current_filled, true, 1f64 - (current_filled as f64 / 9f64), depth, current_broken, runes_to_wait, max_depth) + calc(current_filled, false, current_filled as f64 / 9f64, depth, current_broken, runes_to_wait, max_depth))
        }
    }
}