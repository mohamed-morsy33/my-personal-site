+++
title = "Infinite Power Tower"
posted = 2026-11-16
tags = ["Math", "Calculus", "Derivatives"]
status = "solved"
problem = '''
Find $\frac{dy}{dx}$ for the infinite power tower $y = x^{x^{x^{x^{...}}}}$.
'''
+++

The key to this problem is similar to problems in algebra with this recursive pattern for x. Now, we just use calculus after via implicit differentiation. Let me explain:


## Solution

Using the idea of self-similarity, we can say that $y=x^y$. Via logarithmic differentiation:

$$
\begin{aligned}
\ln y = y\ln x \\
\frac{1}y \frac{dy}{dx} = \frac{dy}{dx} \ln x + \frac{y}x \\
\end{aligned}
$$

Now we can group like terms:

$$
\begin{aligned}
\frac{dy}{dx}\left( \frac{1}y - \ln x \right) = \frac{y}x \\ 
\frac{dy}{dx} = \frac{y^2}{x(1-y\ln x)} \\ 
\end{aligned}
$$

Note that $\ln y = y \ln x$:

$$
\boxed{\frac{dy}{dx} = \frac{y^2}{x(1-\ln y)}}
$$

Note that Euler proved this infinite power tower converges $\iff e^{-e} \le x \le e^{1/e}$, so that's where our valid derivatives lie.
