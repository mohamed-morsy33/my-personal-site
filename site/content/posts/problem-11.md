+++
title = "Regiomontanus's Problem"
posted = 2026-11-17
tags = ["Trigonometry", "Calculus", "Geometry"]
status = "solved"
problem = '''
A painting hangs on a wall with its bottom edge at height B above eye level and its top edge at height A above eye level ($A > B > 0$). How far back ($x$) from the wall should you stand to maximize you viewing angle $\theta$?
'''
+++

This is more of a geometry problem at first glance, but the word "maximize" made me think about optimization:

## Solution

Let $\alpha$ be the angle from eye level to the top of the painting, and $\beta$ be the angle from eye level to the bottom. Then the viewing angle is $\theta = \alpha - \beta$, where:

<figure class="diagram">
<svg viewBox="0 0 500 300" role="img" aria-label="Eye at distance x from a wall. Lines from the eye to the bottom (height B) and top (height A) of the painting form two right triangles; the angle between them is theta.">
<polygon class="fill" points="60,250 400,50 400,170"/>
<line class="ax" x1="400" y1="20" x2="400" y2="285" stroke-width="3"/>
<rect class="dot" x="392" y="50" width="8" height="120" rx="2"/>
<line class="guide" x1="60" y1="250" x2="400" y2="250"/>
<polyline class="ax" points="388,250 388,238 400,238"/>
<line class="ink" x1="60" y1="250" x2="400" y2="50"/>
<line class="ink" x1="60" y1="250" x2="400" y2="170"/>
<path class="ink" d="M110,250 A50,50 0 0 0 103.1,224.65" stroke-width="1.5"/>
<path class="ink" d="M135,250 A75,75 0 0 0 133,232.8" stroke-width="1.5"/>
<path class="hi" d="M167.1,224.8 A110,110 0 0 0 154.8,194.2" stroke-width="3"/>
<text x="114" y="230">α</text>
<text x="146" y="243">β</text>
<text class="hi" x="172" y="205">θ</text>
<circle class="dot-ink" cx="60" cy="250" r="6"/>
<text class="dim" x="60" y="275" text-anchor="middle">eye</text>
<line class="ax" x1="60" y1="292" x2="400" y2="292"/>
<text x="230" y="286" text-anchor="middle">x</text>
<line class="ax" x1="420" y1="250" x2="420" y2="170"/>
<text x="426" y="215">B</text>
<line class="ax" x1="450" y1="250" x2="450" y2="50"/>
<text x="456" y="155">A</text>
<text class="dim" x="384" y="42" text-anchor="end">painting</text>
</svg>
</figure>

$$
\begin{aligned}
\tan \alpha = \frac{A}x, \text{   } \tan \beta = \frac{B}x
\end{aligned}
$$

Using this formula for tangent where: $\tan \theta = \tan(\alpha - \beta)$:

$$
\frac{\tan\alpha - \tan\beta}{1+\tan\alpha\tan\beta}=\frac{\frac{A}x - \frac{B}x}{1+\frac{AB}{x^2}} = \frac{x(A-B)}{x^2+AB}
$$

We know that $\theta$ is between $0$ and $\frac{\pi}2$ since the painting is on a wall in front of us, so maximizing $\theta$ is the same as maximizing a function $f(x)$ = our result above for $x>0$, so we differentiate it:

$$
\begin{aligned}
g'(x) = (A-B)\cdot \frac{(x^2+AB)(1)-x(2x)}{(x^2+AB)^2}=\frac{(A-B)(AB-x^2)}{(x^2+AB)^2} \\
g'(x) = 0 \rightarrow AB - x^2 = 0 \text{,    Since we know that }A-B>0
\end{aligned}
$$

$$
\therefore \boxed{x = \sqrt{AB}}
$$

We can confirm this is a maximum by checking how $g'(x) > 0$ for $x <\sqrt{AB}$ and $g'(x) < 0$ for $x > \sqrt{AB}$.