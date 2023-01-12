<h1>Zero-Knowledge Range Proofs</h1>
<h4>Zero-knowledge range proofs are a key building block for confidential transaction systems, such as Confidential Transactions for Bitcoin, Chain’s Confidential Assets, and many other protocols. Range proofs allow a verifier to ensure that secret values, such as asset amounts, are nonnegative. This prevents a user from forging value by secretly using a negative amount. Since every transaction involves one or more range proofs, their efficiency, both in terms of proof size and verification time, is key to transaction performance.

In 2017, Bünz, Bootle, Boneh, Poelstra, Wuille, and Maxwell published Bulletproofs, which dramatically improves proof performance both in terms of proof size and verification time. In addition, it allows for proving a much wider class of statements than just range proofs.</h4>

<h2>Definitions</h2>
<h4>Commitment — a commitment ```Com(m)``` to message ```m``` is hiding, which means it does not reveal ```m```. It is also binding, which means that if you make a commitment to ```m```, you cannot open it to a different message ```m’```. In the context of Bulletproofs, commitment refers to a ```Pedersen commitment```, which has the additional property of being ```additively homomorphic```, which means that ```Com(a) + Com(b) = Com(c)``` only if ```a + b = c```.

Zero-knowledge proof — a proof that a certain statement is true, without revealing the secret that the statement is about. This is usually done by making a commitment to the secret that the statement is about, and sharing the commitment along with the proof.

Zero-knowledge range proof — a proof that a secret value is in a certain range (from `0` to `2^n — 1`). The proof is usually shared with a commitment to the secret value so that it can be verified.

Inner product — the sum of an entry-wise multiplication of two vectors.

</h4>
