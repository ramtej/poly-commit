use algebra::Field;
pub use ff_fft::DensePolynomial as Polynomial;
use rand_core::RngCore;
use std::borrow::Cow;

/// Defines the minimal interface for public params for any polynomial
/// commitment scheme.
pub trait PCUniversalParams: Clone + std::fmt::Debug {
    /// Outputs the maximum degree supported by the committer key.
    fn max_degree(&self) -> usize;
}

/// Defines the minimal interface of committer keys for any polynomial
/// commitment scheme.
pub trait PCCommitterKey: Clone + std::fmt::Debug {
    /// Outputs the maximum degree supported by the universal parameters
    /// `Self` was derived from.
    fn max_degree(&self) -> usize;

    /// Outputs the maximum degree supported by the committer key.
    fn supported_degree(&self) -> usize;
}

/// Defines the minimal interface of verifier keys for any polynomial
/// commitment scheme.
pub trait PCVerifierKey: Clone + std::fmt::Debug {
    /// Outputs the maximum degree supported by the universal parameters
    /// `Self` was derived from.
    fn max_degree(&self) -> usize;

    /// Outputs the maximum degree supported by the verifier key.
    fn supported_degree(&self) -> usize;
}

/// Defines the minimal interface of commitments for any polynomial
/// commitment scheme.
pub trait PCCommitment: Clone + algebra::ToBytes {
    /// Outputs a non-hiding commitment to the zero polynomial.
    fn empty() -> Self;

    /// Does this commitment have a degree bound?
    fn has_degree_bound(&self) -> bool;

    /// Size in bytes
    fn size_in_bytes(&self) -> usize;
}

/// Defines the minimal interface of commitment randomness for any polynomial
/// commitment scheme.
pub trait PCRandomness: Clone {
    /// Outputs empty randomness that does not hide the commitment.
    fn empty() -> Self;

    /// Samples randomness for commitments;
    /// `num_queries` specifies the number of queries that the commitment will be opened at.
    fn rand<R: RngCore>(num_queries: usize, rng: &mut R) -> Self;
}

/// Defines the minimal interface of evaluation proofs for any polynomial
/// commitment scheme.
pub trait PCProof: Clone + algebra::ToBytes {
    /// Size in bytes
    fn size_in_bytes(&self) -> usize;
}

/// A polynomial along with information about its degree bound (if any), and the
/// maximum number of queries that will be made to it. This latter number determines
/// the amount of protection that will be provided to a commitment for this polynomial.
#[derive(Debug, Clone)]
pub struct LabeledPolynomial<'a, F: Field> {
    label: String,
    polynomial: Cow<'a, Polynomial<F>>,
    degree_bound: Option<usize>,
    hiding_bound: Option<usize>,
}

impl<'a, F: Field> std::ops::Deref for LabeledPolynomial<'a, F> {
    type Target = Polynomial<F>;

    fn deref(&self) -> &Self::Target {
        &self.polynomial
    }
}

impl<'a, F: Field> LabeledPolynomial<'a, F> {
    /// Construct a new labeled polynomial by consuming `polynomial`.
    pub fn new_owned(
        label: String,
        polynomial: Polynomial<F>,
        degree_bound: Option<usize>,
        hiding_bound: Option<usize>,
    ) -> Self {
        Self {
            label,
            polynomial: Cow::Owned(polynomial),
            degree_bound,
            hiding_bound,
        }
    }

    /// Construct a new labeled polynomial.
    pub fn new(
        label: String,
        polynomial: &'a Polynomial<F>,
        degree_bound: Option<usize>,
        hiding_bound: Option<usize>,
    ) -> Self {
        Self {
            label,
            polynomial: Cow::Borrowed(polynomial),
            degree_bound,
            hiding_bound,
        }
    }

    /// Return the label for `self`.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Retrieve the polynomial from `self`.
    pub fn polynomial(&self) -> &Polynomial<F> {
        &self.polynomial
    }

    /// Evaluate the polynomial in `self`.
    pub fn evaluate(&self, point: F) -> F {
        self.polynomial.evaluate(point)
    }

    /// Retrieve the degree bound in `self`.
    pub fn degree_bound(&self) -> Option<usize> {
        self.degree_bound
    }

    /// Retrieve whether the polynomial in `self` should be hidden.
    pub fn is_hiding(&self) -> bool {
        self.hiding_bound.is_some()
    }

    /// Retrieve the hiding bound for the polynomial in `self`.
    pub fn hiding_bound(&self) -> Option<usize> {
        self.hiding_bound
    }
}

/// A commitment along with information about its degree bound (if any).
#[derive(Clone)]
pub struct LabeledCommitment<C: PCCommitment> {
    label: String,
    commitment: C,
    degree_bound: Option<usize>,
}

impl<C: PCCommitment> LabeledCommitment<C> {
    /// Instantiate a new polynomial_context.
    pub fn new(label: String, commitment: C, degree_bound: Option<usize>) -> Self {
        Self {
            label,
            commitment,
            degree_bound,
        }
    }

    /// Return the label for `self`.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Retrieve the polynomial from `self`.
    pub fn commitment(&self) -> &C {
        &self.commitment
    }

    /// Retrieve the degree bound in `self`.
    pub fn degree_bound(&self) -> Option<usize> {
        self.degree_bound
    }
}

impl<C: PCCommitment> algebra::ToBytes for LabeledCommitment<C> {
    #[inline]
    fn write<W: std::io::Write>(&self, writer: W) -> std::io::Result<()> {
        self.commitment.write(writer)
    }
}
