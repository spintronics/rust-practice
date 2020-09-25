struct Cubic {
  a: f64,
  b: f64,
  c: f64,
  d: f64,
}

#[derive(Debug)]
struct Root {
  value: f64,
  iterations: i32,
}

impl Cubic {
  fn evaluate(&self, x: &f64) -> f64 {
    self.a * x.powf(3.0) + self.b * x.powf(2.0) + self.c * x + self.d
  }
  fn find_root(&self, start: f64, end: f64, tol: f64) -> Option<Root> {
    let mut iterations = 0;
    let increasing = self.evaluate(&end) > self.evaluate(&start);
    let mut start = start;
    let mut end = end;
    loop {
      iterations += 1;
      if (end - start).abs() < tol {
        break None;
      }

      let mid_point = start + ((end - start).abs() / 2.0);
      let value = self.evaluate(&mid_point);

      if (value.abs() < tol) || value == 0.0 {
        break Some(Root {
          value: mid_point,
          iterations,
        });
      }
      if value < 0.0 {
        if increasing {
          start = mid_point;
        } else {
          end = mid_point;
        }
      } else {
        if increasing {
          end = mid_point;
        } else {
          start = mid_point;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn increasing() {
    let [a, b, c, d] = [1.0, 1.0, 1.0, 1.0];
    let equation = Cubic { a, b, c, d };

    match equation.find_root(-3.0, 3.0, 1e-7) {
      Some(root) => assert!((root.value + 1.0).abs() < 1e-7),
      None => panic!("the disco"),
    }
  }

  #[test]
  fn decreasing() {
    let [a, b, c, d] = [2.0, 4.0, -3.0, -3.0];
    let equation = Cubic { a, b, c, d };

    match equation.find_root(-1.0, 0.0, 1e-7) {
      Some(root) => {
        println!("{:?}", root);
        assert!((root.value + 0.634).abs() < 1e-4);
      }
      None => panic!("the disco"),
    }
  }

  #[test]
  fn no_solution() {
    let [a, b, c, d] = [5.0, 1.0, 2.0, -3.0];
    let equation = Cubic { a, b, c, d };

    match equation.find_root(-1.0, 0.0, 1e-7) {
      Some(_) => panic!("expected no solution"),
      None => (),
    }
  }
}

fn main() {
  let [a, b, c, d] = [2.0, 4.0, -3.0, -3.0];
  let equation = Cubic { a, b, c, d };

  match equation.find_root(-1.0, 0.0, 1e-7) {
    Some(root) => println!(
      "found the root {} in {} iterations",
      root.value, root.iterations
    ),
    None => println!("No root found"),
  }
}
