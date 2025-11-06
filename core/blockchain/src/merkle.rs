//! Merkle tree implementation for transaction verification

use lean_crypto::Hash256;
use lean_transaction::Transaction;

/// A Merkle tree for efficient transaction verification
#[derive(Clone, Debug)]
pub struct MerkleTree {
    /// Tree nodes (layer by layer, bottom to top)
    nodes: Vec<Vec<Hash256>>,

    /// Root hash
    root: Hash256,
}

impl MerkleTree {
    /// Create a new Merkle tree from transaction hashes
    pub fn new(leaves: Vec<Hash256>) -> Self {
        if leaves.is_empty() {
            return Self {
                nodes: vec![vec![Hash256::zero()]],
                root: Hash256::zero(),
            };
        }

        let mut nodes = vec![leaves];

        // Build tree layer by layer
        while nodes.last().unwrap().len() > 1 {
            let current_layer = nodes.last().unwrap();
            let mut next_layer = Vec::new();

            for chunk in current_layer.chunks(2) {
                let left = chunk[0];
                let right = if chunk.len() == 2 {
                    chunk[1]
                } else {
                    // Duplicate last node if odd number
                    chunk[0]
                };

                let combined = Self::combine_hashes(left, right);
                next_layer.push(combined);
            }

            nodes.push(next_layer);
        }

        let root = nodes.last().unwrap()[0];

        Self { nodes, root }
    }

    /// Create a Merkle tree from transactions
    pub fn from_transactions(transactions: &[Transaction]) -> Self {
        if transactions.is_empty() {
            return Self::new(Vec::new());
        }

        let mut leaves: Vec<Hash256> = transactions
            .iter()
            .map(|tx| {
                // Clone transaction and compute hash
                let mut tx_clone = tx.clone();
                tx_clone.hash()
            })
            .collect();

        Self::new(leaves)
    }

    /// Combine two hashes into one
    fn combine_hashes(left: Hash256, right: Hash256) -> Hash256 {
        let mut hasher = lean_crypto::Hasher::new();
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        hasher.finalize()
    }

    /// Get the root hash
    pub fn root(&self) -> Hash256 {
        self.root
    }

    /// Get a Merkle proof for a transaction at the given index
    pub fn proof(&self, index: usize) -> Option<MerkleProof> {
        if self.nodes.is_empty() || index >= self.nodes[0].len() {
            return None;
        }

        let mut proof_hashes = Vec::new();
        let mut current_index = index;

        // Traverse up the tree, collecting sibling hashes
        for layer in &self.nodes[..self.nodes.len() - 1] {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < layer.len() {
                proof_hashes.push(layer[sibling_index]);
            } else {
                // Duplicate if no sibling
                proof_hashes.push(layer[current_index]);
            }

            current_index /= 2;
        }

        Some(MerkleProof {
            leaf_index: index,
            leaf_hash: self.nodes[0][index],
            proof_hashes,
            root: self.root,
        })
    }

    /// Verify a Merkle proof
    pub fn verify_proof(proof: &MerkleProof) -> bool {
        let mut current_hash = proof.leaf_hash;
        let mut index = proof.leaf_index;

        for sibling_hash in &proof.proof_hashes {
            current_hash = if index % 2 == 0 {
                Self::combine_hashes(current_hash, *sibling_hash)
            } else {
                Self::combine_hashes(*sibling_hash, current_hash)
            };
            index /= 2;
        }

        current_hash == proof.root
    }

    /// Get the number of leaves
    pub fn len(&self) -> usize {
        if self.nodes.is_empty() {
            0
        } else {
            self.nodes[0].len()
        }
    }

    /// Check if tree is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A Merkle proof for transaction inclusion
#[derive(Clone, Debug)]
pub struct MerkleProof {
    /// Index of the leaf in the tree
    pub leaf_index: usize,

    /// Hash of the leaf
    pub leaf_hash: Hash256,

    /// Sibling hashes along the path to root
    pub proof_hashes: Vec<Hash256>,

    /// Root hash
    pub root: Hash256,
}

impl MerkleProof {
    /// Verify this proof
    pub fn verify(&self) -> bool {
        MerkleTree::verify_proof(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::new(Vec::new());
        assert_eq!(tree.root(), Hash256::zero());
        assert!(tree.is_empty());
    }

    #[test]
    fn test_single_leaf() {
        let leaf = Hash256::hash(b"test");
        let tree = MerkleTree::new(vec![leaf]);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.root(), leaf);
    }

    #[test]
    fn test_multiple_leaves() {
        let leaves: Vec<Hash256> = (0..4)
            .map(|i| Hash256::hash(&[i]))
            .collect();

        let tree = MerkleTree::new(leaves.clone());
        assert_eq!(tree.len(), 4);
        assert_ne!(tree.root(), Hash256::zero());
    }

    #[test]
    fn test_merkle_proof() {
        let leaves: Vec<Hash256> = (0..8)
            .map(|i| Hash256::hash(&[i]))
            .collect();

        let tree = MerkleTree::new(leaves.clone());

        // Get proof for each leaf and verify
        for i in 0..leaves.len() {
            let proof = tree.proof(i).unwrap();
            assert!(proof.verify());
            assert_eq!(proof.leaf_hash, leaves[i]);
        }
    }

    #[test]
    fn test_invalid_proof() {
        let leaves: Vec<Hash256> = (0..4)
            .map(|i| Hash256::hash(&[i]))
            .collect();

        let tree = MerkleTree::new(leaves);

        let mut proof = tree.proof(0).unwrap();
        // Tamper with the proof
        proof.leaf_hash = Hash256::hash(b"tampered");

        assert!(!proof.verify());
    }

    #[test]
    fn test_odd_number_of_leaves() {
        let leaves: Vec<Hash256> = (0..5)
            .map(|i| Hash256::hash(&[i]))
            .collect();

        let tree = MerkleTree::new(leaves.clone());
        assert_eq!(tree.len(), 5);

        // Verify proofs still work with odd number
        for i in 0..leaves.len() {
            let proof = tree.proof(i).unwrap();
            assert!(proof.verify());
        }
    }
}
