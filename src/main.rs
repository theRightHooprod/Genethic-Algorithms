use rand::{Rng, RngExt};
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;

fn cal_target_y(x: f64) -> f64 {
    cal_y(x, &vec![8, 25, 4, 45, 10, 17, 35])
}

fn cal_y(x: f64, variables: &Vec<u8>) -> f64 {
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
    fn new_random(i: &usize) -> Self {
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

        ind.calc_fitness(i);
        ind
    }

    fn calc_fitness(&mut self, i: &usize) {
        let max_gen = *self
            .genes
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        println!("{}", max_gen);

        let weight = 255 / max_gen;

        let after_weight_gens: Vec<u8> = self.genes.iter().map(|&g| (g / weight) as u8).collect();

        let x = *i as f64 / 10.0;
        let y_target = cal_target_y(x);
        let y = cal_y(x, &after_weight_gens);

        let error = (y_target - y).abs();

        self.fitness = error;
    }

    fn mutate(&mut self, i: &usize, rate: f64) {
        // let mut rng = rand::rng();
        //
        // for byte in self.genes.iter_mut() {
        //     for bit_pos in 0..8 {
        //         if rng.random_bool(rate) {
        //             *byte ^= (1 << bit_pos) as u8; // Cast literal to u8 explicitly
        //         }
        //     }
        // }
        //
        // self.calc_fitness(i);
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            genes: vec![0, 0, 0, 0, 0, 0, 0],
            fitness: 0.0,
        }
    }
}

fn main() {
    let pop_size = 100;
    let mutation_rate = 0.15;
    let mut rng = rand::rng();

    let mut population: Vec<Individual> =
        (0..pop_size).map(|i| Individual::new_random(&i)).collect();

    output_target_curve();

    for fgen in 1..pop_size {
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        if fgen % 20 == 0 {
            output_generation(fgen, population[0].fitness);
            println!("Gen No. {}: Best genes: {:?}", fgen, population[0].genes);
        }

        let mut next_gen = vec![population[0].clone(), population[1].clone()];

        while next_gen.len() < pop_size {
            let parent1 = tournament_select(&population, &mut rng);
            let parent2 = tournament_select(&population, &mut rng);

            let mut child = parent1.crossover(&parent2);
            child.mutate(&fgen, mutation_rate);
            next_gen.push(child);
        }
        population = next_gen;
    }

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    println!("Final Best: {:?}", population[0].genes);
    println!("Target: [2.5, 1.3, 4.2]");
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
        let y = cal_y(x, variables);

        match output(file_name.clone(), x, y) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn output_generation(gen_i: usize, gen_val: f64) {
    match output(String::from("generation_iteration"), gen_i as f64, gen_val) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn tournament_select(pop: &[Individual], rng: &mut impl Rng) -> Individual {
    let mut best = &pop[rng.random_range(0..pop.len())];
    for _ in 0..3 {
        let contender = &pop[rng.random_range(0..pop.len())];
        if contender.fitness > best.fitness {
            best = contender;
        }
    }
    best.clone()
}

fn output(file_name: String, x: f64, y: f64) -> Result<()> {
    let mut target_curve = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name + ".csv")?;

    writeln!(target_curve, "{},{}", x, y)?;

    Ok(())
}
