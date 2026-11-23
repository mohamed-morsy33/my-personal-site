+++
title = "Fresnel Integrals Solved Without Using Complex Analysis"
description = "Solve the Fresnel integrals of cos(x²) and sin(x²) using differentiation under the integral sign. Both equal √(2π)/2."
posted = 2026-11-13
tags = ["Math", "Advanced Integration"]
status = "solved"
problem = '''
Solve the famous Fresnel integrals:
$$
\begin{gathered}
\int_{-\infty}^{\infty} \cos (x^2) \, dx \\
\int_{-\infty}^{\infty} \sin (x^2) \, dx
\end{gathered}
$$
'''
+++

This can be solved using the Gaussian-like integral $\int_{-\infty}^{\infty} e^{-ix^2} \, dx$ and some special techniques, like taking the derivative under the integral. Let's see:

## Solution

By Euler's formula:

$$
\int_{-\infty}^{\infty} e^{-ix^2} \, dx = \int_{-\infty}^{\infty} \cos (x^2) \, dx - i\int_{-\infty}^{\infty} \sin (x^2) \, dx
$$

Let $I(t)$ be defined as follows:

$$
I(t) = \left( \int_0^t e^{-ix^2} \, dx \right)^2
$$

Since the integrand is even:

$$
\begin{aligned}
I &= \int_{-\infty}^{\infty} e^{-ix^2} \, dx = 2\int_0^{\infty} e^{-ix^2} \, dx = 2\sqrt{I(\infty)}
\end{aligned}
$$

Differentiate $I(t)$ using the Leibniz rule:

$$
\begin{aligned}
I'(t) &= \frac{d}{dt}\left( \int_0^t e^{-ix^2} \, dx\right)^2 \\
&= 2\int_0^t e^{-ix^2} \, dx\left( \int_0^t \frac{\partial}{\partial t}\left(e^{-ix^2} \right) dx + e^{-it^2}\frac{dt}{dt} - e^{-i(0)^2}\frac{d}{dt}(0) \right) \\
&= 2\int_0^t e^{-ix^2} \, dx \cdot e^{-it^2} \\
&= 2\int_0^t e^{-i(x^2+t^2)} \, dx \\
&= 2\int_0^t e^{-it^2\left(1+\left(\frac{x}{t}\right)^2\right)} \, dx
\end{aligned}
$$

Substitute $u = \frac{x}{t}$, so $dx = t \, du$ and the bounds become $0$ to $1$:

$$
\begin{aligned}
I'(t) &= 2t\int_0^1 e^{-it^2(1+u^2)} \, du = \int_0^1 2te^{-it^2(1+u^2)} \, du
\end{aligned}
$$

The integrand is itself a derivative with respect to $t$:

$$
\begin{aligned}
2te^{-it^2(1+u^2)} &= \frac{\partial}{\partial t} \left[ \frac{e^{-it^2(1+u^2)}}{-i(1+u^2)} \right] = \frac{\partial}{\partial t} \left[ \frac{ie^{-it^2(1+u^2)}}{1+u^2} \right] \\
\implies I'(t) &= \int_0^1 \frac{\partial}{\partial t}\left( \frac{ie^{-it^2(1+u^2)}}{1+u^2} \right) du = \frac{d}{dt} \int_0^1 \frac{ie^{-it^2(1+u^2)}}{1+u^2} \, du \\
\implies I(t) &= \int_0^1 \frac{ie^{-it^2(1+u^2)}}{1+u^2} \, du + C
\end{aligned}
$$

Find $C$ using $I(0) = 0$:

$$
\begin{aligned}
I(0) = \left( \int_0^0 e^{-ix^2} \, dx \right)^2 = 0 &= \int_0^1 \frac{ie^{-i(0^2)(1+u^2)}}{1+u^2} \, du + C \\
0 &= \int_0^1 \frac{i}{1+u^2} \, du + C \\
C &= -i\left[ \arctan u \right]_0^1 = -\frac{i\pi}{4}
\end{aligned}
$$

As $t \to \infty$ the remaining integral vanishes, so:

$$
\begin{aligned}
I = 2\sqrt{I(\infty)} &= 2 \sqrt{\int_0^1 \frac{ie^{-i \lim_{t\rightarrow\infty}t^2(1+u^2)}}{1+u^2} \, du - \frac{i\pi}{4}} \\
&= 2\sqrt{-\frac{i\pi}{4}} = 2\cdot\frac{\sqrt{\pi}}{2}\sqrt{-i} = \sqrt{\pi}\left( \frac{\sqrt{2}}{2} - \frac{i\sqrt{2}}{2} \right) \\
&= \frac{\sqrt{2\pi}}{2} - i\frac{\sqrt{2\pi}}{2} \\
&= \int_{-\infty}^\infty \cos(x^2) \, dx - i\int_{-\infty}^\infty \sin(x^2) \, dx
\end{aligned}
$$

Matching real and imaginary parts:

$$
\therefore \boxed{ \int_{-\infty}^\infty \cos(x^2) \, dx = \int_{-\infty}^\infty \sin(x^2) \, dx = \frac{\sqrt{2\pi}}{2} }
$$
