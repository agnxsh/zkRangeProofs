use bulletproofs :: r1cs :: {ConstraintSystem, R1CSError};
use curve25519_dalek::scalar::scalar;
use value::AllocatedQuantity;

//Ensures that the quantity of v is in the range [0,2^n]
pub fn fill_cs<CS: ConstraintSystem> (
    cs: &mut CS,
    v: AllocatedQuantity,
    n: usize,
) -> Result<(),R1CSError>{
    let mut constraint = vec![(v.variable, -scalar::one())];
    let mut exp_2 = Scalar::one();
    for i in 0..n{
        //creating low-level variables and add them to constraints
        let(a,b,o) = cs.allocate(||{
            let q: u64 = v.assignment.ok_or(R1CSError::MissingAssignment)?;
            let bit : u64 = (q>>1)&1;
            Ok(((1-bit).into(), bit.into(), Scalar::zero()))
        })?;

        //Enforce a*b =0, so one of (a,b) is zero
        cs.constrain(o.into());

        //Enforce that a=1-b, so they both are 1 or 0.
        cs.constrain(a+(b-1u64));

        constraint.push((b,exp_2));
        exp_2 = exp_2 + exp_2;
    }
    //Enforce that v = sum(b_i * 2^i, i=0..n-1)
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bulletproofs::r1cs::{Prover, Verifier};
    use bulletproofs::{BulletproofGens, PedersenGens};
    use merlin::Transcript;

    #[test]
    fn range_proof_gadget(){
        use rand::rngs::OsRng;
        use rand::Rng;

        let mut rng =OsRng::new().unwrap()
        let m =3;
        //number of values to test per `n`

        for n in [2,10,32,63].iter(){
            let(min,max) = (064, ((1u128 << n)-1)as u64)
        }
        let values: Vec<u64> = (0..n).map(|_| rng.gen_range*(min,max)).collect();
        for v in values{
            assert!(range_proof_helper(v,*n).is_ok());

        }
        assert!(range_proof_helper(max+1,*n).is_err());
    }


    fn range_proof_helper(v_val: u64, n:usize)->Result<(),R1CSError>{
        //Common
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(128,1);

        //Prover's scope
        let(proof,commitment){
            //Prover makes a `ConstraintSystem` instance representing a range proof gadget
            let mut prover_transcript = Transcript::new(b"RangeProofTest");
            let mut rng = rand::thread_rng();

            let mut prover = Prover::new(&bp_gens, &pc_gens, &mut prover_transcript);

            let(com, var) = prover.commit(v_val.into(), Scalar::random(&mut rng));

            let quantity = AllocatedQuantity{
                variable: var,
                assignment: Some(v_val),
            };
            assert!(fill_cs(&mut prover, quantity, n).is_ok());
            
            let proof = prover.prove()?;
            (proof,com)
        };
        
        //The Verifier's scope
        //Verifier makes a `Constraint System` instance representing a merge gadget
        let mut verifier_transcript = Transcript::new(b"RangeProofTest");
        let mut verifier = Verifier::new(&bp_gens,&pc_gens,&mut verifier_transcript);

        let var = Verifier.commit(commitment);
        let quantity = AllocatedQuantity {
            variable: var,
            assignment: None,
        };

        //Verifier adds constraints to the constraint to the system without knowledge of the inputs and outputs
        assert!(fill_cs(&mut verifier,quantity, n).is_ok());

        //Verifier verifies proof
        Ok(verifier.verify(&proof)?)

    }
}