+++
title = "Quadratic Approximation"
posted = 2026-10-28
tags = ["Math"]
status = "solved"
problem = '''
Approximate $f(x,y)=e^{x/2}\sin y$ near the point $(0,\tfrac{\pi}{2})$, where $f(0,\tfrac{\pi}{2}) = 1$.
'''
+++

When I was first studying multivariable calculus, I remember when I came across quadratic approximations, and how I thought they wouldn't be all too different from the linear approximations I learned in single variable calculus:

$$
L(x,y)=a+bx+cy
$$

However, I soon came to learn that this wasn't the case.

In three dimensions:

$$
\begin{aligned}
L(x,y) = {} & f(x_0,y_0) \\
& + f_x(x_0,y_0)(x-x_0) \\
& + f_y(x_0,y_0)(y-y_0)
\end{aligned}
$$

The quadratic approximation consists of a constant term, a linear term, and a quadratic term, just like how a linear approximation consists of a constant term and a linear term:

$$
\begin{aligned}
Q(x,y) = {} & f(x_0,y_0) \\
& + f_x(x_0,y_0)(x-x_0) \\
& + f_y(x_0,y_0)(y-y_0) \\
& + \tfrac{1}{2}f_{xx}(x_0,y_0)(x-x_0)^2 \\
& + f_{xy}(x_0,y_0)(x-x_0)(y-y_0) \\
& + \tfrac{1}{2}f_{yy}(x_0,y_0)(y-y_0)^2
\end{aligned}
$$

## Solution

Given our $Q(x,y)$, the derivatives at $(0,\tfrac{\pi}{2})$ are:

$$
\begin{aligned}
f_x(0,\tfrac{\pi}{2}) &= \tfrac{1}{2} \\
f_y(0,\tfrac{\pi}{2}) &= 0 \\
f_{xx}(0,\tfrac{\pi}{2}) &= \tfrac{1}{4} \\
f_{yy}(0,\tfrac{\pi}{2}) &= -1 \\
f_{xy}(0,\tfrac{\pi}{2}) = f_{yx}(0,\tfrac{\pi}{2}) &= 0
\end{aligned}
$$

Substituting into $Q$:

$$
\begin{aligned}
Q(x,y) &= 1 + \tfrac{1}{2}(x-0) + 0\,(y-\tfrac{\pi}{2}) \\
&\quad + \tfrac{1}{8}(x-0)^2 \\
&\quad + 0\,(x-0)(y-\tfrac{\pi}{2}) \\
&\quad - \tfrac{1}{2}(y-\tfrac{\pi}{2})^2
\end{aligned}
$$

$$
\therefore\; Q(x,y) = 1 + \frac{x}{2} + \frac{x^2}{8} - \frac{1}{2}\left(y-\frac{\pi}{2}\right)^2
$$
