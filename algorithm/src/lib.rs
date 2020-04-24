#[derive(Debug)]
pub enum Status {
    ///Not infected
    Susceptible,
    ///Infected but asymptomatic
    Exposed,
    Infectious,
    Removed,
}

#[derive(Debug)]
pub struct Person {
    status: Status,
    infection_radius: f32,
    symptomatic: bool,
    p_symptomatic_on_infection: f32,
    max_speed: f32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sim() {
        const N: f32 = 1000.0;
        const iter: usize = 50;

        let mut I = 1.0;
        let mut S = N - I;
        let mut R = 0.0;

        //avg contact per person
        let beta = 1.0;
        //the avg # of recovery per day
        let recovery = 5.0;
        let gamma = 1.0 / recovery;
        println!("time, num, group");
        let mut res = vec![];
        for _ in 0..iter {
            let s0 = S as f32;
            let i0 = I as f32;
            let r0 = R as f32;
            S += -beta * (I / N) as f32 * s0;
            // println!("{}, S", S);
            I += beta * (I / N) as f32 * s0 - gamma * i0;
            // println!("{}, I", I);
            R += gamma * i0;
            // println!("{}, R", R);
            res.push((S, I, R));
        }
        for (i, (s, _, _)) in res.iter().enumerate() {
            println!("{}, {}, S", i, s)
        }
        for (i, (_, inf, _)) in res.iter().enumerate() {
            println!("{}, {}, I", i, inf)
        }
        for (i, (_, _, r)) in res.iter().enumerate() {
            println!("{}, {}, R", i, r)
        }
    }
}
