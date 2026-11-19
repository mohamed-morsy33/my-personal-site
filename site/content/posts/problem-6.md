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
A = \{n \in \mathbb{Z} | n = 4p, p \in \mathbb{Z}\} \\ 
B = \{m \in \mathbb{Z} | n = 2q, q \in \mathbb{Z}\} \\
\end{align}
$$

<figure class="diagram">
<svg viewBox="0 0 520 280" role="img" aria-label="Venn diagram. Inside the integers, B is the even numbers and A, the multiples of 4, sits entirely inside B.">
<rect class="ax" x="10" y="10" width="500" height="260" rx="12"/>
<text x="26" y="38">ℤ</text>
<ellipse class="ink" cx="270" cy="145" rx="200" ry="105"/>
<ellipse class="hi" cx="230" cy="150" rx="105" ry="62" fill="var(--yellow-dim)"/>
<text x="320" y="80">B: evens</text>
<text class="hi" x="230" y="122" text-anchor="middle">A: multiples of 4</text>
<text class="hi" x="230" y="158" text-anchor="middle" word-spacing="14">−4 0 4 8</text>
<text class="hi" x="230" y="184" text-anchor="middle" word-spacing="14">12 16</text>
<text x="380" y="150" word-spacing="14">2 6</text>
<text x="375" y="180" word-spacing="10">10 −2</text>
<text class="dim" x="40" y="250" word-spacing="12">1 3 5 7 …</text>
</svg>
</figure>

Let $x\in A$, $x=4p$ for some $p\in\mathbb{Z}$:
$$
= 2(2p)
$$

Let $q=2p\in\mathbb{Z}$

So $x\in B$

$$
\therefore \boxed{A \subseteq B}
$$
