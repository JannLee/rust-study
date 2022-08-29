trait RouteStrategy {
    fn method(&self, target: u128);
}

struct SimpleStrategy;

impl RouteStrategy for SimpleStrategy {
    fn method(&self, target: u128) {
        for _num in 2..target {
            if target % _num == 0 {
                println!("{} is not prime.", target);
                return;
            }
        }
        println!("{} is prime.", target);
    }
}

struct HalfStrategy;

impl RouteStrategy for HalfStrategy {
    fn method(&self, target: u128) {
        for _num in 2..target/2 {
            if target % _num == 0 {
                println!("{} is not prime.", target);
                return;
            }
        }
        println!("{} is prime.", target);
    }
}

struct SqrtStrategy;

impl RouteStrategy for SqrtStrategy {
    fn method(&self, target: u128) {
        for _num in 2..(target as f64).sqrt() as u128 {
            if target % _num == 0 {
                println!("{} is not prime.", target);
                return;
            }
        }
        println!("{} is prime.", target);
    }
}

struct Solver<T: RouteStrategy> {
    route_strategy: T,
}

impl<T: RouteStrategy> Solver<T> {
    pub fn new(route_strategy: T) -> Self {
        Self { route_strategy }
    }

    pub fn method(&self, target: u128) {
        self.route_strategy.method(target);
    }
}

fn main() {
    let solver = Solver::new(SimpleStrategy);
    solver.method(524_287);

    let solver = Solver::new(HalfStrategy);
    solver.method(6_027);

    let solver = Solver::new(SqrtStrategy);
    solver.method(2_147_483_647);
}