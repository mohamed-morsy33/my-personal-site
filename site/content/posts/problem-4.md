+++
title = "Point-Slope Form via Polynomial Interpolation"
posted = 2026-10-31
tags = ["Math", "Numerical Methods", "Calculus", "Linear Algebra"]
status = "solved"
problem = '''
Derive the point-slope formula using the Lagrange Interpolation Theorem.
'''
+++

Lagrange Interpolation Theorem is something that you'll often come across if you were to look into polynomial interpolation via numerical methods.

While we now have access to fancy software like Mathematica and MatLab, polynomial interpolation can still interpreted and accomplished *without* a calculator.

The point-slope formula initially seems like it has nothing to do with polynomial interpolation, but I will now show you how that is most definitely NOT the case:

## Solution

Given $N + 1$ points $x_j\in \mathbb{R}$, $0\le j \le N$ and sample values $y_j=f(x_j)$ of a function at these points:

The polynomial interpolation problems consists in finding a polynomial $p_N(x)$ of degree $N$ which reproduces:

$$
y_j=p_N(x_j)\text{ for } j=0,...N
$$

This means $p_N$ should pass through $(x_j,y_j) \text{  }\forall j=0...N$

$$
p_N(x)=\sum_{n=0}^N a_n x^n \text{ for coefficients } a_0...a_N 
$$

Let $\textbf{V}$ be the Vandermonde matrix:

$$
\textbf{V}_{jn} = x_{j}^n = 
\begin{bmatrix}
1 & x_1 & x_1^2 & \dots & x_1^{N-1} \\
1 & x_2 & x_2^2 & \dots & x_2^{N-1} \\
\vdots & \vdots & \vdots & \ddots & \vdots \\
1 & x_N & x_N^2 & \dots & x_N^{N}
\end{bmatrix}
$$

$$
\textbf{V}a=y \iff \sum_{n=0}^N \textbf{V}_{jn}a_n = y_j
$$


For our approximation, we need it to be reasonable at all locations, not just on average. This is due to the fact that it should have a $(L^\infty)$ uniformly bounded error, meaning $(L^\infty)\le \epsilon$ on $[x_0,x_N]$, meaning no large errors are obfuscated.

Call $p_N$ the space of real-valued degree-$N$ polynomials:

$$
p_N=\{ \sum_{n=0}^N a_nx^n : a_n \in \mathbb{R} \}
$$

Let $\{ x_j, j=0...N \}$ be a collection of disjoint numbers. $\forall k=0,...N$ $\exists $ a unique degree-$N$ polynomial $L_k(x)$ S.T.:

$$
L_k(x_j)=\delta_{jk}=
\begin{cases} 
1, & \text{if } j = k \\ 
0, & \text{otherwise} 
\end{cases}
$$

A proof of this goes beyond the scope of this a little, but just now that for a fixed k, $L_k$ has root at $x_j$ for $j\ne k$, resulting in $L_k$ being of the form:

$$
L_k(x)=C\prod_{j\ne k} (x-x_j)
$$

So at $x=x_k$:

$$
L_k(x)= \frac{\prod_{j\ne k}(x-x_j)}{\prod_{j\ne k}(x_k-x_j)}
$$

Lagrange Interpolation Theorem tells us that for a colection of disjoint real numbers $x_j$ and a collective of real numbers:

$$
p_N(x)=\sum_{n=0}^N y_kL_k(x)
$$

So for a linear interpolation through $(x_1,y_1)$ and $(x_2, y_2)$:

$$
\begin{align*}
L_1(x)=\frac{x-x_2}{x_1-x_2}, \\
L_2(x) = \frac{x-x_1}{x_2-x_1} \\
p_1(x) = y_1L_1(x) + y_2L_2(x) \\
\end{align*}
$$

Through some algebraic maniuplation, we get point-slope form:

$$
\boxed{p_1(x) = y_1 + \frac{(y_2-y_1)}{(x_2-x_1)}(x-x_1)}
$$
