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

<figure class="diagram">
<svg viewBox="0 0 420 310" role="img" aria-label="Unit circle in the complex plane. The point e to the ix sits at angle x, with real part cos x and imaginary part sin x. Going half way round, angle pi, lands on minus one.">
<line class="ax" x1="60" y1="165" x2="360" y2="165"/>
<line class="ax" x1="210" y1="35" x2="210" y2="295"/>
<text class="dim" x="364" y="169">Re</text>
<text class="dim" x="216" y="40">Im</text>
<circle class="ink" cx="210" cy="165" r="110" stroke-width="1.5"/>
<path class="hi" d="M320,165 A110,110 0 0 0 100,165" opacity="0.35" stroke-width="6"/>
<line class="hi" x1="210" y1="165" x2="273.1" y2="74.9"/>
<line class="guide" x1="273.1" y1="74.9" x2="273.1" y2="165"/>
<line class="hi" x1="210" y1="165" x2="273.1" y2="165"/>
<path class="ink" d="M240,165 A30,30 0 0 0 227.2,140.4" stroke-width="1.5"/>
<text x="246" y="155">x</text>
<text x="241.5" y="187" text-anchor="middle">cos x</text>
<text x="279.1" y="153">sin x</text>
<circle class="dot" cx="273.1" cy="74.9" r="5"/>
<text class="hi" x="283.1" y="68.9">e<tspan dy="-7" font-size="11">ix</tspan></text>
<circle class="dot-ink" cx="320" cy="165" r="4"/>
<text class="dim" x="326" y="183">1</text>
<circle class="dot" cx="100" cy="165" r="5"/>
<text class="hi" x="94" y="187" text-anchor="end">e<tspan dy="-7" font-size="11">iπ</tspan> = −1</text>
<text class="hi" x="150" y="63" text-anchor="end">x = π</text>
</svg>
</figure>

Additionally, if we plug in $\pi$, we get Euler's Identity:

$$
e^{i\pi}=\cos(\pi)+i\sin(\pi) \\
e^{i\pi}=-1 \\ 
\boxed{e^{i\pi}+1=0}
$$