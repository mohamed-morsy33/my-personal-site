+++
title = "Fresnel Integrals Solved Without Using Complex Analysis"
posted = 2026-11-13
tags = ["Math", "Advanced Integration"]
status = "solved"
problem = '''
Solve the famous Fresnel integrals:
$$
\begin{aligned}
\int_{-\infty}^{\infty} \cos (x^2) \\
 
\int_{-\infty}^{\infty} \sin (x^2) \\
\end{aligned}
$$
'''
+++


This can be solved using the gaussian integral: $\int_{-\infty}^{\infty} e^{-ix^2} \, dx$ using some special techniques, like taking the derivative under the integral. Let's see:

## Solution

By Euler's Identity:

$$
\int_{-\infty}^{\infty} e^{-ix^2} \, dx = \int_{-\infty}^{\infty} \cos (x^2) \, dx  -i\int_{-\infty}^{\infty} \sin (x^2) \, dx
$$

Let $I(t)$ be defined as follows:

$$
I(t) = \left( \int_0^t e^{-ix^2} \, dx \right)^2
$$

$$
\begin{aligned}
\therefore 2\int_0^{\infty} e^{-ix^2} \, dx = I \\ 
I = 2\sqrt{I(t=\infty)} \\ 
I'(t) = \frac{d}{dt}\left( \int_0^t e^{-ix^2} \, dx\right)^2 \\
= 2\int_0^t e^{-ix^2} \, dx\left( \int_0^t \frac{\partial}{\partial t}\left(e^{-ix^2} \right) \, dx + e^{-it^2}\frac{dt}{dt} - e^{-i(0)^2}\frac{d}{dt}(0)    \right) \\
= 2\int_0^t e^{-ix^2} \, dx\left(e^{-it^2}\right) \\ 
= 2\int_0^t e^{-i(x^2+t^2)} \, dx \\ 
= 2\int_0^t e^{-it^2}(1+(\frac{x}t)^2) \, dx \\ 
u = \frac{x}t, du=\frac{dx}t \\ 

I'(t) = 2t\int_0^t e^{-it^2(1+y^2)} \, dy \\ 
= 2\int_0^t e^{-it^2(1+u^2)}(t \, du) \\

2te^{-it^2(1+u^2)} = \frac{\partial}{\partial t} \left[ \frac{e^{-it^2(1+u^2)}}{-i(1+u^2)} \right] = \frac{\partial}{\partial t} \left[ \frac{ie^{-it^2(1+u^2)}}{1+u^2} \right] \iff I'(t) = \int_0^t \frac{\partial}{\partial t}\left( \frac{ie^{-it^2(1+u^2)}}{1+u^2} \right) \, du \\ 
= \frac{d}{dt} \int_0^t \frac{ie^{-it^2(1+u^2)}}{1+u^2} \, du \\

\int I'(t) \, dt = I(t) \int \frac{d}{dt} \left( \int_0^1 \frac{ie^{-it^2(1+u^2)}}{1+u^2} \, du\right) \, dt \\ 
= \int_0^1 \frac{ie^{-it^2(1+u^2)}}{1+u^2} \, du + C \\ 

I(t=0) = \left( \int_0^0 e^{-ix^2} \, dx \right) = \int_0^1 \frac{ie^{-i(0^2)(1+u^2)}}{1+u^2} \, du + C \\ 
0 = \int_0^1 \frac{i}{1+u^2} \, du + C \\ 
C = -i\left[ \arctan u \right]_0^1 = \frac{-i\pi}{4} \\ 

I=2\sqrt{I(t=\infty)} = 2 \sqrt{\int_0^1 \frac{ie^{-i \lim_{t\rightarrow\infty}t^2(1+u^2)}}{1+u^2} \, du - \frac{i\pi}{4}} \\
= 2\sqrt{\frac{-i\pi}{4}} = 2\frac{\sqrt{\pi}}{2}\sqrt{-i} = \sqrt{\pi}\left( \frac{\sqrt{2}}2 - \frac{i\sqrt{2}}2 \right) \\
= \frac{\sqrt{2\pi}}2 - i\frac{\sqrt{2}\pi}2 \\
= \int_{-\infty}^\infty \cos(x^2) \, dx - i\int_{-\infty}^\infty \sin(x^2) \, dx \\ 

\therefore \boxed{ \int_{-\infty}^\infty \cos(x^2) \, dx = \frac{\sqrt{2}\pi}{2} = \int_{-\infty}^\infty \sin(x^2) \, dx} 

\end{aligned} 
$$