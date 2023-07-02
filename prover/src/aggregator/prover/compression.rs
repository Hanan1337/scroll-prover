use super::Prover;
use crate::Proof;
use aggregator::CompressionCircuit;
use anyhow::Result;
use halo2_proofs::poly::commitment::Params;
use rand::Rng;
use snark_verifier_sdk::Snark;

impl Prover {
    pub fn gen_comp_snark(
        &mut self,
        id: &str,
        is_fresh: bool,
        degree: u32,
        mut rng: impl Rng + Send,
        prev_snark: Snark,
    ) -> Snark {
        let mut params = self.params.clone();
        params.downsize(degree);

        let circuit = CompressionCircuit::new(&params, prev_snark, is_fresh, &mut rng);
        self.gen_snark(id, &mut rng, &params, circuit)
    }

    pub fn gen_comp_evm_proof(
        &mut self,
        id: &str,
        is_fresh: bool,
        degree: u32,
        mut rng: impl Rng + Send,
        prev_snark: Snark,
    ) -> Result<Proof> {
        let mut params = self.params.clone();
        params.downsize(degree);

        let circuit = CompressionCircuit::new(&params, prev_snark, is_fresh, &mut rng);
        self.gen_evm_proof(id, &mut rng, &params, circuit)
    }
}
