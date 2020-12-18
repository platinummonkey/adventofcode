use std::ops::RangeInclusive;
#[allow(unused_imports)]
use util::*;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_16/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_16/data/input"));
}

fn part_1(filename: &str) -> i64 {
    let (all_filters, _ticket, tickets) = read_raw_data(filename);
    let mut expected_set: HashSet<i64> = HashSet::new();
    for filters in all_filters.values() {
        for filter in filters.clone() {
            for v in filter {
                expected_set.insert(v);
            }
        }
    }
    let mut running_sum: i64 = 0;
    for ticket in tickets {
        for val in ticket {
            if !expected_set.contains(&val) {
                running_sum += val;
            }
        }
    }
    running_sum
}

fn part_2(filename: &str) -> i64 {
    let (all_filters, my_ticket, tickets) = read_raw_data(filename);
    let mut expected_set: HashSet<i64> = HashSet::new();
    for filters in all_filters.values() {
        for filter in filters.clone() {
            for v in filter {
                expected_set.insert(v);
            }
        }
    }
    let mut valid_tickets: Tickets = Tickets::new();
    let mut coords: HashMap<usize, HashSet<i64>> = HashMap::new();
    for ticket in tickets {
        if ticket.clone().iter().all(|v| expected_set.contains(v)) {
            valid_tickets.push(ticket.clone());
            for (i, v) in ticket.clone().iter().enumerate() {
                let mut ns: HashSet<i64> = HashSet::new();
                ns.insert(*v);
                match coords.insert(i, ns) {
                    Some(mut prev) => {
                        prev.insert(*v);
                        coords.insert(i, prev);
                    }
                    None => {}
                }
            }
        }
    }

    let ticket_cols = valid_tickets[0].len();

    let mut coords_found: HashMap<usize, String> = HashMap::new();
    let mut coords_remaining: HashMap<usize, HashSet<String>> = HashMap::new();
    let all_filter_names: HashSet<String> = all_filters
        .keys()
        .map(|k| k.to_string())
        .collect::<HashSet<String>>();

    for i in 0..ticket_cols {
        coords_remaining.insert(i, all_filter_names.clone());
    }

    for filter_name in all_filter_names.iter().map(|s| s.to_string()).cycle() {
        if coords_found.len() == ticket_cols {
            break;
        }

        for col in 0..ticket_cols {
            let remaining = coords_remaining.get_mut(&col).expect("wut");
            if remaining.len() == 1 {
                // found our match for a given index
                let found = remaining
                    .iter()
                    .take(1)
                    .next()
                    .expect("expected value")
                    .to_string();
                coords_found.insert(col, found.clone());
                for rem in coords_remaining.values_mut() {
                    rem.remove(found.clone().as_str());
                }
            }
        }

        // try to find the matching index, if not just pop it back onto the queue and try again later.
        let filters = all_filters.get(&filter_name).expect("filter should exist");
        let mut required_for_filter: HashSet<i64> = HashSet::new();
        for filter in filters.clone() {
            required_for_filter.extend(filter);
        }

        for col in 0..ticket_cols {
            let matches = valid_tickets
                .iter()
                .map(|t| t[col])
                .all(|v| required_for_filter.contains(&v));
            if !matches {
                let remaining = coords_remaining.get_mut(&col).expect("wut");
                remaining.remove(filter_name.as_str());
                if remaining.len() == 1 {
                    // found our match for a given index
                    let found = remaining
                        .iter()
                        .take(1)
                        .next()
                        .expect("expected value")
                        .to_string();
                    coords_found.insert(col, found.clone());
                    for rem in coords_remaining.values_mut() {
                        rem.remove(found.clone().as_str());
                    }
                }
            }
        }
    }

    let mut result: i64 = 1;
    for (col, filter_name) in coords_found.iter() {
        if filter_name.starts_with("departure") {
            result *= my_ticket[col.clone()];
        }
    }

    result
}

type RangeFilters = Vec<RangeInclusive<i64>>;
type Ticket = Vec<i64>;
type Tickets = Vec<Ticket>;

#[derive(Clone)]
enum ParseState {
    Filters,
    YourTicket,
    NearbyTickets,
}

fn read_raw_data(filename: &str) -> (HashMap<String, RangeFilters>, Ticket, Tickets) {
    let mut all_filters: HashMap<String, RangeFilters> = HashMap::new();
    let mut ticket: Ticket = Ticket::new();
    let mut tickets: Tickets = Tickets::new();
    let mut parse_state = ParseState::Filters {};
    if let Ok(lines) = util::iter_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match parse_state.clone() {
                ParseState::Filters => {
                    if line.len() == 0 {
                        parse_state = ParseState::YourTicket;
                        continue;
                    }
                    let filter_name = line.splitn(2, ':').take(1).collect_vec()[0].to_string();
                    let mut filters: RangeFilters = RangeFilters::new();
                    for part in line
                        .split_ascii_whitespace()
                        .skip_while(|p| !p.ends_with(":"))
                        .skip(1)
                        .filter(|&s| s != "or")
                    {
                        let range_parts = part
                            .split("-")
                            .map(|s| s.parse::<i64>().expect("expected integer"))
                            .collect_vec();
                        assert_eq!(2, range_parts.len());
                        let filter_range: RangeInclusive<i64> = range_parts[0]..=range_parts[1];
                        filters.push(filter_range);
                    }
                    all_filters.insert(filter_name, filters);
                }
                ParseState::YourTicket => {
                    if line.len() == 0 {
                        parse_state = ParseState::NearbyTickets;
                        continue;
                    } else if line == "your ticket:" {
                        continue;
                    }
                    ticket.extend(
                        line.split(",")
                            .map(|i| i.parse::<i64>().expect("expected integer")),
                    );
                }
                ParseState::NearbyTickets => {
                    if line == "nearby tickets:" {
                        continue;
                    }
                    let ticket: Ticket = line
                        .split(",")
                        .map(|i| i.parse::<i64>().expect("expected integer"))
                        .collect::<Ticket>();
                    tickets.push(ticket);
                }
            }
        }
    }
    (all_filters, ticket, tickets)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: i64 = 22977;
        let result: i64 = super::part_1("data/input");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 998358379943;
        let result: i64 = super::part_2("data/input");
        assert_eq!(expected, result);
    }
}
