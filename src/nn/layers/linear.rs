#![allow(unreachable_patterns, dead_code)]
use crate::{Result, Tensor};

/// Linear layer, applies matmul(x, W) + b
pub struct Linear {
    weight: Tensor,
    bias: Tensor,
}

impl Linear {
    pub fn new(weight: Tensor, bias: Tensor) -> Self {
        Self { weight, bias }
    }

    /// Forward pass
    pub fn forward(&self, tensor: &Tensor) -> Result<Tensor> {
        // TODO
        tensor.matmul(&self.weight)
        // x.broadcast_add(&self.bias)
    }
}

/// Linear layer, applies matmul(x, W.T) + b
pub struct LinearT {
    weight: Tensor,
    bias: Tensor,
}

impl LinearT {
    pub fn new(weight: Tensor, bias: Tensor) -> Self {
        Self { weight, bias }
    }

    pub fn forward(&self, tensor: &Tensor) -> Result<Tensor> {
        tensor.matmul(&self.weight.t()?)
        //TODO
    }
}

pub struct UnbiasedLinear {
    weight: Tensor,
}

impl UnbiasedLinear {
    pub fn new(weight: Tensor) -> Self {
        Self {
            weight: weight.t().unwrap(),
        }
    }

    pub fn forward(&self, tensor: &Tensor) -> Result<Tensor> {
        tensor.matmul(&self.weight)
    }
}

#[cfg(test)]
#[cfg(feature = "cpu")]
mod tests {
    use super::*;

    use crate::cpu::f32::Tensor;

    #[test]
    fn test_linear() {
        let zeros = Tensor::zeros(vec![2, 2]);
        let weights = Tensor::zeros(vec![3, 2]);
        let bias = Tensor::zeros(vec![3]);
        let mut out = Tensor::zeros(vec![2, 3]);

        let linear = Linear::new(weights, bias);

        linear.forward(&zeros, &mut out).unwrap();
    }
}
