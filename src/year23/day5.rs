use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    destination: usize,
    source: usize,
    range: usize,
}

fn parse_map(lines: &str) -> Vec<Map> {
    let mut maps: Vec<Map> = Vec::new();
    let convertions = lines.trim().split("\n").skip(1);
    for conv in convertions {
        let mut range = conv.trim().split(' ').map(|x| x.parse::<usize>().unwrap());
        let destination = range.next().unwrap();
        let source = range.next().unwrap();
        let range = range.next().unwrap();
        maps.push(Map {
            destination,
            source,
            range,
        });
    }
    maps
}

fn map_to_destination(source: usize, maps: &Vec<Map>) -> usize {
    for map in maps {
        if source >= map.source && source <= map.source + map.range {
            let offset = source - map.source;
            return map.destination + offset;
        }
    }
    source
}

fn part1(input: String) -> String {
    let mut maps = input.split("\n\n");
    let seeds: Vec<usize> = maps
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let seed_to_soil = parse_map(maps.next().unwrap());
    let soil_to_fertilizer = parse_map(maps.next().unwrap());
    let fertilize_to_water = parse_map(maps.next().unwrap());
    let water_to_light = parse_map(maps.next().unwrap());
    let light_to_temperature = parse_map(maps.next().unwrap());
    let temperature_to_humidity = parse_map(maps.next().unwrap());
    let humidity_to_location = parse_map(maps.next().unwrap());

    let mut locations: Vec<usize> = Vec::new();
    for seed in seeds {
        let soil = map_to_destination(seed, &seed_to_soil);
        let fertilizer = map_to_destination(soil, &soil_to_fertilizer);
        let water = map_to_destination(fertilizer, &fertilize_to_water);
        let light = map_to_destination(water, &water_to_light);
        let temperature = map_to_destination(light, &light_to_temperature);
        let humidity = map_to_destination(temperature, &temperature_to_humidity);
        locations.push(map_to_destination(humidity, &humidity_to_location));
    }
    locations.iter().min().unwrap().to_string()
}

fn map_source_range_to_destination_range(
    source_start: usize,
    source_end: usize,
    maps: &Vec<Map>,
) -> HashSet<(usize, usize)> {
    let mut ranges = HashSet::new();
    println!("{} {}", source_start, source_end);
    for map in maps {
        let mut start_destination = source_start;
        let mut end_destination = source_end;
        if source_start >= map.source && source_start <= map.source + map.range {
            let offset = source_start - map.source;
            start_destination = map.destination + offset;
        }
        if source_end >= map.source && source_end <= map.source + map.range {
            let offset = source_end - map.source;
            end_destination = map.destination + offset;
        }
        ranges.insert((start_destination, end_destination));
    }
    ranges
}

fn part2(input: String) -> String {
    //let mut maps = input.split("\n\n");
    //let seeds: Vec<usize> = maps
    //    .next()
    //    .unwrap()
    //    .split(':')
    //    .nth(1)
    //    .unwrap()
    //    .trim()
    //    .split(' ')
    //    .map(|x| x.parse::<usize>().unwrap())
    //    .collect();

    //let seed_to_soil = parse_map(maps.next().unwrap());
    //let soil_to_fertilizer = parse_map(maps.next().unwrap());
    //let fertilize_to_water = parse_map(maps.next().unwrap());
    //let water_to_light = parse_map(maps.next().unwrap());
    //let light_to_temperature = parse_map(maps.next().unwrap());
    //let temperature_to_humidity = parse_map(maps.next().unwrap());
    //let humidity_to_location = parse_map(maps.next().unwrap());

    //for seeds_range in seeds.chunks(2) {
    //    let end_seeds = seeds_range[0] + seeds_range[1];
    //    let soil = map_source_range_to_destination_range(seeds_range[0], end_seeds, &seed_to_soil);
    //    let fertilizer = map_source_range_to_destination_range(soil.0, soil.1, &soil_to_fertilizer);
    //    let water =
    //        map_source_range_to_destination_range(fertilizer.0, fertilizer.1, &fertilize_to_water);
    //    let light = map_source_range_to_destination_range(water.0, water.1, &water_to_light);
    //    let temperature =
    //        map_source_range_to_destination_range(light.0, water.1, &light_to_temperature);
    //    let humidity = map_source_range_to_destination_range(
    //        temperature.0,
    //        temperature.1,
    //        &temperature_to_humidity,
    //    );
    //    let location =
    //        map_source_range_to_destination_range(humidity.0, humidity.1, &humidity_to_location);
    //    //locations.insert(seed, location);
    //    println!(
    //        "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
    //        seeds_range, soil, fertilizer, water, light, temperature, humidity, location
    //    );
    //}
    //"".to_string()
    ////locations.values().min().unwrap().to_string()
    "".to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
