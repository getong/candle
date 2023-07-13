//! Various optimization algorithms.
use candle::{Result, Tensor, Var};

#[derive(Debug)]
pub struct SGD {
    vars: Vec<Var>,
    learning_rate: f64,
}

impl SGD {
    pub fn new(learning_rate: f64) -> Self {
        Self {
            vars: vec![],
            learning_rate,
        }
    }

    pub fn into_inner(self) -> Vec<Var> {
        self.vars
    }

    pub fn learning_rate(&self) -> f64 {
        self.learning_rate
    }

    pub fn push(&mut self, var: Var) {
        self.vars.push(var)
    }

    pub fn backward_step(&self, loss: &Tensor) -> Result<()> {
        let grads = loss.backward()?;
        for var in self.vars.iter() {
            if let Some(grad) = grads.get(var) {
                var.set(&var.sub(&(grad * self.learning_rate)?)?)?
            }
        }
        Ok(())
    }
}
