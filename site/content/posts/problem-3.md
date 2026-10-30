+++
title = "Proving divisiblity is transitive"
posted = 2026-10-29
tags = ["Math", "Discrete Math", "Logic", "Proofs"]
status = "solved"
problem = '''
Prove that divisibility is transitive.
'''
+++

When learning discrete mathematics and basic proofing, one of the basic properties of arithmetic comes up: divisibility.

Dividing seems straightforward, and it is, but if you asked someone to prove if it's "transitive", they'll probably have idea what you're talking about.

Transitive, relating to the transitive property, is the idea that if one element relates to a second in some way, and a second relates to a third, then the first must also related to a third:

$$
\begin{aligned}
a = b \\ 
b = c \\ 
\therefore a = c
\end{aligned}
$$

Let's work through this:

## Solution

$12$ is divisible by $3$.

$$
\frac{12}3 = 4 \in \mathbb{Z}
$$

$12$ is **NOT** divisible by $5$.

$$
\frac{12}5 \notin \mathbb{Z}
$$

$$
\begin{aligned}
\therefore 12 = 3\cdot 4 \, | \, \exists 4 \in \mathbb{Z} \\
12 \ne 5 \cdot p \, | \, \exists p \in \mathbb{Z}
\end{aligned}
$$

Now let's define divisibility:

$$
\textbf{Definition: } \text{For } n \text{ and } d \text{ integers, } d \ne 0, d | n \iff \exists k \in \mathbb{Z} \text{ S.T. } n =dk
$$

This means that "$d$ divides $n$", AKA $n$ is divisible by $d$, meaning $\frac{n}d\in \mathbb{Z}$.

$$
\textbf{Prove: }\text{If } b | a \text{, and } c | b \text{, then } c | a \text{.}
$$

This means that if $a$ is divisible by $b$ and $b$ is divisible by $c$, then $a$ is divisible by $c$, which would prove that divisibility as a property is in and of itself **transitive**.

$$
\text{Let } b | a \text{ and } c | b \\ 
\exists s,t \in \mathbb{Z} \text{ S.T. } a=sb, b=tc \\ 
\text{Then } a = sb = s(tc) = c(st), st \in \mathbb{Z}
$$

We know that if $a = cu$, where $u\in\mathbb{Z}$, then $c|a$. Letting $u=st$:

$$
\boxed{\therefore c | a}
$$


And you know what that implies? Divisibility is indeed transitive!