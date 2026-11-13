+++
title = "Element Method"
posted = 2026-11-11
tags = ["Math", "Proofs", "Logic"]
status = "solved"
problem = '''
Prove set $A \subseteq B \\ \forall x \in A, x \in B.$
'''
+++

This notation denotes a **proper subset**, meaning a subset that isn't just the same set as the original. This literally means it's **properly** a **sub**set, not being allowed to be potentially equal.

To go about proving this, we use the **element method**:


## Solution

$$
\begin{align}
A = {n \in \mathbb{Z} | n = 4p, p \in \mathbb{Z}} \\ 
B = {m \in \mathbb{Z} | n = 2q, q \in \mathbb{Z}} \\
\end{align}
$$

Let $x\in A$, $x=4p$ for some $p\in\mathbb{Z}$:
$$
= 2(2p)
$$

Let $q=2p\in\mathbb{Z}$

So $x\in B$

$$
\therefore \boxed{A \subseteq B}
$$
