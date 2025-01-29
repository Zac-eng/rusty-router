pub struct RoundRobinScheduler {
  num_of_wan: usize,
  next: usize
}

impl RoundRobinScheduler {
  pub fn new(num_of_wan: usize) -> RoundRobinScheduler {
    return RoundRobinScheduler {
      num_of_wan,
      next: 0
    }
  }
  pub fn next(&mut self) -> usize {
    let tmp: usize = self.next;
    self.next = (self.next + 1) % self.num_of_wan;
    return tmp
  }
}
