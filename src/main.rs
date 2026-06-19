use rand::{Rng, RngExt};
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use std::thread;
use std::time::Duration;

fn cal_target_y(x: &f64) -> f64 {
    cal_y(x, &vec![8, 25, 4, 45, 10, 17, 35])
}

fn cal_y(x: &f64, variables: &Vec<u8>) -> f64 {
    if variables.len() < 7 {
        return 0.0;
    }

    let a = variables[0] as f64;
    let b = variables[1] as f64;
    let c = variables[2] as f64;
    let d = variables[3] as f64;
    let e = variables[4] as f64;
    let f = variables[5] as f64;
    let g = variables[6] as f64;

    let c = if c == 0.0 { 1.0 } else { c };
    let e = if e == 0.0 { 1.0 } else { e };

    a * (b * (x / c).sin() + d * (x / e).cos()) + f * (*x) - g
}

#[derive(Clone, Debug)]
struct Individual {
    genes: Vec<u8>,
    fitness: f64,
}

impl Individual {
    fn new_random() -> Self {
        let mut rng = rand::rng();

        let genes = vec![
            rng.random_range(1..=255),
            rng.random_range(1..=255),
            rng.random_range(1..=255),
            rng.random_range(1..=255),
            rng.random_range(1..=255),
            rng.random_range(1..=255),
            rng.random_range(1..=255),
        ];

        let mut ind = Individual {
            genes,
            fitness: 0.0,
        };

        ind.calc_fitness();
        ind
    }

    fn calc_fitness(&mut self) {
        let max_gen = *self.genes.iter().max().unwrap_or(&1);

        let factor = if max_gen > 0 { 255 / max_gen } else { 1 };

        let safe_factor = if factor == 0 { 1 } else { factor };

        let after_weight_gens: Vec<u8> = self
            .genes
            .iter()
            .map(|&g| {
                let value = g / safe_factor;

                if value == 0 {
                    return 1;
                }

                value
            })
            .collect();

        let mut error: f64 = 0.0;

        for i in 1..1000 {
            let x = (i as f64) / 10.0;
            let y_target = cal_target_y(&x);
            let y = cal_y(&x, &after_weight_gens);

            error += (y_target - y).abs();
        }

        self.fitness = error;
    }

    fn mutate(&mut self, rate: f64) {
        let mut rng = rand::rng();

        for byte in self.genes.iter_mut() {
            for bit_pos in 0..8 {
                if rng.random_bool(rate) {
                    *byte ^= 1 << bit_pos;
                }
            }
        }

        self.calc_fitness();
    }

    fn crossover(&self, other: &Self) -> (Self, Self) {
        let mut rng = rand::rng();
        let cut_point = rng.random_range(0..=55);

        let mut p1_buf = [0u8; 8];
        let mut p2_buf = [0u8; 8];

        p1_buf[..7].copy_from_slice(&self.genes[..7]);
        p2_buf[..7].copy_from_slice(&other.genes[..7]);

        let p1 = u64::from_le_bytes(p1_buf);
        let p2 = u64::from_le_bytes(p2_buf);

        let mask = (1u64 << cut_point) - 1;

        let c1 = (p1 & mask) | (p2 & !mask);
        let c2 = (p2 & mask) | (p1 & !mask);

        let c1_genes = c1.to_le_bytes()[..7].to_vec();
        let c2_genes = c2.to_le_bytes()[..7].to_vec();

        let mut child = Individual {
            genes: c1_genes,
            fitness: 0.0,
        };

        let mut child2 = Individual {
            genes: c2_genes,
            fitness: 0.0,
        };

        child.calc_fitness();
        child2.calc_fitness();

        (child, child2)
    }
}

fn main() {
    let elitism = false;
    let pop_size = 100;
    let mutation_rate = 0.15;
    let mut rng = rand::rng();

    let mut population: Vec<Individual> = (0..pop_size).map(|_| Individual::new_random()).collect();

    let mut poblation_father = Vec::with_capacity(pop_size / 2);
    let mut poblation_mothers = Vec::with_capacity(pop_size / 2);

    let mut next_generation = Vec::with_capacity(pop_size);

    for _ in 0..(mutation_rate * 10.0) as usize {
        let target_idx = rng.random_range(0..pop_size);
        population[target_idx].mutate(mutation_rate);
    }

    output_target_curve();

    if elitism {
        // Lower fitness error goes first
        population.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        next_generation.push(population[0].clone());
    }

    for _ in 0..pop_size / 2 {
        let parent1 = tournament_select(&population, &mut rng);
        let parent2 = tournament_select(&population, &mut rng);

        poblation_father.push(parent1);
        poblation_mothers.push(parent2);
    }

    for _ in 0..(mutation_rate * 10.0) as usize {
        let target_idx = rng.random_range(0..poblation_mothers.len() / 2);
        poblation_father[target_idx].mutate(mutation_rate);
    }
    for _ in 0..(mutation_rate * 10.0) as usize {
        let target_idx = rng.random_range(0..poblation_father.len() / 2);
        poblation_mothers[target_idx].mutate(mutation_rate);
    }

    for i in 0..pop_size {
        if i % 2 == 0 {
            let parent1 = tournament_select(&poblation_father, &mut rng);
            let parent2 = tournament_select(&poblation_mothers, &mut rng);

            let (child1, child2) = parent1.crossover(&parent2);

            next_generation.push(child1);
            next_generation.push(child2);
        }
    }

    for _ in 0..(mutation_rate * 10.0) as usize {
        let target_idx = rng.random_range(0..next_generation.len() / 2);
        next_generation[target_idx].mutate(mutation_rate);
    }

    next_generation.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    for i in 0..next_generation.len() {
        output_curve("result_curve", &next_generation[i].genes);
        output_generation(i as f64, next_generation[i].fitness);

        thread::sleep(Duration::from_millis(50));
    }

    let best_child = &next_generation.last().unwrap();

    println!("Total population size: {}", next_generation.len());
    println!("Target: [8, 25, 4, 45, 10, 17, 35]");
    println!(
        "Final best child: {:?} fitness (error): {}",
        best_child.genes, best_child.fitness
    );
}

fn tournament_select(pop: &[Individual], rng: &mut impl Rng) -> Individual {
    let mut best = &pop[rng.random_range(0..pop.len())];

    for _ in 0..=3 {
        let contender = &pop[rng.random_range(0..pop.len())];
        // Lower error is better
        if contender.fitness < best.fitness {
            best = contender;
        }
    }

    best.clone()
}

fn output_target_curve() {
    output_curve("target_curve", &vec![8, 25, 4, 45, 10, 17, 35]);
}

fn output_curve(file_name: &str, variables: &Vec<u8>) {
    let file_path = format!("{}.csv", file_name);
    let file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {}: {}", file_path, e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    for i in 1..1000 {
        let x = (i as f64) / 10.0;
        let y = cal_y(&x, variables);

        if let Err(e) = writer_output(&mut writer, x, y) {
            eprintln!("Write error: {}", e);
        }
    }

    if let Err(e) = writer.flush() {
        eprintln!("Flush error: {}", e);
    }
}

fn writer_output<W: Write>(writer: &mut W, x: f64, y: f64) -> io::Result<()> {
    writeln!(writer, "{},{}", x, y)
}

fn output_generation(gen_i: f64, gen_val: f64) {
    let file_path = "generation_iteration.csv";
    let file = match OpenOptions::new().create(true).append(true).open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {}: {}", file_path, e);
            return;
        }
    };
    let mut writer = BufWriter::new(file);
    if let Err(e) = writer_output(&mut writer, gen_i, gen_val) {
        eprintln!("Write error: {}", e);
    }
}
