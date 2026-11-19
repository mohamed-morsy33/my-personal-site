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

<figure class="diagram">
<svg viewBox="0 0 360 340" role="img" aria-label="The field y i minus x j circulates clockwise. The path C runs counterclockwise around the unit circle, so it always moves against the field.">
<line class="ax" x1="30" y1="170" x2="330" y2="170"/>
<line class="ax" x1="180" y1="20" x2="180" y2="320"/>
<circle class="hi" cx="180" cy="170" r="85.7"/>
<line class="ink" x1="61.2" y1="308.3" x2="45.9" y2="293.1"/><polygon class="dot-ink" points="41.7,288.8 48,290.9 43.8,295.2"/>
<line class="ink" x1="58.3" y1="266.1" x2="47.9" y2="250.4"/><polygon class="dot-ink" points="44.5,245.4 50.4,248.7 45.4,252"/>
<line class="ink" x1="55.1" y1="223.8" x2="49.7" y2="207.6"/><polygon class="dot-ink" points="47.8,202 52.5,206.7 46.8,208.6"/>
<line class="ink" x1="51.4" y1="181.1" x2="51.4" y2="164.9"/><polygon class="dot-ink" points="51.4,158.9 54.4,164.9 48.4,164.9"/>
<line class="ink" x1="47.8" y1="138" x2="53.2" y2="121.9"/><polygon class="dot-ink" points="55.1,116.2 56,122.9 50.3,121"/>
<line class="ink" x1="44.5" y1="94.6" x2="55" y2="78.9"/><polygon class="dot-ink" points="58.3,73.9 57.5,80.6 52.5,77.3"/>
<line class="ink" x1="41.7" y1="51.2" x2="56.9" y2="35.9"/><polygon class="dot-ink" points="61.2,31.7 59.1,38 54.8,33.8"/>
<line class="ink" x1="104.6" y1="305.5" x2="88.9" y2="295"/><polygon class="dot-ink" points="83.9,291.7 90.6,292.5 87.3,297.5"/>
<line class="ink" x1="101.9" y1="263.3" x2="90.9" y2="252.3"/><polygon class="dot-ink" points="86.7,248.1 93,250.2 88.8,254.5"/>
<line class="ink" x1="98.5" y1="221.4" x2="92.7" y2="209.7"/><polygon class="dot-ink" points="90,204.4 95.4,208.4 90,211.1"/>
<line class="ink" x1="94.3" y1="179" x2="94.3" y2="167"/><polygon class="dot-ink" points="94.3,161 97.3,167 91.3,167"/>
<line class="ink" x1="90" y1="135.6" x2="95.9" y2="124"/><polygon class="dot-ink" points="98.5,118.6 98.5,125.3 93.2,122.7"/>
<line class="ink" x1="86.7" y1="91.9" x2="97.7" y2="80.9"/><polygon class="dot-ink" points="101.9,76.7 99.8,83 95.5,78.8"/>
<line class="ink" x1="83.9" y1="48.3" x2="99.6" y2="37.9"/><polygon class="dot-ink" points="104.6,34.5 101.3,40.4 98,35.4"/>
<line class="ink" x1="148" y1="302.2" x2="131.9" y2="296.8"/><polygon class="dot-ink" points="126.2,294.9 132.9,294 131,299.7"/>
<line class="ink" x1="145.6" y1="260" x2="134" y2="254.1"/><polygon class="dot-ink" points="128.6,251.5 135.3,251.5 132.7,256.8"/>
<line class="ink" x1="142.6" y1="218.3" x2="135.9" y2="211.6"/><polygon class="dot-ink" points="131.7,207.4 138,209.5 133.8,213.7"/>
<line class="ink" x1="137.1" y1="176.9" x2="137.1" y2="169.1"/><polygon class="dot-ink" points="137.1,163.1 140.1,169.1 134.1,169.1"/>
<line class="ink" x1="131.7" y1="132.6" x2="138.4" y2="125.9"/><polygon class="dot-ink" points="142.6,121.7 140.5,128 136.3,123.8"/>
<line class="ink" x1="128.6" y1="88.5" x2="140.3" y2="82.7"/><polygon class="dot-ink" points="145.6,80 141.6,85.4 138.9,80"/>
<line class="ink" x1="126.2" y1="45.1" x2="142.4" y2="39.7"/><polygon class="dot-ink" points="148,37.8 143.3,42.5 141.4,36.8"/>
<line class="ink" x1="191.1" y1="298.6" x2="174.9" y2="298.6"/><polygon class="dot-ink" points="168.9,298.6 174.9,295.6 174.9,301.6"/>
<line class="ink" x1="189" y1="255.7" x2="177" y2="255.7"/><polygon class="dot-ink" points="171,255.7 177,252.7 177,258.7"/>
<line class="ink" x1="186.9" y1="212.9" x2="179.1" y2="212.9"/><polygon class="dot-ink" points="173.1,212.9 179.1,209.9 179.1,215.9"/>
<line class="ink" x1="173.1" y1="127.1" x2="180.9" y2="127.1"/><polygon class="dot-ink" points="186.9,127.1 180.9,130.1 180.9,124.1"/>
<line class="ink" x1="171" y1="84.3" x2="183" y2="84.3"/><polygon class="dot-ink" points="189,84.3 183,87.3 183,81.3"/>
<line class="ink" x1="168.9" y1="41.4" x2="185.1" y2="41.4"/><polygon class="dot-ink" points="191.1,41.4 185.1,44.4 185.1,38.4"/>
<line class="ink" x1="233.8" y1="294.9" x2="217.6" y2="300.3"/><polygon class="dot-ink" points="212,302.2 216.7,297.5 218.6,303.2"/>
<line class="ink" x1="231.4" y1="251.5" x2="219.7" y2="257.3"/><polygon class="dot-ink" points="214.4,260 218.4,254.6 221.1,260"/>
<line class="ink" x1="228.3" y1="207.4" x2="221.6" y2="214.1"/><polygon class="dot-ink" points="217.4,218.3 219.5,212 223.7,216.2"/>
<line class="ink" x1="222.9" y1="163.1" x2="222.9" y2="170.9"/><polygon class="dot-ink" points="222.9,176.9 219.9,170.9 225.9,170.9"/>
<line class="ink" x1="217.4" y1="121.7" x2="224.1" y2="128.4"/><polygon class="dot-ink" points="228.3,132.6 222,130.5 226.2,126.3"/>
<line class="ink" x1="214.4" y1="80" x2="226" y2="85.9"/><polygon class="dot-ink" points="231.4,88.5 224.7,88.5 227.3,83.2"/>
<line class="ink" x1="212" y1="37.8" x2="228.1" y2="43.2"/><polygon class="dot-ink" points="233.8,45.1 227.1,46 229,40.3"/>
<line class="ink" x1="276.1" y1="291.7" x2="260.4" y2="302.1"/><polygon class="dot-ink" points="255.4,305.5 258.7,299.6 262,304.6"/>
<line class="ink" x1="273.3" y1="248.1" x2="262.3" y2="259.1"/><polygon class="dot-ink" points="258.1,263.3 260.2,257 264.5,261.2"/>
<line class="ink" x1="270" y1="204.4" x2="264.1" y2="216"/><polygon class="dot-ink" points="261.5,221.4 261.5,214.7 266.8,217.3"/>
<line class="ink" x1="265.7" y1="161" x2="265.7" y2="173"/><polygon class="dot-ink" points="265.7,179 262.7,173 268.7,173"/>
<line class="ink" x1="261.5" y1="118.6" x2="267.3" y2="130.3"/><polygon class="dot-ink" points="270,135.6 264.6,131.6 270,128.9"/>
<line class="ink" x1="258.1" y1="76.7" x2="269.1" y2="87.7"/><polygon class="dot-ink" points="273.3,91.9 267,89.8 271.2,85.5"/>
<line class="ink" x1="255.4" y1="34.5" x2="271.1" y2="45"/><polygon class="dot-ink" points="276.1,48.3 269.4,47.5 272.7,42.5"/>
<line class="ink" x1="318.3" y1="288.8" x2="303.1" y2="304.1"/><polygon class="dot-ink" points="298.8,308.3 300.9,302 305.2,306.2"/>
<line class="ink" x1="315.5" y1="245.4" x2="305" y2="261.1"/><polygon class="dot-ink" points="301.7,266.1 302.5,259.4 307.5,262.7"/>
<line class="ink" x1="312.2" y1="202" x2="306.8" y2="218.1"/><polygon class="dot-ink" points="304.9,223.8 304,217.1 309.7,219"/>
<line class="ink" x1="308.6" y1="158.9" x2="308.6" y2="175.1"/><polygon class="dot-ink" points="308.6,181.1 305.6,175.1 311.6,175.1"/>
<line class="ink" x1="304.9" y1="116.2" x2="310.3" y2="132.4"/><polygon class="dot-ink" points="312.2,138 307.5,133.3 313.2,131.4"/>
<line class="ink" x1="301.7" y1="73.9" x2="312.1" y2="89.6"/><polygon class="dot-ink" points="315.5,94.6 309.6,91.3 314.6,88"/>
<line class="ink" x1="298.8" y1="31.7" x2="314.1" y2="46.9"/><polygon class="dot-ink" points="318.3,51.2 312,49.1 316.2,44.8"/>
<line class="hi" x1="247.4" y1="117.1" x2="241.4" y2="111.1"/><polygon class="dot" points="232.9,102.6 245.6,106.8 237.2,115.3"/>
<line class="hi" x1="127.1" y1="102.6" x2="121.1" y2="108.6"/><polygon class="dot" points="112.6,117.1 116.8,104.4 125.3,112.8"/>
<line class="hi" x1="112.6" y1="222.9" x2="118.6" y2="228.9"/><polygon class="dot" points="127.1,237.4 114.4,233.2 122.8,224.7"/>
<line class="hi" x1="232.9" y1="237.4" x2="238.9" y2="231.4"/><polygon class="dot" points="247.4,222.9 243.2,235.6 234.7,227.2"/>
<text class="hi" x="244.3" y="254">C</text>
<text class="dim" x="330" y="164" text-anchor="end">x</text>
<text class="dim" x="186" y="32">y</text>
</svg>
<figcaption>The field (grey arrows) circulates clockwise while C (yellow) runs counterclockwise, so the force always points against the motion. That is why the work comes out negative.</figcaption>
</figure>

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