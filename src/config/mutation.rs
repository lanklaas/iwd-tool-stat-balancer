pub struct Mutation {
    pub initial_population_size: usize,
    pub run_for_generations: usize,
    pub stat_mutate_max_mult: f32,
    pub elitism_percentage: f32,
    pub crossover_percentage: f32,
    pub evolve_stat_percentage: f32,
}

impl Mutation {
    pub const fn default() -> Self {
        Self {
            initial_population_size: 100,
            run_for_generations: 300,
            stat_mutate_max_mult: 2.0,
            elitism_percentage: 0.15,
            crossover_percentage: 0.15,
            evolve_stat_percentage: 0.2,
        }
    }
}
