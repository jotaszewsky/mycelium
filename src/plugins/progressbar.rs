pub struct Progressbar {
    count: usize,
    progress: usize,
    iteration: usize
}

impl Progressbar {

    pub fn new(count: usize) -> Progressbar {
        Progressbar { count, progress: 0, iteration: 0 }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        self.iteration += 1;
        if self.count >= 50 && (self.iteration % ((self.count / 50) as usize)) == 0 && self.progress < 100 {
            self.progress += 2;
            eprint!("\r{}% ", self.progress);
            for _ in 0..(self.progress) {
                eprint!(".");
            }
        }
        if self.iteration == self.count {
            println!("");
        }
        Ok(())
    }
}
