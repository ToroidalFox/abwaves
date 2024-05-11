use noise::utils::*;
use noise::{Fbm, Perlin};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() {
    let fbm = Fbm::<Perlin>::new(69420);

    PlaneMapBuilder::new(fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file(std::path::Path::new("fbm_perlin_test.png"));
}

fn test_rng() {
    let seeds = vec![69420, 0, 42, 69, 420];

    for seed in seeds {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let f_rand = rng.gen::<f64>();
        dbg!(f_rand);
    }
}
