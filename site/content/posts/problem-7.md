+++
title = "Lebesgue Integrals"
posted = 2026-11-12
tags = ["Calculus", "Measure Theory"]
status = "solved"
problem = '''
Evaluate $\int_0^1 D(x) \, dx$ where $D(x)$ is the Dirichlet function.
'''
+++

The Dirichlet function $D(x)$ is defined as follows:

$$
D(x) = \begin{cases} 
  1, &  x \in \mathbb{Q} \\
  0,  & x \notin \mathbb{Q} 
\end{cases}
$$

This isn't Riemann integrable, so in order to solve this we're going to need to integrate by partitioning the y-axis as opposed to x. This is known as **lebesgue integration**, and to get this to work we first need to prove this function isn't Riemann integrable:

$$
\int_0^{\overline{1}} D(x) \, dx \ne \int_{\underline{0}}^1 D(x) \, dx
$$

This notation refers to the upper and lower integrals, which equal $1$ and $0$ respectively. In terms of what "lower" and "upper" mean, the upper integral uses the infimum of the upper sum and the lower integral uses the supremum of the lower sum, denoted by the little bars in the integral bounds.

The infimum of the upper sums (the upper Riemann integral) is the **smallest** possible value you can get by taking the upper Riemann sum over all possible partitions of an interval. The supremum of the lower sums (the lower Riemann integral) is the **largest** possible value you can get by taking the lower Riemann sum over all possible partitions.

So let's see the short solution that Lebesgue gives us:

## Solution

By using lebesgue integration, let $\mu(E)$ be the lebesgue measure ("size" for now) of some $E$. Then:

$$
\int_0^1 D(x) \, dx = 1 * \mu(\mathbb{Q}\cap{[0, 1]}) + 0 * (\mathbb{Q}^C\cap{[0,1]}) = 1\cdot 0 + 0\cdot 1 = \boxed{0}
$$

<figure class="diagram">
<svg viewBox="0 0 560 250" role="img" aria-label="The Dirichlet function on 0 to 1. It is 1 on the rationals, a set of measure 0, and 0 on the irrationals, a set of measure 1.">
<line class="ax" x1="60" y1="210" x2="500" y2="210"/>
<line class="ax" x1="60" y1="20" x2="60" y2="210"/>
<line class="ink" x1="60" y1="30" x2="500" y2="30" stroke-dasharray="0.1 9" stroke-width="3"/>
<line class="hi" x1="60" y1="210" x2="500" y2="210" stroke-width="4"/>
<text x="50" y="35" text-anchor="end">1</text>
<text x="50" y="215" text-anchor="end">0</text>
<text x="500" y="18" text-anchor="end">x ∈ ℚ: measure 0</text>
<text class="hi" x="500" y="198" text-anchor="end">x ∉ ℚ: measure 1</text>
<text class="dim" x="60" y="232" text-anchor="middle">0</text>
<text class="dim" x="500" y="232" text-anchor="middle">1</text>
</svg>
</figure>

## Quick Note

A measure $\mu(E)$ is a function that assigns a real number that's greater than $0$ or $\infty$ to certain subsets of a set $E$. It's a way of generalizing length, area and vojlume at the same time.

$$
\mu(E=\cup E_n) = \sum_n (E_n), \text{ Where $E_n$ are pairwise disjoint sets}
$$