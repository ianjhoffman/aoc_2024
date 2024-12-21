use util::res::Result;
use util::file::GenericParseError;

struct OrderingRule {
    before: u8,
    after: u8,
}

struct Update {
    pages: Vec<u8>
}

impl std::str::FromStr for OrderingRule {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 2 {
            return Err(GenericParseError::ValueError(format!("Invalid ordering rule: {}", s).to_owned()))
        }

        Ok(OrderingRule{
            before: parts[0].parse::<u8>()?,
            after: parts[1].parse::<u8>()?,
        })
    }
}

impl Update {
    fn middle_page_number<T: From<u8>>(&self) -> T {
        T::from(self.pages[self.pages.len() / 2])
    }
}

impl std::str::FromStr for Update {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pages = s.split(',').map(|raw_num| raw_num.parse::<u8>().map_err(|e| e.into()))
            .collect::<std::result::Result<Vec<u8>, Self::Err>>()?;
        Ok(Update{pages})
    }
}

fn parse_ordering_rules_and_updates(lines: &Vec<String>) -> Result<(Vec<OrderingRule>, Vec<Update>)> {
    let mut rules: Vec<OrderingRule> = vec![];
    let mut updates: Vec<Update> = vec![];

    let mut in_ordering_rules: bool = true;
    for line in lines {
        // Transition to section 2: update page lists
        if line.is_empty() {
            in_ordering_rules = false;
            continue;
        }

        if in_ordering_rules {
            rules.push(line.parse::<OrderingRule>()?);
        } else {
            updates.push(line.parse::<Update>()?);
        }
    }

    Ok((rules, updates))
}

fn generate_preceding_bitmaps(ordering_rules: &Vec<OrderingRule>) -> [u128; 100] {
    let mut ret: [u128; 100] = [0u128; 100];
    for ordering_rule in ordering_rules {
        let mask: u128 = 1u128 << u128::from(ordering_rule.before);
        ret[usize::from(ordering_rule.after)] |= mask;
    }
    ret
}

fn categorize_updates(updates: Vec<Update>, preceding_bitmaps: &[u128; 100]) -> (Vec<Update>, Vec<Update>) {
    updates.into_iter().partition(|update| {
        let mut cannot_appear_bitmap = 0u128;
        for page in &update.pages {
            let mask = 1u128 << u128::from(*page);
            if (cannot_appear_bitmap & mask) != 0 {
                // println!("cannot appear");
                return false
            }

            cannot_appear_bitmap |= preceding_bitmaps[usize::from(*page)];
        }
        true
    })
}

fn part1(correctly_ordered: &Vec<Update>) {
    let middle_page_sum: u32 = correctly_ordered.iter().map(|update| update.middle_page_number::<u32>()).sum();

    println!("Middle page number sum of correctly-ordered updates: {:?}", middle_page_sum);
}

fn part2(incorrectly_ordered: &Vec<Update>, preceding_bitmaps: &[u128; 100]) {
    let rearranged: Vec<Update> = incorrectly_ordered.iter().map(|update| {
        let mut pages = update.pages.clone();

        // Construct bitmaps of page numbers appearing after a given index in pages
        let mut appearing_after_masks = vec![0u128; pages.len()];
        for i in (0..(pages.len() - 1)).rev() {
            let mask = 1u128 << u128::from(pages[i + 1]);
            appearing_after_masks[i] = appearing_after_masks[i + 1] | mask;
        }

        // It's bubblesort time baby
        let mut start_idx = 0;
        while start_idx < (pages.len() - 1) {
            for i in start_idx..(pages.len() - 1) {
                // There are no page numbers after this one that must be before it, stop swapping
                if (appearing_after_masks[i] & preceding_bitmaps[usize::from(pages[i])]) == 0 {
                    // If this happened with no swaps, shift forward to just sort the remainder of the list
                    if i == start_idx {
                        start_idx += 1;
                    }

                    break
                }

                // Update appearing_after_masks to reflect that what _was_ after the entry
                // at the current index now includes the page number at the current index
                // and excludes the page number at the following index (consequence of
                // swapping the two pages)
                let mask = 1u128 << u128::from(pages[i]);
                appearing_after_masks[i] = appearing_after_masks[i + 1] | mask;

                // Actually swap the pages
                pages.swap(i, i + 1);
            }
        }

        Update{pages}
    }).collect();
    let middle_page_sum: u32 = rearranged.iter().map(|update| update.middle_page_number::<u32>()).sum();

    println!("Middle page number sum of rearranged incorrectly-ordered updates: {:?}", middle_page_sum);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;

    // Preprocessing :)
    let (ordering_rules, updates) = parse_ordering_rules_and_updates(&lines)?;
    let preceding_bitmaps = generate_preceding_bitmaps(&ordering_rules);
    let (correctly_ordered, incorrectly_ordered) = categorize_updates(updates, &preceding_bitmaps);

    part1(&correctly_ordered);
    part2(&incorrectly_ordered, &preceding_bitmaps);

    Ok(())
}