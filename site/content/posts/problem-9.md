+++
title = "Why 45° Is the Best Launch Angle (and Why Air Ruins It)"
description = "Derive projectile range, flight time and height, prove 45° maximizes range, then simulate how quadratic air drag lowers the best launch angle."
posted = 2026-11-15
tags = ["Physics", "Mechanics", "Kinematics"]
status = "solved"
problem = '''
A projectile is launched from level ground with speed $v_0$ at angle $\theta$ above the horizontal. Ignoring air resistance:

1. Find the range $R(\theta)$, the flight time $T$ and the maximum height $H$.
2. Show that the range is largest at $\theta = 45^\circ$.
3. Show that the angles $\theta$ and $90^\circ - \theta$ give the same range.

Bonus: with quadratic drag, does the best angle stay at $45^\circ$?
'''
+++

The idea is to split the motion into two independent 1D problems: constant velocity horizontally and constant acceleration $-g$ vertically.

## Solution

The initial velocity components are $v_0\cos\theta$ and $v_0\sin\theta$, so

$$
x(t) = v_0 \cos\theta \, t, \qquad y(t) = v_0 \sin\theta \, t - \tfrac{1}{2} g t^2
$$

**Flight time.** The projectile lands when $y(T) = 0$ with $T \neq 0$:

$$
T\left(v_0\sin\theta - \tfrac{1}{2} g T\right) = 0 \implies T = \frac{2 v_0 \sin\theta}{g}
$$

**Range.** Substitute $T$ into $x(t)$ and use $2\sin\theta\cos\theta = \sin 2\theta$:

$$
R = v_0\cos\theta \cdot \frac{2 v_0 \sin\theta}{g} = \frac{v_0^2 \sin 2\theta}{g}
$$

**Maximum height.** The vertical velocity is zero at the top, so $v_0^2\sin^2\theta = 2gH$:

$$
H = \frac{v_0^2 \sin^2\theta}{2g}
$$

**Best angle.** $\sin 2\theta \le 1$, with equality when $2\theta = 90^\circ$. So

$$
\boxed{\theta^* = 45^\circ, \qquad R_{\max} = \frac{v_0^2}{g}}
$$

**Complementary angles.** Replace $\theta$ with $90^\circ - \theta$:

$$
\sin\big(2(90^\circ - \theta)\big) = \sin(180^\circ - 2\theta) = \sin 2\theta
$$

So $R(\theta) = R(90^\circ - \theta)$. A $30^\circ$ shot and a $60^\circ$ shot land at the same spot. The $60^\circ$ shot goes $\tan^2 60^\circ / \tan^2 30^\circ = 9$ times higher and stays in the air $\sqrt{3}$ times longer.

With $v_0 = 30$ m/s and $g = 9.81$ m/s², the maximum range is $900 / 9.81 \approx 91.7$ m.

## Bonus: adding air resistance

Now add quadratic drag, $\vec a = -g\hat y - k\,|\vec v|\,\vec v$. The two directions are now coupled through $|\vec v|$, so there is no closed-form answer. The sim below integrates it numerically. The dashed curve is the vacuum answer from above, and the yellow curve includes drag.

```sim
name = "projectile"
speed = 30
angle = 45
drag = 0.02
```

Try a few things:

- With $k = 0$, the $30^\circ$ and $60^\circ$ shots both land at $79.5$ m.
- With drag on, that symmetry breaks. At $k = 0.02$, the $30^\circ$ shot reaches $40.6$ m and the $60^\circ$ shot only $34.4$ m. The high shot spends longer in the air, so drag has more time to slow it.
- The best angle drops below $45^\circ$ as drag grows (for $v_0 = 30$ m/s):

| drag $k$ (1/m) | best angle | max range |
| --- | --- | --- |
| 0 | 45° | 91.7 m |
| 0.005 | 43° | 68.8 m |
| 0.02 | 39° | 42.2 m |
| 0.05 | 36° | 25.7 m |
| 0.1 | 33° | 16.4 m |

So $45^\circ$ is only special in a vacuum. Real projectiles get the most range from a flatter shot.
