use lubeck::traits::*;
use std::rc::Rc;

use lubeck::common::reader::Reader;

pub struct Config {
    name: String,
    age: f32,
    dog_year_factor: f32,
}

fn main() {
    let conf = Rc::new(Config {
        name: "Lubeck".to_string(),
        age: 0.05,
        dog_year_factor: 7.0,
    });

    {
        // Basic examples
        // ==============

        let r_age = Reader::new(|config: Rc<Config>| config.age);
        let age = r_age.run(conf.clone());
        println!("age: {}", age);

        let r_name = Reader::new(|config: Rc<Config>| config.name.clone());
        let name = r_name.run(conf.clone());
        println!("name: {}\n", name);
    }

    {
        // Functor example
        // ===============

        let r_double_age = Reader::new(|config: Rc<Config>| config.age).fmap(|age| age * 2.0);
        let double_age = r_double_age.run(conf.clone());
        println!("double age: {}\n", double_age);
    }

    {
        // Applicative example
        // ===================

        let r_age = Reader::new(|config: Rc<Config>| config.age);
        let r_dog_year_calculator = Reader::new(|config: Rc<Config>| {
            let dog_year_factor = config.dog_year_factor;
            Box::new(move |human_years| human_years * dog_year_factor)
        });

        let r_dog_age = r_age.app(r_dog_year_calculator);
        let dog_age = r_dog_age.run(conf.clone());
        println!("dog age: {}\n", dog_age);
    }

    {
        // Monad example
        // =============

        let r_age = Reader::new(|config: Rc<Config>| config.age);
        let construct_name_reader_with_age = |age| {
            Reader::new(move |config: Rc<Config>| format!("{} is {} years old", config.name, age))
        };

        let r_name_and_age = r_age.bind(construct_name_reader_with_age);
        let name_and_age = r_name_and_age.run(conf);
        println!("name with age: {}\n", name_and_age);
    }
}
