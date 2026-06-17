use rand::{Rng, RngExt};
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;

fn cal_target_y(x: &f64) -> f64 {
    cal_y(x, &vec![8, 25, 4, 45, 10, 17, 35])
}

fn cal_y(x: &f64, variables: &Vec<u8>) -> f64 {
    let a = variables[0] as f64;
    let b = variables[1] as f64;
    let c = variables[2] as f64;
    let d = variables[3] as f64;
    let e = variables[4] as f64;
    let f = variables[5] as f64;
    let g = variables[6] as f64;

    a * (b * (x / c).sin() + d * (x / e).cos()) + f * x - g
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
            rng.random_range(1..255),
            rng.random_range(1..255),
            rng.random_range(1..255),
            rng.random_range(1..255),
            rng.random_range(1..255),
            rng.random_range(1..255),
            rng.random_range(1..255),
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
            let x = i as f64 / 10.0;
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

    fn crossover(&self, other: &Self) -> Self {
        let mut rng = rand::rng();

        // Pick random bit split point between 1 and 55 inclusive
        let cut_bit = rng.random_range(1..=55);

        let mut child_genes = vec![0u8; 7];

        for i in 0..7 {
            let mut child_byte = 0u8;
            for bit_pos in 0..8 {
                let global_bit_idx = (i * 8) + bit_pos;

                // Determine which parent owns this specific bit
                let parent = if global_bit_idx < cut_bit {
                    self
                } else {
                    other
                };

                // Extract bit from parent byte
                let bit_val = (parent.genes[i] >> bit_pos) & 1;

                // Write bit into child byte
                child_byte |= bit_val << bit_pos;
            }
            child_genes[i] = child_byte;
        }

        let mut ind = Individual {
            genes: child_genes,
            fitness: 0.0,
        };

        ind.calc_fitness();

        ind
    }
}

fn main() {
    let pop_size = 100;
    let mutation_rate = 0.15;
    let mut rng = rand::rng();

    let mut population: Vec<Individual> = (0..pop_size).map(|_| Individual::new_random()).collect();

    let mut children_pupulation: Vec<Individual> = Vec::new();

    output_target_curve();

    for i in 0..pop_size {
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        for _ in 0..4 {
            population[rng.random_range(0..100)].mutate(mutation_rate);
        }

        let parent1 = tournament_select(&population, &mut rng);
        let parent2 = tournament_select(&population, &mut rng);

        let child = parent1.crossover(&parent2);

        output_curve(String::from("result_curve"), &child.genes);
        output_generation(&i, &child.fitness);

        children_pupulation.push(child);
    }

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    children_pupulation.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    println!("Target: [8, 25, 4, 45, 10, 17, 35]");
    println!(
        "Final best: {:?} fitness: {}",
        population[0].genes, population[0].fitness
    );
    println!(
        "Final best children: {:?} fitness: {}",
        children_pupulation[0].genes, children_pupulation[0].fitness
    );
}

fn output_target_curve() {
    output_curve(
        String::from("target_curve"),
        &vec![8, 25, 4, 45, 10, 17, 35],
    );
}

fn output_curve(file_name: String, variables: &Vec<u8>) {
    for i in 1..1000 {
        let x = i as f64 / 10.0;
        let y = cal_y(&x, variables);

        match output(file_name.clone(), &x, &y) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn output_generation(gen_i: &usize, gen_val: &f64) {
    match output(
        String::from("generation_iteration"),
        &(*gen_i as f64),
        gen_val,
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn tournament_select(pop: &[Individual], rng: &mut impl Rng) -> Individual {
    let mut best = &pop[rng.random_range(0..pop.len())];

    for _ in 0..4 {
        let contender = &pop[rng.random_range(0..pop.len())];
        if contender.fitness < best.fitness {
            best = contender;
        }
    }

    best.clone()
}

fn output(file_name: String, x: &f64, y: &f64) -> Result<()> {
    let mut target_curve = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&(file_name + ".csv"))?;

    writeln!(target_curve, "{},{}", x, y)?;

    Ok(())
}
