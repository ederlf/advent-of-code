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

fn map_destination_to_source(destination: usize, maps: &Vec<Map>) -> usize {
    for map in maps {
        if destination >= map.destination && destination < map.destination + map.range {
            let offset = destination - map.destination;
            return map.source + offset;
        }
    }
    destination
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

fn part2(input: String) -> String {
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

    let mut ranges = Vec::new();
    for seeds_range in seeds.chunks(2) {
        ranges.push(seeds_range[0]..seeds_range[0] + seeds_range[1]);
    }
    let mut location = 0;
    'outer: loop {
        location += 1;
        let humidity = map_destination_to_source(location, &humidity_to_location);
        let temperature = map_destination_to_source(humidity, &temperature_to_humidity);
        let light = map_destination_to_source(temperature, &light_to_temperature);
        let water = map_destination_to_source(light, &water_to_light);
        let fertilizer = map_destination_to_source(water, &fertilize_to_water);
        let soil = map_destination_to_source(fertilizer, &soil_to_fertilizer);
        let seed = map_destination_to_source(soil, &seed_to_soil);
        for range in &ranges {
            if range.contains(&seed) {
                break 'outer;
            }
        }
    }
    location.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
