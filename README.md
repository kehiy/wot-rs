# Rust Web of Trust

This Rust crate provides basic types and algorithms to represent a [Web of Trust (WoT)](https://en.wikipedia.org/wiki/Web_of_trust) and use different pre-defined or custom algorithms to calculate a nodes WoT rank.

# WoT use cases

You can use WoT with different algorithms in different places and here are some examples:

1. Search engines: used to determine importance/credit of a page based on other pages pointing to it.
2. Academic papers: used to determine credit based on citations.
3. Social media: used to determine how much a user is trustworthy, used in protocols like [Nostr](https://en.wikipedia.org/wiki/Nostr).
5. PKI alternative: used as an alternative to PKI to verify PGP (or any other cryptographic) keys.
6. Any other system that can be modeled as a graph and you need to calculate credit, trustworthiness or importance of a node in it.

# Contribution

Any kind of contribution is welcomed.

# Resources

1. [The PageRank Citation Ranking:
Bringing Order to the Web](http://ilpubs.stanford.edu:8090/422/1/1999-66.pdf)
2. [Fast Incremental and Personalized PageRank](https://www.vldb.org/pvldb/vol4/p173-bahmani.pdf)
3. [A Framework of Algorithms: Computing the Bias and
Prestige of Nodes in Trust Networks](https://arxiv.org/pdf/1207.5661v1)
4. [The EigenTrust Algorithm for Reputation Management in
P2P Networks](https://nlp.stanford.edu/pubs/eigentrust.pdf)
5. [Combating Web Spam with TrustRank](https://www.vldb.org/conf/2004/RS15P3.PDF)

# License

This software is published under [MIT License](./LICENSE).
