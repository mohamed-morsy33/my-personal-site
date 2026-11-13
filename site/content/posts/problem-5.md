+++
title = "Calculating Work for a Vector Field"
posted = 2026-11-10
tags = ["Math", "Vector Calculus", "Physics"]
status = "solved"
problem = '''
Calculate the work done on a path $C$ for $f(x,y)=y\hat{\imath}-x\hat{\jmath}$, where is $C$ is parameterized by $x(t)=\cos (t)$, $y(t)=\sin (t)$ for $0 \le t \le 2\pi$
'''
+++

When learning about line integrals in vector fields, usually the idea of work is introduced (or reviewed, if someone has already been exposed to physics before) as the magnitude of force in the direction of displacement dotted with the distance displaced.

When working with vectors, this is often denoted as $W=\hat{d}\cdot\hat{F}$. Remember, the dot product produces a scalar value, and work is a scalar value, often in ft-lbs or joules (J).

## Solution

Let $W$ denote the work done in this field on this path $C$. 

$$
W = \int_C \hat{f} \cdot d\hat{r} \\ 
\hat{r}(t) = \cos t \hat{\imath} + \sin t \hat{\jmath}, \, 0\le t \le 2\pi
$$

$$
\begin{align}
\frac{d\hat{r}}{dt} = r'(t) = -\sin t \hat{\imath} + \cos t \hat{\jmath} \\ 
d\hat{r} = -\sin t dt \hat{\imath} + \cos t dt \hat{\jmath} \\
\hat{f}(t) = \sin t \hat{\imath} - \cos t \hat{\jmath} \\
\end{align}
$$

$$
\begin{align}
\therefore W = \int_0^{2\pi} (-\sin^2t - \cos^2t) \, dt \\
= -\int_0^{2\pi}1 \, dt \\
= -2\pi
\end{align}
$$

$$
\boxed{-2\pi \text{ J (assuming SI units)}}
$$