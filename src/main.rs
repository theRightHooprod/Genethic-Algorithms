use rand::{Rng, RngExt};
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;

fn target_curve(x: f64) -> f64 {
    8.0 * (25.0 * (x / 4.0).sin() + 45.0 * (x / 10.0).cos()) + 17.0 * x - 35.0
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

    fn calc_fitness(&mut self) {}

    fn mutate(&mut self, rate: f64) {}

    fn crossover(&self, other: &Self) -> Self {
        Self {
            genes: vec![0, 0, 0, 0, 0, 0, 0],
            fitness: 0.0,
        }
    }
}

fn main() {
    let pop_size = 100;
    let generations = 1000;
    let mutation_rate = 0.1;
    let mut rng = rand::rng();

    let mut population: Vec<Individual> = (0..pop_size).map(|_| Individual::new_random()).collect();

    output_target_curve();

    for fgen in 0..generations {
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        if fgen % 100 == 0 {
            println!("Gen No. {}: Best genes: {:?}", fgen, population[0].genes);
        }

        let mut next_gen = vec![population[0].clone(), population[1].clone()];

        while next_gen.len() < pop_size {
            let parent1 = tournament_select(&population, &mut rng);
            let parent2 = tournament_select(&population, &mut rng);

            let mut child = parent1.crossover(&parent2);
            child.mutate(mutation_rate);
            next_gen.push(child);
        }
        population = next_gen;
    }

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    println!("Final Best: {:?}", population[0].genes);
    println!("Target: [2.5, 1.3, 4.2]");
}

fn output_target_curve() {
    for i in 1..1000 {
        let x = i as f64 / 10.0;
        let y = target_curve(x);

        match output(String::from("target_curve"), x, y) {
            Ok(_) => println!("Target cruve exported"),
            Err(e) => eprintln!("{}", e),
        }
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
