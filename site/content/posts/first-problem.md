+++
title = "Euler's Formula & Euler's Identity"
posted = 2026-10-27
tags = ["Math", "Calculus"]
status = "solved"
problem = '''
Prove Euler's Formula and Identity:
$e^{ix}=\cos(x)+i\sin(x)$
$e^{i\pi}=-1$
'''
+++

I remember when I first encountered this problem when I was still studying series and sequences for the first time two years ago. Before I learned about integrating power series, I first encountered this question, and I wrote down the solution, not thinking much of it. 

Now that I've learned more about how this works, little did I know back then, the math I was learning was *far* more interesting than I could've ever imagined. 

## Solution


It is known that given the elementary power series:

$$
e^x=\sum_{n=0}^\infty \frac{x^n}{n!}
$$

Substituting $ix$ for $x$, we obtain:

$$
e^{ix} = \sum_{n=0}^\infty \frac{(ix)^n}{n!}
$$

Writing out the terms of this series,

$$
\begin{aligned}
e^{ix} &= 1 + ix + \frac{(ix)^2}{2!} + \frac{(ix)^3}{3!} + \frac{(ix)^4}{4!} + \frac{(ix)^5}{5!} + \cdots \\
&= 1+ix-\frac{x^2}{2!} - \frac{ix^3}{3!} + \frac{x^4}{4!} + \frac{ix^5}{5!} - \frac{x^6}{6!} - \frac{ix^7}{7!} + \cdots \\[8pt]
&= \underbrace{\left(1-\frac{x^2}{2!}+\frac{x^4}{4!}-\frac{x^6}{6!}+\cdots\right)}_{\cos x}
+i\underbrace{\left(x-\frac{x^3}{3!}+\frac{x^5}{5!}-\frac{x^7}{7!}+\cdots\right)}_{\sin x}
\end{aligned}
$$

Notice that the even powers of $x$ form the Taylor series for $\cos x$, while the odd powers form the Taylor series form $\sin x$:

$$
\cos x=\sum_{n=0}^{\infty}\frac{(-1)^n x^{2n}}{(2n)!},
\qquad
\sin x=\sum_{n=0}^{\infty}\frac{(-1)^n x^{2n+1}}{(2n+1)!}.
$$

Therefore, we arrive at Euler's formula:

$$ e^{ix}=\cos x+i\sin x$$

Additionally, if we plug in $\pi$, we get Euler's Identity:

$$
e^{i\pi}=\cos(\pi)+i\sin(\pi) \\
e^{i\pi}=-1 \\ 
\boxed{e^{i\pi}+1=0}
$$